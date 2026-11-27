use super::*;
use crate::fruit::{APPLE, Fruits, new_item};
use crate::id::Id;
use crate::item::{Item, ItemKind, Placed, Rotation};
use crate::poi::ChunkPos;
use crate::quest::{QuestId, Slot};

fn apple(count: u32) -> Item {
    let def = Fruits::built_in().get(APPLE).expect("the apple");
    new_item(def, count, Id::for_test(1), None)
}

#[test]
fn a_common_apple_is_one_copper() {
    let apple = apple(1);
    assert_eq!(copper_shown(each(&apple)), 1);
    let poor = Item {
        quality: Some(Quality::Poor),
        ..apple
    };
    assert_eq!(each(&poor), FINE_PER_COPPER / 2);
}

#[test]
fn quality_doubles() {
    let times: Vec<u64> = Quality::ALL.iter().map(|&q| quality_times(q)).collect();
    assert_eq!(times, [1, 2, 4, 8, 16, 32, 64]);
    let sword = |quality| Item {
        quality: Some(quality),
        ..Item::new(Id::for_test(2), ItemKind::Sword)
    };
    assert_eq!(
        each(&sword(Quality::Rare)),
        4 * each(&sword(Quality::Common))
    );
}

#[test]
fn coins_are_worth_what_they_say() {
    let gold = Item {
        count: 3,
        ..Item::new(Id::for_test(3), ItemKind::GoldCoin)
    };
    assert_eq!(copper_shown(total(&gold)), 30_000);
}

#[test]
fn a_stack_is_every_item_in_it() {
    let apples = apple(5);
    assert_eq!(total(&apples), 5 * each(&apples));
}

#[test]
fn a_bag_adds_what_is_in_it() {
    let empty = Item::new(Id::for_test(4), ItemKind::Bag);
    let mut full = empty.clone();
    full.contents.push(Placed {
        item: apple(4),
        x: 0,
        y: 0,
        rotation: Rotation::None,
    });
    assert_eq!(total(&full), total(&empty) + total(&apple(4)));
}

#[test]
fn quest_items_are_worth_nothing() {
    let parcel = Item {
        quest: Some(QuestId {
            town: ChunkPos { x: 0, y: 0 },
            slot: Slot::Feeder,
        }),
        ..Item::new(Id::for_test(5), ItemKind::Parcel)
    };
    assert_eq!(each(&parcel), 0);
}

#[test]
fn rounding_up_only_when_shown() {
    assert_eq!(copper_shown(0), 0);
    assert_eq!(copper_shown(1), 1);
    assert_eq!(copper_shown(1000), 1);
    assert_eq!(copper_shown(1001), 2);
}

#[test]
fn coins_split_by_kind() {
    let coins = Coins::from_copper(1_234_567);
    assert_eq!(
        coins,
        Coins {
            gold: 123,
            silver: 45,
            copper: 67
        }
    );
}

#[test]
fn every_kind_has_a_value() {
    for kind in ItemKind::ALL {
        let item = Item::new(Id::for_test(6), kind);
        if item.kind.coin_value().is_none() {
            assert!(each(&item) > 0, "{kind:?} is worth nothing");
        }
    }
}
