//! Feeding yeast and beginning a starter through a connected client: flour and
//! water used, an early feed split into a second yeast using an empty bottle,
//! and a bottle of water turned into a starter.

use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid, Water};
use shared::protocol::{ClientMessage, YeastAsk};

use crate::connections::tests::TestClient;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn bottle(id: u128, liquid: Option<Liquid>) -> Item {
    Item {
        form: Some(ItemForm::Bottle(BottleForm::new(liquid, 0))),
        ..Item::new(Id::for_test(id), ItemKind::Bottle)
    }
}

fn flour(id: u128, sacks: u32) -> Item {
    let grain = shared::flour::grains().next().unwrap();
    shared::flour::new_item(grain, sacks, Id::for_test(id), None).unwrap()
}

fn count(test: &TestClient, kind: ItemKind) -> u32 {
    test.client.inventory.count_of(kind)
}

const WATER: Option<Liquid> = Some(Liquid::Water(Water::Plain));

#[test]
fn an_early_feed_splits_the_yeast_using_flour_water_and_a_bottle() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(
        &mut test,
        shared::yeast::new_item(Quality::Common, Id::for_test(1), None),
    );
    give(&mut test, flour(2, 3));
    give(&mut test, bottle(3, WATER));
    give(&mut test, bottle(4, WATER));
    give(&mut test, bottle(5, None));
    test.send(&[ClientMessage::Yeast(YeastAsk::Feed {
        yeast: Id::for_test(1),
        flour: Id::for_test(2),
    })]);
    assert_eq!(count(&test, ItemKind::Yeast), 2);
    assert_eq!(count(&test, ItemKind::Flour), 1);
    // Two waters drawn, the empty bottle gone into the new yeast.
    assert_eq!(count(&test, ItemKind::Bottle), 2);
    assert_eq!(test.client.inventory.water_source(), None);
}

#[test]
fn flour_on_a_bottle_of_water_begins_a_starter() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, flour(2, 1));
    give(&mut test, bottle(3, WATER));
    test.send(&[ClientMessage::Yeast(YeastAsk::Start {
        water: Id::for_test(3),
        flour: Id::for_test(2),
    })]);
    assert_eq!(count(&test, ItemKind::Flour), 0);
    assert_eq!(count(&test, ItemKind::Bottle), 0);
    let starter = test.client.inventory.stack_to_use(ItemKind::Yeast).unwrap();
    let starter = test.client.inventory.find(starter).unwrap();
    assert!(starter.yeast().unwrap().developing > 0);
}
