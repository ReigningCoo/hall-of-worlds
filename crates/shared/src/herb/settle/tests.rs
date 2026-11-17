use super::*;
use crate::herb::{HerbDef, new_item};
use crate::id::{Id, stand_ins};
use crate::item::{Holder, Quality};

fn def(key: &str) -> HerbDef {
    Herbs::built_in().get(key).expect("a built-in herb").clone()
}

/// `count` of `def` with id `n`, given the rolled quality of an old save.
fn old(def: &HerbDef, quality: Quality, count: u32, n: u128) -> Item {
    let mut item = new_item(def, count, Id::for_test(n), None);
    item.quality = Some(quality);
    item
}

/// An inventory holding these, each in its own place (nothing stacks).
fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    for item in items {
        inventory
            .give(item, StackLimits::NONE, &mut stand_ins())
            .expect("room");
    }
    inventory
}

#[test]
fn old_herbs_take_their_rarity_and_join() {
    let roseroot = def("roseroot");
    assert_eq!(roseroot.abundance.quality(), Quality::Rare);
    let mut inventory = carrying(vec![
        old(&roseroot, Quality::Poor, 3, 1),
        old(&roseroot, Quality::Common, 4, 2),
        old(&roseroot, Quality::Rare, 2, 3),
    ]);
    let changed = settle_herbs(&mut inventory, &Herbs::built_in(), StackLimits::default());
    assert_eq!(changed, 2);
    let grid = inventory.grid(Holder::Inventory).unwrap();
    assert_eq!(grid.len(), 1, "one stack of roseroot");
    assert_eq!(grid[0].item.count, 9);
    assert_eq!(grid[0].item.quality, Some(Quality::Rare));
    assert_eq!(grid[0].item.id, Id::for_test(1), "the first stays");
}

#[test]
fn joining_stops_at_the_stack_limit() {
    let roseroot = def("roseroot");
    let mut inventory = carrying(vec![
        old(&roseroot, Quality::Poor, 6, 1),
        old(&roseroot, Quality::Common, 7, 2),
    ]);
    let limits = StackLimits {
        most: 10,
        ..StackLimits::default()
    };
    settle_herbs(&mut inventory, &Herbs::built_in(), limits);
    let grid = inventory.grid(Holder::Inventory).unwrap();
    let counts: Vec<u32> = grid.iter().map(|placed| placed.item.count).collect();
    assert_eq!(counts, [10, 3]);
}

#[test]
fn herbs_the_server_lacks_are_left_alone() {
    let mut gone = def("roseroot");
    gone.key = "a_mods_herb".into();
    let mut inventory = carrying(vec![
        old(&gone, Quality::Poor, 3, 1),
        old(&def("aloe"), Quality::Poor, 2, 2),
    ]);
    let changed = settle_herbs(&mut inventory, &Herbs::built_in(), StackLimits::default());
    assert_eq!(changed, 1);
    let grid = inventory.grid(Holder::Inventory).unwrap();
    assert_eq!(grid[0].item.quality, Some(Quality::Poor));
    assert_eq!(grid[1].item.quality, Some(Quality::Common));
}
