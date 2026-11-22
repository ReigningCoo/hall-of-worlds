use serde_json::json;

use super::member::{MemberSettings, Membership, Read, member_from, read};
use super::tests::variables;
use super::*;

#[test]
fn discords_answers_are_read() {
    let member = read(200, &json!({"roles": ["111", "222"], "user": {}}));
    assert_eq!(member, Read::Done(Membership::Member(vec![111, 222])));
    let not = read(404, &json!({"code": 10007, "message": "Unknown Member"}));
    assert_eq!(not, Read::Done(Membership::NotMember));
    let no_user = read(404, &json!({"code": 10013}));
    assert_eq!(no_user, Read::Done(Membership::NotMember));
    for refused in [
        read(404, &json!({"code": 10004})),
        read(401, &json!({})),
        read(403, &json!({})),
    ] {
        assert!(
            matches!(&refused, Read::Done(Membership::Refused(why)) if why.contains("HOW_DISCORD_BOT_TOKEN")),
            "{refused:?}"
        );
    }
    assert_eq!(read(429, &json!({"retry_after": 1.25})), Read::Wait(1250));
    assert_eq!(read(429, &json!({})), Read::Wait(1000));
    assert!(matches!(
        read(502, &json!(null)),
        Read::Done(Membership::Unknown(_))
    ));
    assert!(matches!(
        read(200, &json!({})),
        Read::Done(Membership::Unknown(_))
    ));
}

#[test]
fn membership_is_required_by_a_server_id_and_needs_the_bot() {
    assert_eq!(member_from(&variables(&[])), Ok(None));
    // A bot without a server is only warned about.
    let bot = [("HOW_DISCORD_BOT_TOKEN", "abc")];
    assert_eq!(member_from(&variables(&bot)), Ok(None));
    let alone = [("HOW_DISCORD_SERVER_ID", "613425648685547541")];
    assert!(member_from(&variables(&alone)).is_err());
    let nonsense = [
        ("HOW_DISCORD_SERVER_ID", "my server"),
        ("HOW_DISCORD_BOT_TOKEN", "abc"),
    ];
    assert!(member_from(&variables(&nonsense)).is_err());
    let both = [
        ("HOW_DISCORD_SERVER_ID", " 613425648685547541 "),
        ("HOW_DISCORD_BOT_TOKEN", "abc"),
    ];
    let settings = member_from(&variables(&both)).unwrap().unwrap();
    assert_eq!(
        settings,
        MemberSettings {
            server: 613425648685547541,
            bot_token: "abc".into(),
        }
    );
    assert!(!format!("{settings:?}").contains("abc"));
}

#[test]
fn membership_needs_discord_logins() {
    let member = [
        ("HOW_DISCORD_SERVER_ID", "613425648685547541"),
        ("HOW_DISCORD_BOT_TOKEN", "abc"),
    ];
    assert!(Setup::from(&variables(&member)).is_err());
    let with_logins = [
        member[0],
        member[1],
        ("HOW_DISCORD_CLIENT_ID", "id"),
        ("HOW_DISCORD_CLIENT_SECRET", "secret"),
    ];
    let setup = Setup::from(&variables(&with_logins)).unwrap();
    assert_eq!(setup.member.unwrap().server, 613425648685547541);
}
