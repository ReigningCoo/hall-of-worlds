use axum::http::StatusCode;
use shared::protocol::{
    ClientLink, ClientMessage, Credentials, DISCORD, DiscordLink, GuestToken, ServerMessage,
};

use super::super::login_tests::Server;
use super::super::tests::answers;
use crate::accounts::AccountId;
use crate::oidc::tests::{FakeProvider, come_back};

/// The id the fake Discord gives for the code "good".
const NELLY: u64 = 80351110224678912;

/// Logs in through `provider` (the fake OIDC one or the fake Discord), coming
/// back with `code`: what the client is told once the login is finished.
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
    answers(end)
}

/// Asks to link Discord and comes back from it with `code`: what the client
/// is told once that is finished.
fn link(server: &mut Server, end: &ClientLink, code: &str) -> Vec<ServerMessage> {
    let told = server.send(end, ClientMessage::LinkDiscord);
    let [ServerMessage::OpenLogin { url }] = &told[..] else {
        panic!("{told:?}");
    };
    let url = url.clone();
    assert_eq!(
        come_back(server.logins.as_ref().unwrap(), &url, code),
        StatusCode::OK
    );
    server.tick();
    let told = answers(end);
    // The state is used once.
    let logins = server.logins.as_ref().unwrap();
    assert_eq!(come_back(logins, &url, code), StatusCode::BAD_REQUEST);
    told
}

fn account(server: &Server, client: usize) -> AccountId {
    server.connections.as_ref().unwrap().clients[client]
        .account
        .unwrap()
}

fn linked(server: &mut Server, id: &AccountId) -> Option<u64> {
    let accounts = &mut server.connections.as_mut().unwrap().accounts;
    accounts.discord_of(id).unwrap()
}

#[test]
fn an_account_links_discord_and_then_either_login_opens_it() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (end, _) = server.connect();
    let told = through(&mut server, &end, "fake", "good");
    let [
        ServerMessage::LoggedIn { .. },
        ServerMessage::Characters { .. },
        ServerMessage::DiscordLink(DiscordLink::CanLink),
    ] = &told[..]
    else {
        panic!("{told:?}");
    };
    let id = account(&server, 0);

    let told = link(&mut server, &end, "good");
    assert_eq!(told, [ServerMessage::DiscordLink(DiscordLink::Linked)]);
    assert_eq!(linked(&mut server, &id), Some(NELLY));
    // Asking again only says so.
    let told = server.send(&end, ClientMessage::LinkDiscord);
    assert_eq!(told, [ServerMessage::DiscordLink(DiscordLink::Linked)]);

    // Through Discord, to the same account, which says it is linked.
    let (by_discord, _) = server.connect();
    let told = through(&mut server, &by_discord, DISCORD, "good");
    assert!(
        matches!(
            told[..],
            [
                ServerMessage::LoggedIn { .. },
                ServerMessage::Characters { .. },
                ServerMessage::DiscordLink(DiscordLink::Linked),
            ]
        ),
        "{told:?}"
    );
    assert_eq!(account(&server, 1), id);

    // And the first way still leads there.
    let (by_provider, _) = server.connect();
    through(&mut server, &by_provider, "fake", "good");
    assert_eq!(account(&server, 2), id);
}

#[test]
fn a_discord_account_with_its_own_game_account_is_not_linked() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (by_discord, _) = server.connect();
    through(&mut server, &by_discord, DISCORD, "good");
    let its_own = account(&server, 0);

    let (end, _) = server.connect();
    through(&mut server, &end, "fake", "good");
    let id = account(&server, 1);
    let told = link(&mut server, &end, "good");
    assert!(
        matches!(&told[..], [ServerMessage::DiscordLink(DiscordLink::Failed(why))] if why.contains("its own")),
        "{told:?}"
    );
    assert_eq!(linked(&mut server, &id), None);

    // Each login still leads to its own account; another Discord user links.
    let (again, _) = server.connect();
    through(&mut server, &again, DISCORD, "good");
    assert_eq!(account(&server, 2), its_own);
    let told = link(&mut server, &end, "other");
    assert_eq!(told, [ServerMessage::DiscordLink(DiscordLink::Linked)]);
    assert_eq!(linked(&mut server, &id), Some(41771983423143937));
}

#[test]
fn a_link_that_goes_wrong_says_why_and_can_be_tried_again() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (end, _) = server.connect();
    through(&mut server, &end, "fake", "good");
    let told = server.send(&end, ClientMessage::LinkDiscord);
    let [ServerMessage::OpenLogin { url }] = &told[..] else {
        panic!("{told:?}");
    };
    come_back(server.logins.as_ref().unwrap(), url, "noid");
    server.tick();
    let told = answers(&end);
    assert!(
        matches!(
            told[..],
            [ServerMessage::DiscordLink(DiscordLink::Failed(_))]
        ),
        "{told:?}"
    );
    let told = link(&mut server, &end, "good");
    assert_eq!(told, [ServerMessage::DiscordLink(DiscordLink::Linked)]);
}

#[test]
fn guests_and_servers_without_discord_are_not_offered_linking() {
    let provider = FakeProvider::start();
    let mut server = Server::new(Some(provider.discord_logins("secret", false)));
    let (end, _) = server.connect();
    let told = server.send(
        &end,
        ClientMessage::Login(Credentials::Guest(GuestToken([5; 32]))),
    );
    assert!(
        matches!(told[..], [ServerMessage::Characters { .. }]),
        "{told:?}"
    );
    let told = server.send(&end, ClientMessage::LinkDiscord);
    assert!(
        matches!(
            told[..],
            [ServerMessage::DiscordLink(DiscordLink::Failed(_))]
        ),
        "{told:?}"
    );

    let mut server = Server::new(Some(provider.logins("secret")));
    let (end, _) = server.connect();
    let told = through(&mut server, &end, "fake", "good");
    assert!(
        matches!(
            told[..],
            [
                ServerMessage::LoggedIn { .. },
                ServerMessage::Characters { .. }
            ]
        ),
        "{told:?}"
    );
    let told = server.send(&end, ClientMessage::LinkDiscord);
    assert!(
        matches!(
            told[..],
            [ServerMessage::DiscordLink(DiscordLink::Failed(_))]
        ),
        "{told:?}"
    );
}
