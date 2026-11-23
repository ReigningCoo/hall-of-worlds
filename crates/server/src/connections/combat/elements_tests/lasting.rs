//! Air, Earth, Poison, Curse, Illusion and Warding (magic plan PR 12): Air
//! knocks a mob away, Poison harms it over time, Stoneskin and Warding lie on
//! the caster and change their blows and resistance, reversed Poison cures,
//! reversed Warding dispels, reversed Curse lifts a worn curse.

use shared::bonus::Source;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::Favour;
use shared::poi::PoiKind;
use shared::protocol::ClientMessage;
use shared::spell::{Effect, ILLUSION, STONESKIN, Spell, Step, WARDED, Where};

use super::super::tests::mob_health;
use super::{cast, magician, on_self, unlock_touch};
use crate::connections::tests::TestClient;

/// A player in a castle with room to the east and a dwarf in the next cell
/// that way, Touch unlocked.
fn mob_east() -> (TestClient, Id) {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevEnterSite {
        kind: PoiKind::Castle,
        seed: 7,
        level: 0,
        ruin: 0,
    }]);
    let (x, y) = test.client.visit.as_ref().unwrap().room_to_walk_east(4);
    test.send(&[ClientMessage::DevTeleport { x, y }]);
    let population = test.world.populations.0.values_mut().next().unwrap();
    let id = population
        .remake_for_test(0, CreatureKind::Dwarf, (x + 1, y))
        .id;
    unlock_touch(&mut test);
    (test, id)
}

fn mob_x(test: &TestClient, id: Id) -> i64 {
    (test.world.populations.0.values())
        .flat_map(|population| &population.mobs)
        .find(|mob| mob.id == id)
        .map(|mob| mob.spot.x)
        .unwrap()
}

fn touch(effects: &[(Effect, u8)]) -> Spell {
    Spell::new(vec![Step::at(Where::Touch, 1), Step::effects(effects)])
}

fn under(test: &TestClient, part: shared::spell::Part) -> bool {
    let tick = test.world.action_clock.ticks();
    test.client.sheet.under_spell(part, tick)
}

#[test]
fn air_knocks_the_mob_away() {
    let (mut test, mob) = mob_east();
    let before = mob_x(&test, mob);
    let _ = cast(&mut test, &touch(&[(Effect::Air, 2)]), Some(mob));
    assert!(
        mob_x(&test, mob) > before,
        "knocked east, away from the caster"
    );
}

#[test]
fn poison_harms_the_mob_over_time() {
    let (mut test, mob) = mob_east();
    let full = mob_health(&test, mob).unwrap();
    let _ = cast(&mut test, &touch(&[(Effect::Poison, 1)]), Some(mob));
    for _ in 0..200 {
        test.world.action_clock.advance(1);
        let tick = test.world.action_clock.ticks();
        crate::entities::poison_mobs(&mut test.world, tick);
    }
    assert!(mob_health(&test, mob).unwrap() < full, "poisoned");
}

#[test]
fn stoneskin_and_warding_lie_on_the_caster() {
    let mut test = magician();
    let resisting = test.client.resistance_for_test();
    let _ = cast(
        &mut test,
        &on_self(&[(Effect::Void, 1), (Effect::Earth, 1)]),
        None,
    );
    assert!(under(&test, STONESKIN));
    assert!(test.client.fighter().guard.blows_taken < 1000);
    let _ = cast(&mut test, &on_self(&[(Effect::Warding, 1)]), None);
    assert!(under(&test, WARDED));
    assert!(test.client.resistance_for_test() > resisting);
}

#[test]
fn reversed_warding_dispels_and_reversed_poison_cures() {
    let mut test = magician();
    let _ = cast(&mut test, &on_self(&[(Effect::Illusion, 1)]), None);
    assert!(under(&test, ILLUSION));
    assert!(test.client.fighter().guard.defence > 1000);
    let _ = cast(
        &mut test,
        &on_self(&[(Effect::Void, 1), (Effect::Warding, 1)]),
        None,
    );
    assert!(!under(&test, ILLUSION), "dispelled");
    assert_eq!(test.client.fighter().guard.defence, 1000);
    let tick = test.world.action_clock.ticks();
    test.client.sheet.effects.add(shared::bonus::poison(tick));
    let _ = cast(
        &mut test,
        &on_self(&[(Effect::Void, 1), (Effect::Poison, 1)]),
        None,
    );
    let poisoned = (test.client.sheet.effects.0.iter()).any(|e| e.source == Source::Poison);
    assert!(!poisoned, "cured");
}

#[test]
fn reversed_curse_lifts_a_worn_curse() {
    let mut test = magician();
    let worn = test.client.inventory.worn.first_mut();
    worn.expect("a new character wears clothes").favour = Some(Favour::Cursed);
    let _ = cast(
        &mut test,
        &on_self(&[(Effect::Void, 1), (Effect::Curse, 1)]),
        None,
    );
    assert_eq!(test.client.inventory.worn[0].favour, None);
}
