use shared::protocol::DiscordLink;

use super::shown;

#[test]
fn link_discord_is_offered_until_linked_and_says_why_it_failed() {
    assert_eq!(shown(None, false), (false, None, None));
    assert_eq!(
        shown(Some(&DiscordLink::CanLink), false),
        (true, None, None)
    );
    let (offered, said, _) = shown(Some(&DiscordLink::CanLink), true);
    assert!(!offered && said.unwrap().contains("browser"));
    let linked = shown(Some(&DiscordLink::Linked), false);
    assert_eq!(linked, (false, Some("Discord linked".into()), None));
    let failed = shown(Some(&DiscordLink::Failed("it has its own".into())), false);
    assert!(failed.0);
    assert_eq!(
        failed.2.as_deref(),
        Some("Linking Discord didn't work: it has its own.")
    );
}
