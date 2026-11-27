//! Counting a gathering step's herb wherever it is carried, and taking the
//! poorest first.

use super::*;
use crate::creature::CreatureKind;
use crate::herb::new_item;
use crate::item::{Placed, Rotation, StackLimits};

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// `count` of a built-in herb at `quality`, with id `n` (a picked herb's
/// quality is its plant's rarity; an old save's could be anything).
fn herbs(key: &str, quality: Quality, count: u32, n: u128) -> Item {
    let def = Herbs::built_in_ref().get(key).expect("a built-in herb");
    let mut item = new_item(def, count, id(n), None);
    item.quality = Some(quality);
    item
}

fn placed(item: Item) -> Placed {
    Placed {
        item,
        x: 0,
        y: 0,
        rotation: Rotation::None,
    }
}

/// A human carrying 3 Common yarrow in their own grid, and a bag (10) holding
/// a herb pouch (11) with 2 Poor yarrow, 4 Uncommon yarrow beside it in the
/// bag, and 5 sage that never count.
fn carrying() -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    let mut pouch = Item::new(id(11), ItemKind::HerbPouch);
    pouch
        .contents
        .push(placed(herbs("yarrow", Quality::Poor, 2, 1)));
    let mut bag = Item::new(id(10), ItemKind::Bag);
    bag.contents.push(placed(pouch));
    let mut beside = placed(herbs("yarrow", Quality::Uncommon, 4, 2));
    beside.x = 1;
    bag.contents.push(beside);
    inventory.items.push(placed(bag));
    for item in [
        herbs("yarrow", Quality::Common, 3, 3),
        herbs("sage", Quality::Common, 5, 4),
    ] {
        let limits = StackLimits::default();
        inventory
            .give(item, limits, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory
}

fn yarrow(least: Quality) -> HerbAsk {
    HerbAsk {
        herb: "yarrow".to_owned(),
        least,
    }
}

#[test]
fn every_one_carried_counts_in_bags_at_any_depth() {
    let inventory = carrying();
    assert_eq!(yarrow(Quality::Poor).carried(&inventory), 9);
    // Asking better leaves the poorer out.
    assert_eq!(yarrow(Quality::Common).carried(&inventory), 7);
    assert_eq!(yarrow(Quality::Uncommon).carried(&inventory), 4);
    assert_eq!(yarrow(Quality::Rare).carried(&inventory), 0);
}

#[test]
fn the_count_goes_down_as_they_go() {
    let mut inventory = carrying();
    let stack = (inventory.items.iter())
        .find(|placed| placed.item.herb().is_some_and(|form| form.herb == "yarrow"))
        .map(|placed| placed.item.id)
        .unwrap();
    inventory
        .take_one(stack, &mut crate::id::stand_ins())
        .unwrap();
    assert_eq!(yarrow(Quality::Poor).carried(&inventory), 8);
}

#[test]
fn handing_in_takes_the_poorest_first() {
    let mut inventory = carrying();
    assert!(yarrow(Quality::Poor).take(&mut inventory, 4, &mut crate::id::stand_ins()));
    // Both Poor and 2 of the 3 Common went; the Uncommon are kept.
    assert_eq!(yarrow(Quality::Poor).carried(&inventory), 5);
    assert_eq!(yarrow(Quality::Common).carried(&inventory), 5);
    assert_eq!(yarrow(Quality::Uncommon).carried(&inventory), 4);
    // Sage is untouched.
    let sage = HerbAsk {
        herb: "sage".to_owned(),
        least: Quality::Poor,
    };
    assert_eq!(sage.carried(&inventory), 5);
}

#[test]
fn asking_better_takes_only_those_good_enough() {
    let mut inventory = carrying();
    assert!(yarrow(Quality::Common).take(&mut inventory, 4, &mut crate::id::stand_ins()));
    assert_eq!(yarrow(Quality::Poor).carried(&inventory), 5);
    // The Poor stay, the 3 Common and 1 Uncommon went.
    assert_eq!(yarrow(Quality::Uncommon).carried(&inventory), 3);
}

#[test]
fn too_few_takes_nothing() {
    let mut inventory = carrying();
    let before = inventory.clone();
    assert!(!yarrow(Quality::Poor).take(&mut inventory, 10, &mut crate::id::stand_ins()));
    assert!(!yarrow(Quality::Uncommon).take(&mut inventory, 5, &mut crate::id::stand_ins()));
    assert_eq!(inventory, before);
}

#[test]
fn quality_steps_above_common_raise_the_reward() {
    assert_eq!(yarrow(Quality::Poor).steps_above_common(), 0);
    assert_eq!(yarrow(Quality::Common).steps_above_common(), 0);
    assert_eq!(yarrow(Quality::Uncommon).steps_above_common(), 1);
    assert_eq!(yarrow(Quality::Rare).steps_above_common(), 2);
}
