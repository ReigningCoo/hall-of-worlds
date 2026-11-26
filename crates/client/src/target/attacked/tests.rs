use std::collections::HashMap;

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::mob::EntityLook;
use shared::protocol::{Landed, ServerMessage};
use shared::sheet::{Appearance, Feeling};
use shared::vitals::Points;

use super::hostile;

fn seen(number: u128, feeling: Feeling) -> ServerMessage {
    ServerMessage::EntitySeen(EntityLook {
        id: Id::for_test(number),
        kind: CreatureKind::Wolf,
        gender: shared::gender::Gender::Male,
        name: None,
        appearance: Appearance::default(),
        feeling,
        x: 0,
        y: 0,
        worn: Vec::new(),
        mark: None,
        heading: shared::facing::Heading::WEST,
        boss: None,
        stage: shared::livestock::Stage::Adult,
        light: 0,
    })
}

fn hit(who: Option<u128>, target: Option<u128>) -> ServerMessage {
    ServerMessage::AbilityLanded {
        who: who.map(Id::for_test),
        target: target.map(Id::for_test),
        landed: Landed::Hit {
            harm: Points(1_000),
            left: 500,
        },
    }
}

#[test]
fn a_blow_on_the_player_is_hostile() {
    assert!(hostile(&hit(Some(1), None), &mut HashMap::new()));
}

#[test]
fn hitting_an_enemy_is_hostile_and_a_friend_not() {
    let mut feelings = HashMap::new();
    hostile(&seen(1, Feeling::Unfriendly), &mut feelings);
    hostile(&seen(2, Feeling::Friendly), &mut feelings);
    assert!(hostile(&hit(None, Some(1)), &mut feelings));
    assert!(!hostile(&hit(None, Some(2)), &mut feelings));
    // Others fighting among themselves.
    assert!(!hostile(&hit(Some(2), Some(1)), &mut feelings));
}

#[test]
fn turning_unfriendly_is_hostile_but_being_seen_so_is_not() {
    let mut feelings = HashMap::new();
    assert!(!hostile(&seen(1, Feeling::Unfriendly), &mut feelings));
    assert!(!hostile(&seen(1, Feeling::Unfriendly), &mut feelings));
    assert!(!hostile(&seen(2, Feeling::Neutral), &mut feelings));
    assert!(hostile(&seen(2, Feeling::Unfriendly), &mut feelings));
}

#[test]
fn an_enemy_beginning_an_attack_at_the_player_is_hostile() {
    let mut feelings = HashMap::new();
    hostile(&seen(1, Feeling::Unfriendly), &mut feelings);
    hostile(&seen(2, Feeling::Friendly), &mut feelings);
    let started = |who| ServerMessage::AbilityStarted {
        who: Some(Id::for_test(who)),
        ability: shared::ability::Ability::Attack,
        target: None,
        lands_in: 10,
        ready_in: 20,
    };
    assert!(hostile(&started(1), &mut feelings));
    assert!(!hostile(&started(2), &mut feelings));
    hostile(
        &ServerMessage::EntityGone {
            id: Id::for_test(1),
            died: true,
        },
        &mut feelings,
    );
    assert!(!hostile(&started(1), &mut feelings));
}
