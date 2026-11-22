//! Link Discord: an account logged in to another way (an OIDC provider) adds
//! Discord as a way in, so either login opens it. The player logs in on
//! Discord's page with a state made for their account (`Logins::begin_link`),
//! and on coming back the Discord login is made to lead to that account. It
//! is refused if that Discord user already has an account of their own here
//! (merging accounts isn't done). Logged by account id only.

use bevy::prelude::*;
use shared::protocol::{DiscordLink, ServerMessage};

use super::super::Client;
use crate::accounts::{AccountId, Accounts, LoginKey};
use crate::oidc::{Logins, Proved};

/// Tells a client just logged in whether its account has Discord linked, or
/// can link it: only while the server logs in with Discord, never to a guest.
/// `false` if the client has gone.
pub(super) fn tell_discord(client: &Client, accounts: &mut Accounts, logins: &Logins) -> bool {
    let Some(id) = client.account.filter(|_| logins.has_discord()) else {
        return true;
    };
    let told = match accounts.account(&id) {
        Ok(account) if account.guest => return true,
        Ok(account) if account.discord.is_some() => DiscordLink::Linked,
        Ok(_) => DiscordLink::CanLink,
        Err(error) => return client.refuse_for_saves(&error),
    };
    client.link.send(&ServerMessage::DiscordLink(told))
}

/// Starts linking Discord, as the client asked (`ClientMessage::LinkDiscord`):
/// the page to open, or why not. `false` if the client has gone.
pub(in super::super) fn link_discord(
    client: &mut Client,
    accounts: &mut Accounts,
    logins: Option<&Logins>,
) -> bool {
    let failed = |client: &Client, reason: &str| {
        let told = DiscordLink::Failed(reason.into());
        client.link.send(&ServerMessage::DiscordLink(told))
    };
    let Some(logins) = logins.filter(|logins| logins.has_discord()) else {
        return failed(client, "this server doesn't log in with Discord");
    };
    let Some(id) = client.account else {
        return failed(client, "log in first");
    };
    match accounts.account(&id) {
        Ok(account) if account.guest => return failed(client, "a guest can't link Discord"),
        Ok(account) if account.discord.is_some() => {
            return client
                .link
                .send(&ServerMessage::DiscordLink(DiscordLink::Linked));
        }
        Ok(_) => {}
        Err(error) => return client.refuse_for_saves(&error),
    }
    match logins.begin_link(id) {
        Ok((state, url)) => {
            if let Some(old) = client.login_state.replace(state) {
                logins.forget(&old);
            }
            client.link.send(&ServerMessage::OpenLogin { url })
        }
        Err(reason) => failed(client, &reason),
    }
}

/// Finishes linking Discord to `id`, the account of the client that began it,
/// once the player is back from Discord's page. `false` if the client should go.
pub(super) fn finish_link(
    client: &Client,
    id: AccountId,
    proved: Result<Proved, String>,
    accounts: &mut Accounts,
    logins: &Logins,
) -> bool {
    let told = match proved {
        Ok(Proved::Discord(user)) => {
            let key = LoginKey::Discord(logins.discord_key(user.id));
            match link(id, user.id, &key, accounts) {
                Ok(told) => told,
                Err(error) => return client.refuse_for_saves(&error),
            }
        }
        Ok(Proved::Email(_)) => DiscordLink::Failed("that wasn't Discord".into()),
        Err(reason) => {
            info!("Account {} couldn't link Discord: {reason}", id.0);
            DiscordLink::Failed(reason)
        }
    };
    client.link.send(&ServerMessage::DiscordLink(told))
}

/// Links this Discord user to the account, unless either has another already.
/// An error if saves can't be reached.
fn link(
    id: AccountId,
    user: u64,
    key: &LoginKey,
    accounts: &mut Accounts,
) -> Result<DiscordLink, String> {
    let other_user = accounts.discord_of(&id)?.is_some_and(|had| had != user);
    let leads_to = accounts.login_of(key)?;
    if other_user || leads_to.is_some_and(|other| other != id) {
        info!("Account {} couldn't link Discord: taken", id.0);
        let reason = match other_user {
            true => "this account has another Discord account linked already",
            false => {
                "that Discord account already has its own account on this server, \
                 so it can't be linked to this one"
            }
        };
        return Ok(DiscordLink::Failed(reason.into()));
    }
    if leads_to.is_none() {
        accounts.add_login(key, id);
    }
    accounts.note_discord(&id, user)?;
    info!("Account {} linked Discord", id.0);
    Ok(DiscordLink::Linked)
}
