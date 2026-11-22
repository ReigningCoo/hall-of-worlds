//! Filling a lantern through a connected client, and its oil burning by
//! playtime only where it burns.

use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::lantern::{FOUNT_ML, LanternForm, SECONDS_PER_ML};
use shared::potion::{BottleForm, Liquid, Oil, Water};
use shared::pour::BOTTLE_ML;
use shared::protocol::ClientMessage;

use crate::connections::tests::TestClient;

const LANTERN: u128 = 1;

fn lantern(ml: u32) -> Item {
    Item {
        form: Some(ItemForm::Lantern(LanternForm { ml, burnt: 0 })),
        ..Item::new(Id::for_test(LANTERN), ItemKind::Lantern)
    }
}

fn bottles(id: u128, liquid: Liquid, count: u32) -> Item {
    Item {
        form: Some(ItemForm::Bottle(BottleForm::new(Some(liquid), 3))),
        count,
        ..Item::new(Id::for_test(id), ItemKind::Bottle)
    }
}

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn fill(test: &mut TestClient, bottle: u128) {
    test.send(&[ClientMessage::FillLantern {
        lantern: Id::for_test(LANTERN),
        bottle: Id::for_test(bottle),
    }]);
}

fn oil_in_lantern(test: &TestClient) -> u32 {
    let found = test.client.inventory.find(Id::for_test(LANTERN));
    found.and_then(Item::lantern).unwrap().ml
}

/// Bottles carried that hold nothing.
fn empty_bottles(test: &TestClient) -> u32 {
    let stacks = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    let empty =
        |item: &Item| matches!(&item.form, Some(ItemForm::Bottle(form)) if form.liquid.is_none());
    stacks.filter(|item| empty(item)).map(Item::count).sum()
}

#[test]
fn a_held_lantern_is_filled_from_one_bottle_off_a_stack() {
    let mut test = TestClient::new();
    give(&mut test, lantern(0));
    assert!(test.client.inventory.wear(Id::for_test(LANTERN)).is_ok());
    give(&mut test, bottles(2, Liquid::Oil(Oil::Seed), 3));
    let empty = empty_bottles(&test);
    fill(&mut test, 2);
    assert_eq!(oil_in_lantern(&test), BOTTLE_ML);
    assert_eq!(
        empty_bottles(&test),
        empty + 1,
        "the bottle comes back empty"
    );
    let oil = test.client.inventory.find(Id::for_test(2)).unwrap();
    assert_eq!(oil.count(), 2, "one bottle taken off the stack");
    assert!(test.client.inventory.lantern_lit());
}

#[test]
fn a_full_lantern_or_a_bottle_of_water_is_refused() {
    let mut test = TestClient::new();
    give(&mut test, lantern(FOUNT_ML));
    give(&mut test, bottles(2, Liquid::Oil(Oil::Seed), 1));
    let empty = empty_bottles(&test);
    fill(&mut test, 2);
    assert_eq!(empty_bottles(&test), empty, "full: nothing poured");

    let mut test = TestClient::new();
    give(&mut test, lantern(0));
    give(&mut test, bottles(2, Liquid::Water(Water::Plain), 1));
    fill(&mut test, 2);
    assert_eq!(oil_in_lantern(&test), 0, "water isn't oil");
}

#[test]
fn only_a_held_lantern_burns_and_its_client_hears_every_ten_ml() {
    let mut test = TestClient::new();
    give(&mut test, lantern(105));
    assert!(
        !test.client.burn_lanterns(3600),
        "in the grid: nothing burns"
    );
    assert_eq!(oil_in_lantern(&test), 105);

    assert!(test.client.inventory.wear(Id::for_test(LANTERN)).is_ok());
    assert!(
        !test.client.burn_lanterns(SECONDS_PER_ML),
        "1 ml: not told yet"
    );
    assert!(
        test.client.burn_lanterns(10 * SECONDS_PER_ML),
        "past 100 ml: told"
    );
    assert_eq!(oil_in_lantern(&test), 94);
    assert!(test.client.burn_lanterns(3600), "burnt out: told");
    assert_eq!(oil_in_lantern(&test), 0);
    assert!(!test.client.inventory.lantern_lit());
}
