//! Press Oil through a connected client: the stack named decides what is
//! pressed, bottles are filled one after another while there is enough, and
//! nothing begins without an oil press.

use shared::ability::{ALCHEMY, Ability};
use shared::fruit::Fruits;
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid, Oil, grams_per_bottle};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::skill::SkillId;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a bottle takes untrained, and a few more.
const ONE_BOTTLE: u64 = 15 * 20 + 2;

/// Walnuts enough for `bottles` bottles of Nut Oil, so many empty bottles,
/// and an oil press if `press`: the walnuts' stack id.
fn carrying(test: &mut TestClient, bottles: u32, press: bool) -> Id {
    let def = Fruits::built_in().get("walnut").unwrap();
    let one = shared::fruit::new_item(def, 1, Id::for_test(90), None).own_weight();
    let count = grams_per_bottle(Oil::Nut).div_ceil(one) * bottles;
    let walnuts = shared::fruit::new_item(def, count, Id::for_test(91), None);
    let mut empty = Item::new(Id::for_test(92), ItemKind::Bottle);
    empty.form = Some(ItemForm::Bottle(BottleForm::new(None, 1)));
    empty.count = bottles;
    let mut items = vec![walnuts, empty];
    if press {
        items.push(Item::new(Id::for_test(93), ItemKind::OilPress));
    }
    let new_id = &mut shared::id::stand_ins();
    for item in items {
        let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
        assert!(given.is_ok());
    }
    Id::for_test(91)
}

fn press(stack: Id) -> ClientMessage {
    ClientMessage::UseAbility {
        ability: Ability::PressOil,
        target: Some(stack),
    }
}

/// The bottles carried, by what they hold, with how many of each.
fn bottles(test: &TestClient) -> Vec<(Option<Liquid>, u32)> {
    let items = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    items
        .filter(|item| item.kind == ItemKind::Bottle)
        .map(|item| (item.bottle().unwrap().liquid.clone(), item.count()))
        .collect()
}

fn has_walnuts(test: &TestClient) -> bool {
    let mut items = test.client.inventory.items.iter();
    items.any(|placed| placed.item.kind == ItemKind::Fruit)
}

#[test]
fn bottles_are_pressed_while_there_is_enough_and_practise_alchemy() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let stack = carrying(&mut test, 2, true);
    test.send(&[press(stack)]);
    assert!(test.client.fight.pending.is_some());
    run(&mut test, ONE_BOTTLE, false);
    run(&mut test, ONE_BOTTLE, false);
    assert_eq!(bottles(&test), vec![(Some(Liquid::Oil(Oil::Nut)), 2)]);
    assert!(!has_walnuts(&test), "all used up");
    let pressed = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::Pressed(Oil::Nut),
                ..
            }
        )
    });
    assert!(pressed);
    let alchemy = SkillId::find(ALCHEMY).unwrap();
    assert!(test.client.sheet.skills.practice(alchemy) > 0);
    assert!(test.client.fight.pending.is_none(), "nothing left to press");
}

#[test]
fn nothing_is_pressed_without_an_oil_press() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let stack = carrying(&mut test, 1, false);
    test.send(&[press(stack)]);
    assert!(test.client.fight.pending.is_none());
}

#[test]
fn being_harmed_stops_it() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let stack = carrying(&mut test, 1, true);
    test.send(&[press(stack)]);
    assert!(test.client.fight.pending.is_some());
    let hurt = shared::protocol::DevVitals::Hurt(1);
    test.send(&[ClientMessage::DevVitals(hurt)]);
    assert!(test.client.fight.pending.is_none());
    run(&mut test, ONE_BOTTLE, false);
    assert_eq!(bottles(&test), vec![(None, 1)]);
    assert!(has_walnuts(&test));
}
