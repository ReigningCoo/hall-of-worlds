//! Bag slots: belts giving slots, bags tied to them adding to the own grid, and the
//! rules for tying, untying, belts and changing body.

use super::*;
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::wear::WearRefused;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A human with a 6 × 4 grid, carrying these kinds with ids from 1, each
/// placed by `give`.
fn carrying(kinds: &[ItemKind]) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    for (index, &kind) in kinds.iter().enumerate() {
        let item = Item::new(id(index as u128 + 1), kind);
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory
}

/// Moves a carried item to this place in the own grid.
fn put(inventory: &mut Inventory, n: u128, x: u8, y: u8) -> Result<(), MoveRefused> {
    inventory.move_item(id(n), Holder::Inventory, x, y, Rotation::None)
}

/// A human wearing a belt (1) with a pouch (2) tied to it, and these kinds from 3.
fn with_pouch(kinds: &[ItemKind]) -> Inventory {
    let mut all = vec![ItemKind::Belt, ItemKind::Pouch];
    all.extend_from_slice(kinds);
    let mut inventory = carrying(&all);
    inventory.wear(id(1)).unwrap();
    inventory.tie_bag(id(2), 0).unwrap();
    inventory
}

#[test]
fn a_belt_gives_slots_by_its_quality() {
    use Quality::*;
    let slots: Vec<usize> = Quality::ALL.map(|q| belt_slots(Some(q))).to_vec();
    assert_eq!(slots, [1, 1, 2, 3, 4, 5, 6]);
    assert_eq!(belt_slots(None), 1);
    let mut inventory = carrying(&[ItemKind::Belt]);
    assert_eq!(inventory.bag_slot_count(), 0, "not worn");
    inventory.items[0].item.quality = Some(Rare);
    inventory.wear(id(1)).unwrap();
    assert_eq!(inventory.bag_slot_count(), 3);
}

#[test]
fn a_tied_bag_adds_its_inside_to_the_end_of_the_own_grid() {
    let inventory = with_pouch(&[]);
    assert!(inventory.is_tied(id(2)));
    assert!(inventory.items.is_empty(), "the pouch left the grid");
    // 24 cells and a pouch's 16: 6 full rows and 4 cells of a seventh.
    assert_eq!(inventory.grid_cells(), 40);
    assert_eq!(inventory.grid_size(Holder::Inventory), Some((6, 7)));
    assert!(inventory.has_cell(3, 6));
    assert!(!inventory.has_cell(4, 6));
    // A tied bag is carried, but is no grid of its own.
    assert!(inventory.find(id(2)).is_some());
    assert!(inventory.placed(id(2)).is_none());
    assert_eq!(inventory.grid(Holder::Inside(id(2))), None);
}

#[test]
fn things_lie_across_the_rows_bags_add_but_not_off_the_last() {
    use ItemKind::{Ring, ShortSword, Stone};
    let mut inventory = with_pouch(&[Ring, Stone, ShortSword]);
    assert_eq!(put(&mut inventory, 3, 3, 6), Ok(()));
    assert_eq!(put(&mut inventory, 3, 4, 6), Err(MoveRefused::NoRoom));
    // A stone far heavier than a pouch takes lies in its cells: its limits are off.
    assert_eq!(put(&mut inventory, 4, 0, 5), Ok(()));
    // A 1 × 4 sword over the own rows and the pouch's, but not off the short row.
    assert_eq!(put(&mut inventory, 5, 4, 3), Err(MoveRefused::NoRoom));
    assert_eq!(put(&mut inventory, 5, 2, 3), Ok(()));
}

#[test]
fn nothing_goes_inside_a_tied_bag() {
    let mut inventory = with_pouch(&[ItemKind::Ring]);
    let into = inventory.check_move(id(3), Holder::Inside(id(2)), 0, 0, Rotation::None);
    assert_eq!(into, Err(MoveRefused::NoSuchGrid));
    assert!(
        inventory
            .first_place_for(id(3), Holder::Inside(id(2)), Rotation::None)
            .is_none()
    );
    assert_eq!(put(&mut inventory, 3, 0, 0), Ok(()));
}

