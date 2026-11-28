//! Alertness (`alert.rs`): Aware of who it sees, Alert after, calming down,
//! players told, and the critical on a mob caught off guard.

use shared::alertness::{ALERT_TICKS, Alertness};
use shared::combat::{Fighter, Guard, Wielded};
use shared::creature::CreatureKind;
use shared::facing::Heading;
use shared::item::Inventory;
use shared::protocol::ServerMessage;
use shared::sheet::Attributes;

use super::fight::{Striker, hit_mob};
use super::tests::{beside_a_mob, run};
use crate::connections::tests::TestClient;

fn mob(test: &mut TestClient) -> &mut super::Mob {
    &mut test.world.populations.0.values_mut().next().unwrap().mobs[0]
}

/// The `MobAlert`s sent since last asked.
fn told(test: &mut TestClient) -> Vec<bool> {
    let mut told = Vec::new();
    while let Some(message) = test.end.receive().unwrap() {
        if let ServerMessage::MobAlert(alerted) = message {
            told.push(alerted);
        }
    }
    told
}

#[test]
fn a_mob_seeing_the_player_is_aware_then_alert_then_calms_down() {
    // Dwarves take no notice, so stand where they are.
    let mut test = beside_a_mob(CreatureKind::Dwarf, 3);
    told(&mut test);
    run(&mut test, 1);
    let me = test.client.entity_id();
    assert_eq!(mob(&mut test).wariness.stage(), Alertness::Aware);
    assert_eq!(mob(&mut test).wariness.about(), Some(me));
    assert_eq!(told(&mut test), vec![true]);

    // Looking away, it is only Alert, and still wary of them.
    mob(&mut test).heading = Heading::EAST;
    run(&mut test, 1);
    assert_eq!(mob(&mut test).wariness.stage(), Alertness::Alert);
    assert!(told(&mut test).is_empty());
    // Seeing no one for a while, it calms down.
    run(&mut test, ALERT_TICKS);
    assert!(mob(&mut test).wariness.is_unaware());
    assert_eq!(told(&mut test), vec![false]);
}

#[test]
fn a_mob_with_its_back_turned_stays_unaware() {
    let mut test = beside_a_mob(CreatureKind::Dwarf, 3);
    told(&mut test);
    mob(&mut test).heading = Heading::EAST;
    run(&mut test, 5);
    assert!(mob(&mut test).off_guard());
    assert!(told(&mut test).is_empty());
}

#[test]
fn a_blow_on_a_mob_caught_off_guard_does_double_harm() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 1);
    let place = *test.world.populations.0.keys().next().unwrap();
    let id = mob(&mut test).id;
    let me = test.client.entity_id();
    let position = test.client.visit.as_ref().unwrap().position();
    let attributes = Attributes::all(100);
    let fighter = || {
        Fighter::mob(
            CreatureKind::Human,
            &attributes,
            Wielded::of(&Inventory::default()),
            Guard::default(),
        )
    };
    let blow = |test: &mut TestClient, tick| {
        mob(test).vitals = Default::default();
        let by = Striker {
            id: me,
            position,
            fighter: fighter(),
            swing: fighter().swing(0),
            kicked: None,
        };
        hit_mob(&mut test.world, (place, place.1), id, by, tick)
            .unwrap()
            .harm
    };
    let mut landed = 0;
    for tick in 1..50 {
        // Aware of them: an ordinary blow.
        mob(&mut test).noticed(me, tick);
        let ordinary = blow(&mut test, tick);
        // Caught off guard: the same blow, doubled.
        mob(&mut test).wariness = Default::default();
        let sneak = blow(&mut test, tick);
        assert_eq!(sneak.0, ordinary.0 * 2);
        landed += usize::from(ordinary.0 > 0);
        // Once struck it knows they are there.
        assert!(!mob(&mut test).off_guard());
    }
    assert!(landed > 0);
}
