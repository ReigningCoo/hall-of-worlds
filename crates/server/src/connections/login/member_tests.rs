use std::time::{Duration, Instant};

use shared::protocol::{
    CharacterId, ClientLink, ClientMessage, Credentials, DISCORD, DiscordLink, GuestToken,
    ServerMessage, SessionToken,
};
use shared::role::{BANNED, DEVELOPER, MODERATOR, PLAYER, RoleId};

use super::super::login_tests::Server;
use super::super::tests::answers;
use super::member::{CANT_CHECK, LINK_FIRST, NOT_A_MEMBER};
use crate::accounts::{AccountId, LoginKey};
use crate::oidc::tests::{FAKE_SERVER, FakeProvider, come_back};
use crate::roles::{Actor, Change, Roles};
use crate::storage::now;

/// Discord ids, as the fake Discord gives them: Nelly on its Discord server
/// with roles 111 and 222, Kit on it with none, Rob not on it.
const NELLY: u64 = 80351110224678912;
const KIT: u64 = 55555555555555555;
const ROB: u64 = 41771983423143937;

/// A server requiring its players on the fake's Discord server, where its
/// Discord role 222 makes them moderators.
fn member_server(provider: &FakeProvider) -> Server {
    let mut server = Server::new(Some(provider.member_logins("secret")));
    server.guests = false;
    roles(&mut server).map_discord(222, MODERATOR);
    server
}

fn roles(server: &mut Server) -> &mut Roles {
    &mut server.connections.as_mut().unwrap().accounts.roles
}

/// Ticks until the client is told something (the bot's answer taken), or
/// panics after a few seconds.
fn settle(server: &mut Server, end: &ClientLink) -> Vec<ServerMessage> {
    let started = Instant::now();
    loop {
        server.tick();
        let told = answers(end);
        if !told.is_empty() {
            return told;
        }
        assert!(started.elapsed() < Duration::from_secs(10), "no answer");
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Logs in through `provider` coming back with `code`, waiting for the bot
/// if it is asked: everything the client was told.
fn through(
    server: &mut Server,
    end: &ClientLink,
    provider: &str,
    code: &str,
) -> Vec<ServerMessage> {
    let told = server.send(
        end,
        ClientMessage::Login(Credentials::Provider(provider.into())),
    );
    let [ServerMessage::OpenLogin { url }] = &told[..] else {
        panic!("{told:?}");
    };
    come_back(server.logins.as_ref().unwrap(), url, code);
    server.tick();
    let mut told = answers(end);
    if told.len() == 1 {
        told.extend(settle(server, end));
    }
    told
}

/// The account a Discord user's login leads to.
fn account_of(server: &mut Server, user: u64) -> AccountId {
    let key = LoginKey::Discord(server.logins.as_ref().unwrap().discord_key(user));
    let accounts = &mut server.connections.as_mut().unwrap().accounts;
    accounts.log_in(&key).unwrap()
}

fn role_of(server: &mut Server, id: AccountId) -> RoleId {
    roles(server).held(id, false, now()).badge.id
}

fn give(server: &mut Server, id: AccountId, role: &str, by: Actor) {
    let role = RoleId::new(role);
    let change = Change {
        to: &role,
        until: None,
        reason: "",
    };
    roles(server)
        .set_role(by, id, false, change, now())
        .unwrap();
}

fn session(told: &[ServerMessage]) -> SessionToken {
    let Some(ServerMessage::LoggedIn { session, .. }) = told.first() else {
        panic!("{told:?}");
    };
    *session
}

fn in_with(told: &[ServerMessage]) -> bool {
    matches!(
        told,
        [
            ServerMessage::LoggedIn { .. },
            ServerMessage::Characters { .. },
            ServerMessage::DiscordLink(DiscordLink::Linked),
        ]
    )
}

#[test]
fn a_member_is_let_in_with_the_role_its_discord_roles_give() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, DISCORD, "good");
    // Told of its new role again, as anyone on an account whose role changed.
    assert!(in_with(&told[..3]), "{told:?}");
    assert!(matches!(told[3..], [ServerMessage::Role(_)]), "{told:?}");
    let id = account_of(&mut server, NELLY);
    assert!(role_of(&mut server, id).is(MODERATOR));
    let lines = roles(&mut server).audit_lines().join("\n");
    assert!(
        lines.contains(&format!(
            "Discord gave {} moderator (was player): Discord",
            id.0
        )),
        "{lines}"
    );
    assert!(!lines.contains(&NELLY.to_string()), "{lines}");
}