#[test]
fn only_empty_bags_for_belts_go_on_free_slots() {
    use ItemKind::{BagOfHolding, Belt, Pouch, Ring, SmallBox};
    let mut inventory = carrying(&[Belt, Pouch, BagOfHolding, SmallBox, Ring]);
    assert_eq!(
        inventory.tie_bag(id(2), 0),
        Err(MoveRefused::NoSlot),
        "no belt"
    );
    inventory.wear(id(1)).unwrap();
    for bag in 3..=4 {
        assert_eq!(inventory.tie_bag(id(bag), 0), Err(MoveRefused::NotForBelt));
    }
    assert_eq!(inventory.tie_bag(id(5), 0), Err(MoveRefused::NotForBelt));
    assert_eq!(
        inventory.tie_bag(id(2), 1),
        Err(MoveRefused::NoSlot),
        "one slot"
    );
    inventory
        .move_item(id(5), Holder::Inside(id(2)), 0, 0, Rotation::None)
        .unwrap();
    assert_eq!(inventory.tie_bag(id(2), 0), Err(MoveRefused::NotEmpty));
    assert!(!inventory.is_tied(id(2)));
}

#[test]
fn a_bag_comes_off_only_with_room_at_the_grids_end() {
    use ItemKind::{Belt, Pouch, Ring};
    let mut inventory = carrying(&[Belt, Pouch, Pouch, Ring]);
    inventory.items[0].item.quality = Some(Quality::Uncommon);
    inventory.wear(id(1)).unwrap();
    inventory.tie_bag(id(2), 0).unwrap();
    inventory.tie_bag(id(3), 1).unwrap();
    assert_eq!(inventory.grid_cells(), 56);
    // The last cell: neither bag can come off, though the ring is in neither's
    // own cells in particular.
    put(&mut inventory, 4, 1, 9).unwrap();
    for bag in [2, 3] {
        assert_eq!(inventory.check_lift(id(bag)), Err(MoveRefused::BagInUse));
        assert_eq!(put(&mut inventory, bag, 0, 0), Err(MoveRefused::BagInUse));
        assert!(!inventory.to_bin(id(bag)));
        assert!(inventory.hand_over(id(bag)).is_none());
    }
    // Cell 39, the last left once one pouch is off: either may come off.
    put(&mut inventory, 4, 3, 6).unwrap();
    assert_eq!(put(&mut inventory, 2, 0, 0), Ok(()));
    assert!(!inventory.is_tied(id(2)));
    assert_eq!(inventory.grid_cells(), 40);
    // The other can't now: the ring is in the last 16.
    assert_eq!(inventory.check_lift(id(3)), Err(MoveRefused::BagInUse));
}

#[test]
fn a_bag_on_a_filled_slot_swaps_and_the_old_comes_out_empty() {
    use ItemKind::{Bag, Ring};
    let mut inventory = with_pouch(&[Bag, Ring]);
    assert_eq!(inventory.tie_bag(id(3), 0), Ok(()));
    assert_eq!(inventory.bag_slots.len(), 1);
    assert!(inventory.is_tied(id(3)));
    assert!(
        inventory.placed(id(2)).is_some(),
        "the pouch is in the grid"
    );
    assert_eq!(inventory.grid_cells(), 24 + 36);
    // Back to the smaller pouch: refused with something in the cells it would lose.
    put(&mut inventory, 4, 5, 9).unwrap();
    assert_eq!(inventory.tie_bag(id(2), 0), Err(MoveRefused::BagInUse));
    assert!(inventory.is_tied(id(3)), "nothing changed");
    put(&mut inventory, 4, 0, 6).unwrap();
    assert_eq!(inventory.tie_bag(id(2), 0), Ok(()));
    assert!(inventory.placed(id(3)).is_some());
    assert_eq!(inventory.grid_cells(), 40);
}

