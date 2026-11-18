//! Press Juice through a connected client: the fruit named is pressed a
//! pressing at a time into a carried vessel until it is full, and juice drunk
//! from it restores Mana.

use shared::ability::Ability;
use shared::bonus::Source;
use shared::fruit::{FruitTag, Fruits};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::Liquid;
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::vessel::{VesselForm, VesselKind};

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a pressing takes untrained, and a few more.
const ONE_PRESSING: u64 = 15 * 20 + 2;

const APPLES: u128 = 91;
const JUG: u128 = 92;

/// Twenty apples, an empty jug, and an oil press if `press`.
fn carrying(test: &mut TestClient, press: bool) {
    let def = Fruits::built_in().get("apple").unwrap();
    let apples = shared::fruit::new_item(def, 20, Id::for_test(APPLES), None);
    let mut jug = Item::new(Id::for_test(JUG), ItemKind::Vessel);
    jug.form = Some(ItemForm::Vessel(VesselForm::new(VesselKind::Jug)));
    let mut items = vec![apples, jug];
    if press {
        items.push(Item::new(Id::for_test(93), ItemKind::OilPress));
    }
    let new_id = &mut shared::id::stand_ins();
    for item in items {
        let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
        assert!(given.is_ok());
    }
}

fn press() -> ClientMessage {
    ClientMessage::UseAbility {
        ability: Ability::PressJuice,
        target: Some(Id::for_test(APPLES)),
    }
}

fn jug(test: &TestClient) -> VesselForm {
    let jug = test.client.inventory.find(Id::for_test(JUG)).unwrap();
    jug.vessel().unwrap().clone()
}

fn apples_left(test: &TestClient) -> u32 {
    let apples = test.client.inventory.find(Id::for_test(APPLES));
    apples.map_or(0, |stack| stack.count())
}

#[test]
fn fruit_is_pressed_into_a_vessel_until_it_is_full() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    carrying(&mut test, true);
    test.send(&[press()]);
    assert!(test.client.fight.pending.is_some());
    run(&mut test, ONE_PRESSING, false);
    // Ten apples, 2 kg, 1.41 litres.
    assert_eq!(jug(&test).ml, 1410);
    assert_eq!(apples_left(&test), 10);
    run(&mut test, ONE_PRESSING, false);
    // One more apple fills it.
    let full = jug(&test);
    assert_eq!(full.ml, 1500);
    assert_eq!(full.liquid, Some(Liquid::Juice("apple".into())));
    assert_eq!(apples_left(&test), 9);
    assert!(test.client.fight.pending.is_none(), "nowhere for more");
    let juiced = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::Juiced(_),
                ..
            }
        )
    });
    assert!(juiced);
}

#[test]
fn nothing_is_pressed_without_an_oil_press() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    carrying(&mut test, false);
    test.send(&[press()]);
    assert!(test.client.fight.pending.is_none());
    assert_eq!(apples_left(&test), 20);
}

#[test]
fn juice_drunk_from_a_vessel_restores_mana() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    carrying(&mut test, true);
    test.send(&[press()]);
    run(&mut test, ONE_PRESSING, false);
    test.send(&[ClientMessage::CancelAbility]);
    test.send(&[ClientMessage::Consume {
        item: Id::for_test(JUG),
    }]);
    assert_eq!(jug(&test).ml, 1410 - 250);
    let drinking = test
        .client
        .sheet
        .effects
        .0
        .iter()
        .any(|effect| effect.source == Source::Juice(FruitTag::of("apple")));
    assert!(drinking);
}