#[test]
fn someone_not_on_the_discord_server_is_turned_away_unless_given_a_role_by_hand() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, DISCORD, "other");
    assert!(
        matches!(&told[..], [ServerMessage::LoggedIn { .. }, ServerMessage::Refused(why)] if why == NOT_A_MEMBER),
        "{told:?}"
    );
    assert!(server.connections.as_ref().unwrap().clients.is_empty());

    // Given a role by hand, Rob is let in, and Discord doesn't change it.
    let id = account_of(&mut server, ROB);
    give(&mut server, id, DEVELOPER, Actor::Server);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, DISCORD, "other");
    assert!(in_with(&told), "{told:?}");
    assert!(role_of(&mut server, id).is(DEVELOPER));

    // But banned by hand is banned.
    give(&mut server, id, BANNED, Actor::Server);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, DISCORD, "other");
    assert!(
        matches!(&told[..], [ServerMessage::LoggedIn { .. }, ServerMessage::Refused(why)] if why.contains("banned")),
        "{told:?}"
    );
}

#[test]
fn an_account_without_discord_must_link_it_before_playing() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, "fake", "good");
    assert!(
        matches!(
            told[..],
            [
                ServerMessage::LoggedIn { .. },
                ServerMessage::DiscordLink(DiscordLink::MustLink),
                ServerMessage::Characters { .. },
            ]
        ),
        "{told:?}"
    );
    // Choosing or making a character is refused until then.
    let chosen = ClientMessage::ChooseCharacter {
        id: CharacterId(shared::id::Id::for_test(1)),
    };
    let told = server.send(&end, chosen);
    assert_eq!(told, [ServerMessage::Refused(LINK_FIRST.into())]);

    // Linking a member lets the account play, with its Discord role.
    let (end, _) = server.connect();
    through(&mut server, &end, "fake", "good");
    let told = server.send(&end, ClientMessage::LinkDiscord);
    let [ServerMessage::OpenLogin { url }] = &told[..] else {
        panic!("{told:?}");
    };
    come_back(server.logins.as_ref().unwrap(), url, "good");
    server.tick();
    let told = answers(&end);
    assert_eq!(told, [ServerMessage::DiscordLink(DiscordLink::Linked)]);
    let told = settle(&mut server, &end);
    assert!(matches!(told[..], [ServerMessage::Role(_)]), "{told:?}");
    let client = &server.connections.as_ref().unwrap().clients[0];
    let id = client.account.unwrap();
    assert!(!client.discord.must_link);
    assert!(role_of(&mut server, id).is(MODERATOR));
}

#[test]
fn players_are_checked_again_and_their_discord_roles_followed() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    assert!(in_with(&through(&mut server, &end, DISCORD, "kit")));
    let kit = account_of(&mut server, KIT);
    assert!(role_of(&mut server, kit).is(PLAYER));

    // A role from Discord is Discord's to change at the next check.
    give(&mut server, kit, DEVELOPER, Actor::Discord);
    server.connections.as_mut().unwrap().clients[0]
        .discord
        .asked = 0;
    let told = settle(&mut server, &end);
    assert!(matches!(told[..], [ServerMessage::Role(_)]), "{told:?}");
    let told = settle(&mut server, &end);
    assert!(matches!(told[..], [ServerMessage::Role(_)]), "{told:?}");
    assert!(role_of(&mut server, kit).is(PLAYER));
}

#[test]
fn discord_out_of_reach_lets_players_in_but_a_refused_bot_lets_nobody_new_in() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    let told = through(&mut server, &end, DISCORD, "good");
    let session = session(&told);

    // Discord can't be reached: in, as they were.
    let nowhere = "http://127.0.0.1:9";
    let logins = provider.member_logins_with("secret", nowhere, FAKE_SERVER, "bot-token");
    server.logins = Some(logins);
    let (end, _) = server.connect();
    end.send(&ClientMessage::Login(Credentials::Session(session)));
    let told = settle(&mut server, &end);
    assert!(
        matches!(
            told[..],
            [
                ServerMessage::Characters { .. },
                ServerMessage::DiscordLink(DiscordLink::Linked)
            ]
        ),
        "{told:?}"
    );

    // The bot refused (a wrong token, or another Discord server): turned away.
    for (server_id, token) in [(FAKE_SERVER, "wrong"), (1, "bot-token")] {
        let site = provider.base.clone();
        let logins = provider.member_logins_with("secret", &site, server_id, token);
        server.logins = Some(logins);
        let (end, _) = server.connect();
        end.send(&ClientMessage::Login(Credentials::Session(session)));
        let told = settle(&mut server, &end);
        assert_eq!(told, [ServerMessage::Refused(CANT_CHECK.into())]);
    }
}

#[test]
fn guests_are_not_taken_while_membership_is_required() {
    let provider = FakeProvider::start();
    let mut server = member_server(&provider);
    let (end, _) = server.connect();
    let guest = Credentials::Guest(GuestToken([5; 32]));
    let told = server.send(&end, ClientMessage::Login(guest));
    assert!(matches!(told[..], [ServerMessage::Refused(_)]), "{told:?}");
}
