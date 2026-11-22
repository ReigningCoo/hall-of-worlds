//! Membership of the server's Discord server (`HOW_DISCORD_SERVER_ID`): a
//! login to an account following Discord (`Roles::follows_discord`) waits for
//! the bot's answer before it is let in (`let_in`), and everyone in is asked
//! about again every `CHECK_EVERY` seconds (`check_discord`). Each answer sets
//! the account's role from its Discord roles; one no longer on the Discord
//! server is sent away, unless a role given by hand lets it in. An account
//! with no Discord linked must link it before playing. When Discord can't be
//! reached, players are let in as they are and asked about again later; when
//! it refuses the bot, nobody new is let in. Logged by account id only.

use bevy::prelude::*;
use shared::protocol::{DiscordLink, ServerMessage};

use super::super::{Client, Connections};
use super::{logged_in, tell_discord};
use crate::accounts::{AccountId, Accounts};
use crate::oidc::{Logins, Membership};
use crate::storage::now;
use crate::{ServerOptions, ServerWorld, ServerWorlds};

/// How often (seconds) everyone in is asked about again.
pub(crate) const CHECK_EVERY: u64 = 5 * 60;

/// What a player not on the Discord server is told.
pub(crate) const NOT_A_MEMBER: &str =
    "this server is for the members of its Discord server: join it, then log in again";

/// What a player is told while Discord refuses the bot.
pub(crate) const CANT_CHECK: &str = "this server can't check its Discord just now: try again later";

/// What a client trying to play before linking Discord is told.
pub(crate) const LINK_FIRST: &str = "link Discord first: this server is for its Discord's members";

/// A client and the server's Discord.
#[derive(Default)]
pub(crate) struct DiscordGate {
    /// The Discord user its account belongs to, checked every `CHECK_EVERY`.
    pub(super) user: Option<u64>,
    /// A login waiting for the bot's answer: the account, and how (for the log).
    pub(super) waiting: Option<(AccountId, String)>,
    /// When it was last asked about (seconds since 1970).
    pub(super) asked: u64,
    /// The account has no Discord linked and must link it before playing.
    pub(crate) must_link: bool,
}

/// Lets a client into the account it logged in to (`logged_in`, then
/// `tell_discord`), or, while Discord membership is required, asks the bot
/// first. `false` if the client should go.
pub(super) fn let_in(
    client: &mut Client,
    id: AccountId,
    how: String,
    accounts: &mut Accounts,
    world: &ServerWorld,
    options: &ServerOptions,
    logins: &Logins,
) -> bool {
    client.discord = DiscordGate::default();
    if logins.requires_member() {
        let (guest, user) = match accounts.account(&id) {
            Ok(account) => (account.guest, account.discord),
            Err(error) => return client.refuse_for_saves(&error),
        };
        let follows = !guest && accounts.roles.follows_discord(id);
        client.discord.user = user.filter(|_| !guest);
        match user {
            Some(user) if follows => {
                client.discord.waiting = Some((id, how));
                client.discord.asked = now();
                logins.ask_member(user);
                return true;
            }
            None if follows && !client.makes_characters => {
                client.link.send(&ServerMessage::Refused(LINK_FIRST.into()));
                return false;
            }
            // Told before its characters, so a client doesn't choose one at once.
            None if follows => {
                client.discord.must_link = true;
                let told = ServerMessage::DiscordLink(DiscordLink::MustLink);
                return client.link.send(&told)
                    && logged_in(client, id, &how, accounts, world, options);
            }
            _ => {}
        }
    }
    logged_in(client, id, &how, accounts, world, options) && tell_discord(client, accounts, logins)
}

/// Starts checking a client's newly linked Discord user, as for one logged in.
pub(super) fn check_linked(client: &mut Client, user: u64, logins: &Logins) {
    if logins.requires_member() {
        client.discord.must_link = false;
        client.discord.user = Some(user);
        client.discord.asked = now();
        logins.ask_member(user);
    }
}

/// Takes the bot's answers, and asks again about everyone due.
pub(crate) fn check_discord(
    mut connections: ResMut<Connections>,
    mut worlds: ResMut<ServerWorlds>,
    options: Res<ServerOptions>,
    logins: Option<Res<Logins>>,
) {
    let Some(logins) = logins.filter(|logins| logins.requires_member()) else {
        return;
    };
    let Connections { clients, accounts } = &mut *connections;
    let now = now();
    for (user, found) in logins.member_answers() {
        let asked_about = clients
            .iter_mut()
            .filter(|client| !client.gone && client.discord.user == Some(user));
        for client in asked_about {
            let world = worlds.of(client.world);
            if !take_answer(client, &found, accounts, world, &options, &logins, now) {
                client.disconnect(world);
            }
        }
    }
    for client in clients.iter_mut().filter(|client| !client.gone) {
        let gate = &mut client.discord;
        if let Some(user) = gate.user
            && gate.waiting.is_none()
            && now >= gate.asked + CHECK_EVERY
        {
            gate.asked = now;
            logins.ask_member(user);
        }
    }
}

/// Acts on what the bot found of a client's Discord user. `false` if it should go.
fn take_answer(
    client: &mut Client,
    found: &Membership,
    accounts: &mut Accounts,
    world: &ServerWorld,
    options: &ServerOptions,
    logins: &Logins,
    now: u64,
) -> bool {
    let waiting = client.discord.waiting.take();
    let Some(id) = waiting.as_ref().map(|(id, _)| *id).or(client.account) else {
        return true;
    };
    let roles = match found {
        Membership::Member(roles) => Some(roles.as_slice()),
        Membership::NotMember => None,
        Membership::Unknown(reason) => {
            warn!("Account {} couldn't be checked on Discord: {reason}", id.0);
            return match waiting {
                Some((id, how)) => in_now(client, id, &how, accounts, world, options, logins),
                None => true,
            };
        }
        Membership::Refused(reason) => {
            error!("{reason}");
            if waiting.is_none() {
                return true;
            }
            info!("Account {} was turned away: Discord can't be checked", id.0);
            client.link.send(&ServerMessage::Refused(CANT_CHECK.into()));
            return false;
        }
    };
    if let Err(problem) = accounts.roles.follow_discord(id, roles, now) {
        error!("{problem}");
    }
    if roles.is_none() && !accounts.roles.in_without_discord(id) {
        info!("Account {} isn't on the Discord server: sent away", id.0);
        client
            .link
            .send(&ServerMessage::Refused(NOT_A_MEMBER.into()));
        return false;
    }
    match waiting {
        Some((id, how)) => in_now(client, id, &how, accounts, world, options, logins),
        None => true,
    }
}

/// Lets in a login that was waiting for the bot.
fn in_now(
    client: &mut Client,
    id: AccountId,
    how: &str,
    accounts: &mut Accounts,
    world: &ServerWorld,
    options: &ServerOptions,
    logins: &Logins,
) -> bool {
    logged_in(client, id, how, accounts, world, options) && tell_discord(client, accounts, logins)
}
