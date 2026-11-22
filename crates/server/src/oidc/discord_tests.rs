use axum::http::StatusCode;
use serde_json::json;

use super::config::providers_from;
use super::discord::{DISCORD_SITE, DiscordSettings, discord_from, discord_user};
use super::tests::{FakeProvider, come_back, variables};
use super::*;

#[test]
fn discord_needs_its_client_id_and_secret_and_may_give_an_invite() {
    let (settings, only, warnings) = discord_from(&variables(&[
        ("HOW_DISCORD_CLIENT_ID", " id "),
        ("HOW_DISCORD_CLIENT_SECRET", "secret"),
        ("HOW_DISCORD_INVITE", "https://discord.gg/abc"),
    ]));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(!only);
    assert_eq!(
        settings,
        Some(DiscordSettings {
            client_id: "id".into(),
            client_secret: "secret".into(),
            invite: Some("https://discord.gg/abc".into()),
            site: DISCORD_SITE.into(),
        })
    );

    // Nothing set: nothing to say.
    let (settings, only, warnings) = discord_from(&variables(&[("PATH", "x")]));
    assert_eq!((settings, only, warnings.len()), (None, false, 0));

    // Half set up, or an invite that isn't https.
    let (settings, _, warnings) = discord_from(&variables(&[("HOW_DISCORD_CLIENT_ID", "id")]));
    assert!(settings.is_none());
    assert!(
        warnings[0].contains("Discord logins are off"),
        "{warnings:?}"
    );
    let (settings, only, warnings) = discord_from(&variables(&[
        ("HOW_DISCORD_CLIENT_ID", "id"),
        ("HOW_DISCORD_CLIENT_SECRET", "secret"),
        ("HOW_DISCORD_INVITE", "http://discord.gg/abc"),
        ("HOW_DISCORD_ONLY", "TRUE"),
    ]));
    assert_eq!(settings.unwrap().invite, None);
    assert!(only);
    assert!(warnings[0].contains("HOW_DISCORD_INVITE"), "{warnings:?}");
}

#[test]
fn an_oidc_provider_cant_take_discords_name() {
    let (providers, warnings) = providers_from(&variables(&[
        ("HOW_OIDC_DISCORD_CLIENT_ID", "id"),
        ("HOW_OIDC_DISCORD_CLIENT_SECRET", "secret"),
        ("HOW_OIDC_DISCORD_ISSUER", "https://discord.com"),
    ]));
    assert!(providers.is_empty());
    assert_eq!(warnings.len(), 1, "{warnings:?}");
}

#[test]
fn discord_only_turns_the_other_providers_off_and_needs_discord() {
    let google = [
        ("HOW_OIDC_GOOGLE_CLIENT_ID", "id"),
        ("HOW_OIDC_GOOGLE_CLIENT_SECRET", "secret"),
        ("HOW_DISCORD_ONLY", "true"),
    ];
    assert!(Setup::from(&variables(&google)).is_err());
    let mut both = google.to_vec();
    both.extend([
        ("HOW_DISCORD_CLIENT_ID", "id"),
        ("HOW_DISCORD_CLIENT_SECRET", "secret"),
    ]);
    let setup = Setup::from(&variables(&both)).unwrap();
    assert!(setup.oidc.is_empty());
    assert!(setup.discord.is_some() && setup.discord_only);
}

#[test]
fn a_discord_user_needs_an_id_and_a_name() {
    let user = discord_user(&json!({"id": "123", "username": "nelly", "email": "x@y.z"}));
    let user = user.unwrap();
    assert_eq!((user.id, user.name.as_str()), (123, "nelly"));
    assert_eq!(format!("{user:?}"), "DiscordUser(hidden)");
    assert!(discord_user(&json!({"username": "nelly"})).is_err());
    assert!(discord_user(&json!({"id": "not a number", "username": "nelly"})).is_err());
    assert!(discord_user(&json!({"id": "123"})).is_err());
}

#[test]
fn a_discord_id_has_one_key_per_secret() {
    let secret = LoginSecret::new("one");
    assert_eq!(secret.discord_key(5), secret.discord_key(5));
    assert_ne!(secret.discord_key(5), secret.discord_key(6));
    assert_ne!(
        LoginSecret::new("two").discord_key(5),
        secret.discord_key(5)
    );
}

#[test]
fn a_discord_login_goes_through_discords_page_and_comes_back_with_the_user() {
    let provider = FakeProvider::start();
    let logins = provider.discord_logins("secret", false);
    let invite = Some("https://discord.example.com/invite".to_owned());
    assert_eq!(logins.methods().len(), 2);
    assert_eq!(logins.methods()[1], LoginMethod::Discord { invite });
    assert!(!logins.discord_only());

    let (state, page) = logins.begin(shared::protocol::DISCORD).unwrap();
    assert!(page.starts_with(&format!("{}/oauth2/authorize?", provider.base)));
    let url = url::Url::parse(&page).unwrap();
    let asked = |name: &str| {
        let found = url.query_pairs().find(|(key, _)| key == name);
        found.map(|(_, value)| value.into_owned())
    };
    assert_eq!(asked("client_id").as_deref(), Some("discord id"));
    assert_eq!(asked("scope").as_deref(), Some("identify"));
    assert_eq!(asked("code_challenge_method").as_deref(), Some("S256"));

    assert_eq!(come_back(&logins, &page, "good"), StatusCode::OK);
    let finished = logins.finished();
    assert_eq!(finished[0].state, state);
    assert_eq!(finished[0].provider, "discord");
    let Ok(Proved::Discord(user)) = &finished[0].proved else {
        panic!("{:?}", finished[0].proved);
    };
    assert_eq!(user.id, 80351110224678912);
    assert_eq!(user.name, "nelly");

    // No id, or a code Discord doesn't know.
    for code in ["noid", "unknown"] {
        let (_, page) = logins.begin("discord").unwrap();
        assert_eq!(come_back(&logins, &page, code), StatusCode::BAD_REQUEST);
        assert!(logins.finished()[0].proved.is_err(), "{code}");
    }
}

#[test]
fn discord_only_offers_only_discord() {
    let provider = FakeProvider::start();
    let logins = provider.discord_logins("secret", true);
    assert!(logins.discord_only());
    let invite = Some("https://discord.example.com/invite".to_owned());
    assert_eq!(logins.methods(), [LoginMethod::Discord { invite }]);
    assert!(logins.begin("fake").is_err());
}
