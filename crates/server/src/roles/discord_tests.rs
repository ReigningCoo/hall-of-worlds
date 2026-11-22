use shared::id::Id;
use shared::role::*;
use uuid::Uuid;

use super::file::{DiscordRole, HeldRole, RolesFile};
use super::set_role::{Actor, Change};
use super::*;

fn account(n: u128) -> AccountId {
    AccountId(Id(Uuid::from_u128(n)))
}

fn mapped(roles: &[(u64, &str)]) -> Roles {
    let mut mapped = Roles::default();
    for (discord, role) in roles {
        mapped.map_discord(*discord, role);
    }
    mapped
}

fn role(roles: &mut Roles, id: AccountId) -> RoleId {
    roles.held(id, false, 0).badge.id
}

fn by_hand(roles: &mut Roles, id: AccountId, role: &str) {
    let role = RoleId::new(role);
    let change = Change {
        to: &role,
        until: None,
        reason: "",
    };
    roles
        .set_role(Actor::Server, id, false, change, 10)
        .unwrap();
}

#[test]
fn the_first_discord_role_that_matches_gives_the_role() {
    let mut roles = mapped(&[(1, ADMIN), (2, MODERATOR), (3, DEVELOPER)]);
    roles.follow_discord(account(1), Some(&[3, 2]), 10).unwrap();
    assert!(role(&mut roles, account(1)).is(MODERATOR));
    // Nothing matching: back to Player.
    roles.follow_discord(account(1), Some(&[9]), 10).unwrap();
    assert!(role(&mut roles, account(1)).is(PLAYER));
    roles.follow_discord(account(1), Some(&[1]), 10).unwrap();
    assert!(role(&mut roles, account(1)).is(ADMIN));
    // Off the Discord server: none.
    roles.follow_discord(account(1), None, 10).unwrap();
    assert!(role(&mut roles, account(1)).is(PLAYER));
    let lines = roles.audit_lines();
    assert_eq!(lines.len(), 4, "{lines:?}");
    assert!(lines.iter().all(|line| line.contains("Discord gave")));
    // Nothing changes, nothing is written.
    roles.follow_discord(account(1), None, 10).unwrap();
    assert_eq!(roles.audit_lines().len(), 4);
}

#[test]
fn a_role_given_by_hand_is_never_changed_by_discord_and_lets_the_account_in() {
    let mut roles = mapped(&[(2, MODERATOR)]);
    by_hand(&mut roles, account(1), DEVELOPER);
    assert!(!roles.follows_discord(account(1)));
    assert!(roles.in_without_discord(account(1)));
    roles.follow_discord(account(1), Some(&[2]), 10).unwrap();
    roles.follow_discord(account(1), None, 10).unwrap();
    assert!(role(&mut roles, account(1)).is(DEVELOPER));

    // One from Discord is Discord's, and doesn't let anyone in by itself.
    roles.follow_discord(account(2), Some(&[2]), 10).unwrap();
    assert!(roles.follows_discord(account(2)));
    assert!(!roles.in_without_discord(account(2)));

    // Banned by hand keeps them out.
    by_hand(&mut roles, account(3), BANNED);
    assert!(!roles.follows_discord(account(3)));
    assert!(!roles.in_without_discord(account(3)));
}

#[test]
fn the_mapping_never_gives_the_owner_a_guest_or_an_unknown_role() {
    for bad in [OWNER, GUEST, "nobody"] {
        let mut file = RolesFile::new();
        file.discord_roles.push(DiscordRole {
            discord: 5,
            role: RoleId::new(bad),
        });
        assert!(file.check().is_err(), "{bad}");
    }
    // Nor may an account hold the Owner's role from Discord.
    let mut file = RolesFile::new();
    let held = HeldRole {
        role: RoleId::new(OWNER),
        since: 0,
        until: None,
        reason: String::new(),
        by: None,
        discord: true,
    };
    file.accounts.insert(account(1).0.to_string(), held);
    assert!(file.check().is_err());
}

#[test]
fn the_mapping_and_discords_roles_are_kept_in_roles_ron() {
    let id = 613425648685547541;
    let mut roles = mapped(&[(id, MODERATOR)]);
    roles.follow_discord(account(1), Some(&[id]), 10).unwrap();
    let written = ron::to_string(&roles.file).unwrap();
    let mapping = format!("discord_roles:[(discord:{id},role:\"moderator\")],");
    assert!(written.contains(&mapping), "{written}");
    let read: RolesFile = ron::from_str(&written).unwrap();
    assert_eq!(read, roles.file);

    // An older file, with neither, still reads: its roles given by hand.
    let older = written.replace(",discord:true", "").replace(&mapping, "");
    let read: RolesFile = ron::from_str(&older).unwrap();
    assert!(read.discord_roles.is_empty());
    assert!(read.accounts.values().all(|held| !held.discord));
}
