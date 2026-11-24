//! Identify, Teleport, Haste and Mana (magic plan PR 13): Identify finds out
//! what worn things hide and Conceal hides it again, Haste makes the caster
//! faster, a Teleport's move goes straight to a cell in sight and only once,
//! Mana on yourself never gives back more than it cost.

use shared::item::Favour;
use shared::protocol::ClientMessage;
use shared::spell::{Effect, TELEPORTING};
use shared::spot::Spot;
use shared::vitals::Vital;

use super::super::tests::run;
use super::{cast, magician, on_self};

#[test]
fn identify_finds_a_worn_curse_out_and_conceal_hides_it_again() {
    let mut test = magician();
    let worn = test.client.inventory.worn.first_mut();
    worn.expect("a new character wears clothes").favour = Some(Favour::Cursed);
    assert!(!test.client.inventory.worn[0].known.favour);
    let _ = cast(&mut test, &on_self(&[(Effect::Identify, 1)]), None);
    assert!(test.client.inventory.worn[0].known.favour, "found out");
    let conceal = on_self(&[(Effect::Void, 1), (Effect::Identify, 1)]);
    let _ = cast(&mut test, &conceal, None);
    assert!(!test.client.inventory.worn[0].known.favour, "hidden again");
}

#[test]
fn haste_makes_the_caster_faster() {
    let mut test = magician();
    let before = test.client.speed;
    let _ = cast(&mut test, &on_self(&[(Effect::Haste, 1)]), None);
    assert!(
        test.client.speed > before,
        "{} after {before}",
        test.client.speed
    );
}

#[test]
fn mana_on_yourself_never_gains_you_any() {
    let mut test = magician();
    let _ = cast(&mut test, &on_self(&[(Effect::Mana, 1)]), None);
    let tick = test.world.action_clock.ticks();
    let limits = test.client.limits(tick);
    let mana = test.client.sheet.vitals.at(Vital::Mana, limits, tick);
    assert!(
        mana.milli() < limits.mana * 1000,
        "{mana:?} of {}",
        limits.mana
    );
}

#[test]
fn a_teleport_goes_straight_to_a_cell_in_sight_once() {
    let mut test = magician();
    let visit = test.client.visit.as_ref().unwrap();
    let (x, y) = visit.room_to_walk_east(3);
    test.send(&[ClientMessage::DevTeleport { x, y }]);
    let _ = cast(&mut test, &on_self(&[(Effect::Teleport, 1)]), None);
    assert!(test.client.sheet.has_spell(TELEPORTING));
    let to = Spot::middle_of((x + 3, y));
    let map = test.client.map;
    test.send(&[ClientMessage::Teleport { map, number: 1, to }]);
    run(&mut test, 10, false);
    assert_eq!(test.client.visit.as_ref().unwrap().spot(), to);
    assert!(!test.client.sheet.has_spell(TELEPORTING), "used up");
    // A second asks to go back: refused, there is no Teleport left.
    let back = Spot::middle_of((x, y));
    test.send(&[ClientMessage::Teleport {
        map,
        number: 2,
        to: back,
    }]);
    run(&mut test, 10, false);
    assert_eq!(test.client.visit.as_ref().unwrap().spot(), to);
}
