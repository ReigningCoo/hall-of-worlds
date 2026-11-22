use shared::protocol::{
    ClientLink, ClientMessage, Credentials, DISCORD, DiscordLink, EmailAddress, GuestToken,
    LoginMethod, ServerMessage,
};

use super::login_tests::Server;
use super::tests::answers;
use crate::accounts::{AccountId, LoginKey};
use crate::oidc::tests::{FakeProvider, come_back};

/// Logs in through the fake Discord, coming back with `code`: what the client
/// is told once the login is finished.
fn through_discord(server: &mut Server, end: &ClientLink, code: &str) -> Vec<ServerMessage> {
    let told = server.send(
        end,
        ClientMessage::Login(Credentials::Provider(DISCORD.into())),
    );
    let [ServerMessage::OpenLogin { url }] = &told[..] else {
        panic!("{told:?}");
    };
    come_back(server.logins.as_ref().unwrap(), url, code);
    server.tick();
    answers(end)
}

fn account(server: &Server, client: usize) -> AccountId {
    server.connections.as_ref().unwrap().clients[client]
        .account
        .unwrap()
}

#[test]
fn a_player_logs_in_through_discord_and_the_account_keeps_their_discord_id() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (end, told) = server.connect();
    let [ServerMessage::LoginOptions { methods }] = &told[..] else {
        panic!("{told:?}");
    };
    let invite = Some("https://discord.example.com/invite".to_owned());
    assert!(methods.contains(&LoginMethod::Discord { invite }));

    let told = through_discord(&mut server, &end, "good");
    let [
        ServerMessage::LoggedIn { session, email },
        ServerMessage::Characters { .. },
        ServerMessage::DiscordLink(DiscordLink::Linked),
    ] = &told[..]
    else {
        panic!("{told:?}");
    };
    // Discord isn't asked for an address: the menus show the Discord name.
    assert_eq!(email, &EmailAddress("nelly on Discord".into()));
    let id = account(&server, 0);
    let accounts = &mut server.connections.as_mut().unwrap().accounts;
    assert_eq!(accounts.discord_of(&id), Ok(Some(80351110224678912)));

    // Next time, with the session, to the same account.
    let (again, _) = server.connect();
    let told = server.send(&again, ClientMessage::Login(Credentials::Session(*session)));
    assert!(
        matches!(told[..], [ServerMessage::Characters { .. }, _]),
        "{told:?}"
    );
    assert_eq!(account(&server, 1), id);
}

#[test]
fn the_same_discord_user_reaches_the_same_account_on_servers_sharing_the_secret() {
    let provider = FakeProvider::start();
    let account_with = |secret: &str| {
        let mut server = Server::new(Some(provider.discord_logins(secret, false)));
        let (end, _) = server.connect();
        through_discord(&mut server, &end, "good");
        account(&server, 0)
    };
    assert_eq!(account_with("shared"), account_with("shared"));
    assert_ne!(account_with("shared"), account_with("another"));
}

#[test]
fn a_discord_login_that_goes_wrong_can_be_tried_again() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (end, _) = server.connect();
    let told = through_discord(&mut server, &end, "noid");
    assert!(
        matches!(told[..], [ServerMessage::LoginFailed(_)]),
        "{told:?}"
    );
    let told = through_discord(&mut server, &end, "good");
    assert!(
        matches!(told[..], [ServerMessage::LoggedIn { .. }, ..]),
        "{told:?}"
    );
}

#[test]
fn discord_only_lets_in_no_guests_and_no_session_without_discord() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", true)));
    // As `main.rs` sets it for HOW_DISCORD_ONLY.
    server.guests = false;
    let (end, told) = server.connect();
    let invite = Some("https://discord.example.com/invite".to_owned());
    let methods = vec![LoginMethod::Discord { invite }];
    assert_eq!(told, [ServerMessage::LoginOptions { methods }]);
    let told = server.send(
        &end,
        ClientMessage::Login(Credentials::Guest(GuestToken([3; 32]))),
    );
    assert!(matches!(told[..], [ServerMessage::Refused(_)]), "{told:?}");

    // A session for an account made another way (before Discord-only).
    let accounts = &mut server.connections.as_mut().unwrap().accounts;
    let other = accounts.log_in(&LoginKey::Test("other")).unwrap();
    let session = accounts.new_session(other).unwrap();
    let (end, _) = server.connect();
    let told = server.send(&end, ClientMessage::Login(Credentials::Session(session)));
    assert!(
        matches!(told[..], [ServerMessage::LoginFailed(_)]),
        "{told:?}"
    );

    // Discord's own get in.
    let told = through_discord(&mut server, &end, "good");
    assert!(
        matches!(told[..], [ServerMessage::LoggedIn { .. }, ..]),
        "{told:?}"
    );
}