#[test]
fn a_belt_wont_come_off_or_get_worse_with_bags_beyond_its_slots() {
    use ItemKind::{Belt, Pouch};
    let mut inventory = carrying(&[Belt, Pouch, Pouch, Belt]);
    inventory.items[0].item.quality = Some(Quality::Uncommon);
    inventory.items[3].item.quality = Some(Quality::Poor);
    inventory.wear(id(1)).unwrap();
    inventory.tie_bag(id(2), 0).unwrap();
    inventory.tie_bag(id(3), 1).unwrap();
    // Two bags: neither off, nor a Poor belt in its place.
    assert_eq!(inventory.check_lift(id(1)), Err(MoveRefused::BeltInUse));
    assert_eq!(put(&mut inventory, 1, 0, 0), Err(MoveRefused::BeltInUse));
    assert!(!inventory.to_bin(id(1)));
    let swapped = inventory.wear_swapping(id(4), false, StackLimits::NONE);
    assert_eq!(swapped, Err(WearRefused::BagsTied));
    assert!(inventory.is_worn(id(1)));
    // With one bag, the Poor belt will do.
    put(&mut inventory, 3, 0, 0).unwrap();
    assert!(
        inventory
            .wear_swapping(id(4), false, StackLimits::NONE)
            .is_ok()
    );
    assert!(inventory.is_tied(id(2)));
    // And with none, it comes off.
    put(&mut inventory, 2, 4, 0).unwrap();
    assert_eq!(inventory.check_lift(id(4)), Ok(()));
}

#[test]
fn tied_bags_weigh_their_own_and_lighten_by_their_share() {
    let inventory = with_pouch(&[ItemKind::Ring]);
    let ring = inventory.find(id(3)).unwrap().total_weight();
    let belt = inventory.find(id(1)).unwrap().total_weight();
    let pouch = inventory.find(id(2)).unwrap().total_weight();
    assert_eq!(inventory.total_weight(), ring + belt + pouch);
    // A 50% bag giving 16 of 40 cells takes 20% off; two bags add their shares.
    assert_eq!(grams_off(1000, [(50, 16)].into_iter(), 40), 200);
    assert_eq!(grams_off(1000, [(50, 16), (25, 16)].into_iter(), 56), 214);
    assert_eq!(grams_off(1000, [(0, 16)].into_iter(), 40), 0);
    assert_eq!(grams_off(1000, [(100, 40)].into_iter(), 40), 1000);
}

#[test]
fn a_body_without_a_belt_drops_it_and_its_bags_then_what_no_longer_fits() {
    use ItemKind::{Ring, Stone};
    let mut inventory = with_pouch(&[Ring, Stone]);
    put(&mut inventory, 3, 3, 6).unwrap();
    let dropped = inventory.change_body(
        CreatureKind::Horse,
        StackLimits::NONE,
        &mut crate::id::stand_ins(),
    );
    assert!(inventory.bag_slots.is_empty());
    assert!(inventory.worn.is_empty());
    let ids: Vec<Id> = dropped.iter().map(|item| item.id).take(2).collect();
    assert_eq!(ids, [id(1), id(2)], "the belt, then its bag");
    assert!(inventory.all_on_cells());
    let kept = inventory.items.len();
    assert_eq!(dropped.len() + kept, 4);
}

#[test]
fn fewer_slots_than_bags_drop_the_last_bags_first() {
    use ItemKind::Pouch;
    let mut inventory = with_pouch(&[Pouch]);
    // As if tied to a better belt: the Common belt has one slot.
    let extra = inventory.take(id(3)).unwrap();
    inventory.bag_slots.push(extra);
    let dropped = inventory.change_body(
        CreatureKind::Human,
        StackLimits::NONE,
        &mut crate::id::stand_ins(),
    );
    let ids: Vec<Id> = dropped.iter().map(|item| item.id).collect();
    assert_eq!(ids, [id(3)]);
    assert!(inventory.is_tied(id(2)));
}

#[test]
fn only_pouches_bags_and_holders_go_on_belts() {
    let tied: Vec<ItemKind> = ItemKind::ALL
        .into_iter()
        .filter(|kind| kind.def().container.is_some_and(|inside| inside.on_belt))
        .collect();
    assert_eq!(
        tied,
        [
            ItemKind::Bag,
            ItemKind::HerbPouch,
            ItemKind::Keyring,
            ItemKind::Pouch
        ]
    );
    let bag = ItemKind::Bag.def().container.unwrap();
    let pouch = ItemKind::Pouch.def().container.unwrap();
    let backpack = ItemKind::Backpack.def().container.unwrap();
    let cells = |inside: ContainerDef| inside.width as u32 * inside.height as u32;
    assert!(cells(pouch) < cells(bag) && cells(bag) < cells(backpack));
}
