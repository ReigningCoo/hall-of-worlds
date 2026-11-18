//! Pouring and drinking from vessels through a connected client: a tap to
//! draw from a cask, one on the ground too, a bottle off a stack filled and
//! sealed, and water drunk a serving at a time.

use shared::cask::{CaskForm, CaskSize};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::material::Material;
use shared::potion::{BottleForm, Liquid, Water};
use shared::protocol::ClientMessage;
use shared::vessel::{SERVING_ML, VesselForm, VesselKind};

use crate::connections::tests::TestClient;

const WATER: Liquid = Liquid::Water(Water::Plain);

fn with_form(id: u128, kind: ItemKind, form: ItemForm) -> Item {
    Item {
        form: Some(form),
        ..Item::new(Id::for_test(id), kind)
    }
}

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn pour(test: &mut TestClient, from: u128, into: u128) {
    test.send(&[ClientMessage::Pour {
        from: Id::for_test(from),
        into: Id::for_test(into),
    }]);
}

fn carried(test: &TestClient, id: u128) -> Item {
    test.client
        .inventory
        .find(Id::for_test(id))
        .cloned()
        .unwrap()
}

fn full_pin(id: u128) -> Item {
    let cask = CaskForm::new(CaskSize::Pin, Material::Oak).full_of(WATER);
    with_form(id, ItemKind::Cask, ItemForm::Cask(cask))
}

fn tankard(id: u128) -> Item {
    let form = VesselForm::new(VesselKind::Tankard);
    with_form(id, ItemKind::Vessel, ItemForm::Vessel(form))
}

#[test]
fn drawing_from_a_cask_needs_a_tap() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, full_pin(1));
    give(&mut test, tankard(2));
    pour(&mut test, 1, 2);
    assert!(carried(&test, 2).vessel().unwrap().is_empty(), "no tap");
    give(&mut test, Item::new(Id::for_test(3), ItemKind::Tap));
    pour(&mut test, 1, 2);
    assert_eq!(carried(&test, 2).vessel().unwrap().ml, 500);
    let cask = carried(&test, 1);
    let cask = cask.cask().unwrap();
    assert_eq!((cask.ml, cask.opened), (19_500, true));
}

#[test]
fn a_bottle_off_a_stack_is_filled_sealed_on_its_own() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, full_pin(1));
    give(&mut test, Item::new(Id::for_test(3), ItemKind::Tap));
    let mut bottles = with_form(
        4,
        ItemKind::Bottle,
        ItemForm::Bottle(BottleForm::new(None, 1)),
    );
    bottles.count = 3;
    give(&mut test, bottles);
    pour(&mut test, 1, 4);
    assert_eq!(carried(&test, 4).count, 2, "two still empty");
    let filled = test.client.inventory.items.iter().find_map(|placed| {
        let bottle = placed.item.bottle()?;
        (!bottle.is_empty()).then(|| bottle.clone())
    });
    let filled = filled.expect("a filled bottle");
    assert!(filled.sealed);
    assert_eq!(carried(&test, 1).cask().unwrap().ml, 19_900);
}

#[test]
fn a_cask_on_the_ground_can_be_drawn_from() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, full_pin(1));
    give(&mut test, tankard(2));
    give(&mut test, Item::new(Id::for_test(3), ItemKind::Tap));
    test.send(&[ClientMessage::DropItem {
        item: Id::for_test(1),
    }]);
    assert!(test.client.inventory.find(Id::for_test(1)).is_none());
    pour(&mut test, 1, 2);
    assert_eq!(carried(&test, 2).vessel().unwrap().ml, 500);
}

#[test]
fn water_is_drunk_from_a_vessel_which_stays() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let skin = VesselForm::new(VesselKind::Waterskin).full_of(WATER);
    give(
        &mut test,
        with_form(5, ItemKind::Vessel, ItemForm::Vessel(skin)),
    );
    test.send(&[ClientMessage::Consume {
        item: Id::for_test(5),
    }]);
    let left = carried(&test, 5).vessel().unwrap().ml;
    assert_eq!(left, 2000 - SERVING_ML);
}
