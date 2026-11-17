//! The Herb Pouch (owner, 2026-11-17): a holder like the keyring, for herbs
//! only, where herbs from outside go first and stack by herb and quality, and
//! whose herbs weigh less by its quality.

use super::*;
use crate::creature::CreatureKind;
use crate::herb::{HerbDef, Herbs, new_item};
use crate::id::Id;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

fn def(key: &str) -> HerbDef {
    Herbs::built_in().get(key).expect("a built-in herb").clone()
}

/// `count` roseroot (or `key`) of this quality, with id `n`: a herb's quality
/// is its plant's rarity, but an old save's could be anything.
fn herbs(key: &str, quality: Quality, count: u32, n: u128) -> Item {
    let mut item = new_item(&def(key), count, id(n), None);
    item.quality = Some(quality);
    item
}

/// A human with a 6 × 4 grid wearing a Rare belt (1, three slots) with a herb
/// pouch (2) tied to it, and these kinds from 3.
fn with_pouch(kinds: &[ItemKind]) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    let mut all = vec![ItemKind::Belt, ItemKind::HerbPouch];
    all.extend_from_slice(kinds);
    for (index, &kind) in all.iter().enumerate() {
        let item = Item::new(id(index as u128 + 1), kind);
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory.items[0].item.quality = Some(Quality::Rare);
    inventory.wear(id(1)).unwrap();
    inventory.tie_bag(id(2), 0).unwrap();
    inventory
}

fn give(inventory: &mut Inventory, item: Item) {
    inventory
        .give(item, StackLimits::default(), &mut crate::id::stand_ins())
        .expect("room");
}

/// The tied pouch's grid.
fn pouch() -> Holder {
    Holder::Inside(id(2))
}

#[test]
fn a_herb_pouch_is_a_growing_holder_for_herbs_only() {
    let inside = ItemKind::HerbPouch.def().container.expect("a container");
    assert!(inside.holder && inside.on_belt && inside.grows);
    assert!(inside.takes(ItemKind::Herb));
    assert!(!inside.takes(ItemKind::Key));
    assert_eq!(inside.width, 6);
    let mut inventory = with_pouch(&[ItemKind::Stone]);
    assert!(!inventory.adds_to_grid(id(2)));
    assert_eq!(
        inventory.move_item(id(3), pouch(), 0, 0, Rotation::None),
        Err(MoveRefused::NotTaken)
    );
}

#[test]
fn herbs_from_outside_go_into_the_pouch_and_stack() {
    let mut inventory = with_pouch(&[]);
    give(&mut inventory, herbs("roseroot", Quality::Common, 3, 10));
    give(&mut inventory, herbs("roseroot", Quality::Common, 4, 11));
    let grid = inventory.grid(pouch()).expect("its own grid");
    assert_eq!(grid.len(), 1, "one stack of roseroot");
    assert_eq!(grid[0].item.count, 7);
    // Another quality, or another herb, is a stack of its own.
    give(&mut inventory, herbs("roseroot", Quality::Rare, 2, 12));
    give(&mut inventory, herbs("yarrow", Quality::Common, 2, 13));
    assert_eq!(inventory.grid(pouch()).map(<[Placed]>::len), Some(3));
    assert_eq!(
        inventory.grid(Holder::Inventory).map(<[Placed]>::len),
        Some(0)
    );
}

#[test]
fn without_a_tied_pouch_herbs_go_in_the_own_grid() {
    let mut inventory = with_pouch(&[]);
    inventory
        .move_item(id(2), Holder::Inventory, 0, 0, Rotation::None)
        .unwrap();
    let herb = herbs("roseroot", Quality::Common, 1, 10);
    assert_eq!(
        inventory.route_for(&herb, StackLimits::default()),
        Holder::Inventory
    );
}

#[test]
fn a_herb_dragged_to_a_chosen_cell_stays_there() {
    let mut inventory = with_pouch(&[]);
    give(&mut inventory, herbs("roseroot", Quality::Common, 3, 10));
    assert_eq!(
        inventory.move_item(id(10), Holder::Inventory, 4, 2, Rotation::None),
        Ok(())
    );
    assert_eq!(
        inventory.placed(id(10)).map(|(at, _)| at),
        Some(Holder::Inventory)
    );
    // And into a chosen cell of the pouch.
    assert_eq!(
        inventory.move_item(id(10), pouch(), 3, 5, Rotation::None),
        Ok(())
    );
}

#[test]
fn a_herb_pouch_and_a_keyring_are_tied_together() {
    let mut inventory = with_pouch(&[ItemKind::Keyring, ItemKind::HerbPouch]);
    assert_eq!(inventory.tie_bag(id(3), 1), Ok(()));
    assert_eq!(inventory.tie_bag(id(4), 2), Err(MoveRefused::HolderTied));
    let key = Item::new(id(10), ItemKind::Key);
    let herb = herbs("roseroot", Quality::Common, 1, 11);
    let limits = StackLimits::default();
    assert_eq!(inventory.route_for(&key, limits), Holder::Inside(id(3)));
    assert_eq!(inventory.route_for(&herb, limits), pouch());
}

#[test]
fn a_herb_pouch_grows_without_limit() {
    let mut inventory = with_pouch(&[]);
    // Each quality of each of a few herbs is a stack of its own; then many single
    // unstackable ones fill row after row.
    for n in 0..300 {
        let quality = Quality::ALL[n % Quality::ALL.len()];
        let key = ["roseroot", "yarrow"][n % 2];
        give(&mut inventory, herbs(key, quality, 1, 100 + n as u128));
    }
    let counted: u32 = inventory
        .grid(pouch())
        .expect("its own grid")
        .iter()
        .map(|placed| placed.item.count)
        .sum();
    assert_eq!(counted, 300);
    for n in 0..200 {
        inventory
            .give(
                herbs("roseroot", Quality::Common, 1, 1000 + n),
                StackLimits::NONE,
                &mut crate::id::stand_ins(),
            )
            .unwrap();
    }
    assert_eq!(
        inventory.placed(id(1199)).map(|(at, _)| at),
        Some(pouch()),
        "past any first rows"
    );
}

#[test]
fn herbs_in_a_pouch_weigh_less_by_its_quality() {
    let mut inventory = with_pouch(&[]);
    give(&mut inventory, herbs("roseroot", Quality::Common, 12, 10));
    let herb_weight = inventory.find(id(10)).unwrap().total_weight();
    let pouch = ItemKind::HerbPouch.def().weight;
    let belt = inventory.worn[0].total_weight();
    assert_eq!(
        inventory.total_weight(),
        belt + pouch + Quality::Common.lightened(herb_weight)
    );
    inventory.bag_slots[0].quality = Some(Quality::Poor);
    assert_eq!(inventory.total_weight(), belt + pouch + herb_weight);
    inventory.bag_slots[0].quality = Some(Quality::Artifact);
    assert_eq!(inventory.total_weight(), belt + pouch);
}
