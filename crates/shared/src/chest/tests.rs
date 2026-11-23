//! Chests and wardrobes: what they take, filling their grids, and their
//! furniture pieces.

use super::*;
use crate::crafting_bench::NotTaken;
use crate::door::Side;
use crate::furniture::{Bed, Furniture, Held, Holder, Material, Piece, Wood, footprint};
use crate::id::Id;
use crate::item::{Item, ItemKind};

fn item(kind: ItemKind, n: u128) -> Item {
    Item::new(Id::for_test(n), kind)
}

fn piece(furniture: Furniture, long: u8, short: u8, facing: Side) -> Piece {
    Piece {
        at: (3, 4),
        size: footprint(furniture, (long, short), facing),
        facing,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

#[test]
fn a_chest_takes_anything_a_wardrobe_only_clothes() {
    let chest = Storage::Chest(Chest::Large);
    let wardrobe = Storage::Wardrobe(Wardrobe::Wide);
    for kind in [ItemKind::Ring, ItemKind::Longsword, ItemKind::Backpack] {
        assert!(chest.takes(kind), "{kind:?}");
        assert!(!wardrobe.takes(kind), "{kind:?}");
    }
    for kind in [ItemKind::Tunic, ItemKind::Gauntlets] {
        assert!(chest.takes(kind) && wardrobe.takes(kind), "{kind:?}");
    }
}

#[test]
fn filling_goes_until_there_is_no_room() {
    let mut stored = Stored::new(Storage::Chest(Chest::Small));
    let (width, height) = Storage::Chest(Chest::Small).grid();
    // Rings take one cell each.
    for n in 0..u128::from(width) * u128::from(height) {
        assert_eq!(stored.put(item(ItemKind::Ring, n)), Ok(()), "{n}");
    }
    assert_eq!(stored.put(item(ItemKind::Ring, 999)), Err(NotTaken));
    assert!(stored.take(Id::for_test(5)).is_some());
    assert_eq!(stored.put(item(ItemKind::Ring, 999)), Ok(()));
    let mut cells: Vec<_> = stored.items.iter().flat_map(|p| p.cells()).collect();
    cells.sort_unstable();
    cells.dedup();
    assert_eq!(cells.len(), stored.items.len(), "nothing overlaps");
}

#[test]
fn a_log_is_too_long_for_a_small_chest() {
    let log = item(ItemKind::Log, 1);
    assert_eq!(
        Stored::new(Storage::Chest(Chest::Small)).put(log.clone()),
        Err(NotTaken)
    );
    assert_eq!(Stored::new(Storage::Chest(Chest::Large)).put(log), Ok(()));
}

#[test]
fn a_wardrobe_refuses_what_is_not_clothes() {
    let mut stored = Stored::new(Storage::Wardrobe(Wardrobe::Narrow));
    assert_eq!(stored.put(item(ItemKind::Ring, 1)), Err(NotTaken));
    assert_eq!(stored.put(item(ItemKind::Tunic, 2)), Ok(()));
    assert_eq!(stored.items().count(), 1);
}

#[test]
fn chests_and_wardrobes_are_furniture_with_places() {
    for chest in Chest::ALL {
        let furniture = Furniture::Chest { chest };
        let holder = Holder::of(furniture).expect("holds things");
        assert_eq!(holder, Holder::Store(Storage::Chest(chest)));
        assert!(!holder.is_drink_storage());
        assert_eq!(holder.per_place(), u32::MAX, "whole stacks");
        let mut held = Held::new(holder);
        assert_eq!(held.put(item(ItemKind::Ring, 1)), Ok(None));
        assert_eq!(held.holder(), holder);
        assert!(held.take(Id::for_test(1)).is_some());
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            assert!(piece(furniture, chest.length(), 1, facing).is_well_made());
        }
    }
    for wardrobe in Wardrobe::ALL {
        let furniture = Furniture::Wardrobe { wardrobe };
        assert_eq!(holder_name(furniture), wardrobe.name());
        assert!(piece(furniture, wardrobe.length(), 1, Side::West).is_well_made());
        assert!(!piece(furniture, wardrobe.length() + 1, 1, Side::West).is_well_made());
    }
}

fn holder_name(furniture: Furniture) -> String {
    Holder::of(furniture).expect("holds things").name()
}

#[test]
fn beds_are_two_long_and_hold_nothing() {
    for bed in Bed::ALL {
        let furniture = Furniture::Bed { bed };
        assert!(Holder::of(furniture).is_none());
        assert_eq!(
            furniture.cost_per_mille(),
            crate::furniture::TABLE_COST_PER_MILLE
        );
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            let made = piece(furniture, Bed::LENGTH, bed.width(), facing);
            assert!(made.is_well_made(), "{bed:?} {facing:?}");
            let (width, height) = made.size;
            let along_x = matches!(facing, Side::East | Side::West);
            assert_eq!(if along_x { width } else { height }, Bed::LENGTH);
        }
    }
    let squashed = piece(Furniture::Bed { bed: Bed::Single }, 1, 1, Side::North);
    assert!(!squashed.is_well_made());
}
