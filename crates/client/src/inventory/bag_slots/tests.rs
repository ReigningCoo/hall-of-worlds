use shared::creature::CreatureKind;
use shared::item::{Holder, Item, ItemKind, Quality, Rotation, StackLimits};

use super::*;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A human with a 6 × 4 grid wearing a belt (1), carrying these kinds from 2.
fn belted(kinds: &[ItemKind]) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    let all = std::iter::once(ItemKind::Belt).chain(kinds.iter().copied());
    for (index, kind) in all.enumerate() {
        let item = Item::new(id(index as u128 + 1), kind);
        let stand_ins = &mut shared::id::stand_ins();
        inventory.give(item, StackLimits::NONE, stand_ins).unwrap();
    }
    inventory.wear(id(1)).unwrap();
    inventory
}

fn held(n: u128) -> Held {
    Held {
        id: id(n),
        rotation: Rotation::None,
        grip: (0, 0),
    }
}

#[test]
fn the_slots_are_named_bag0_to_bag5() {
    assert_eq!(bag_slot_named("bag0"), Some(0));
    assert_eq!(bag_slot_named("bag5"), Some(5));
    assert_eq!(bag_slot_named("bag6"), None);
    assert_eq!(bag_slot_named("bags"), None);
    assert_eq!(bag_slot_named("hand0"), None);
    assert_eq!(
        shared::item::belt_slots(Some(Quality::Artifact)),
        MOST_SLOTS
    );
}

#[test]
fn only_an_empty_belt_bag_can_be_tied_where_there_is_a_slot() {
    let inventory = belted(&[ItemKind::Bag, ItemKind::Ring]);
    let allowed = |n, slot| tie_aim(&inventory, held(n), slot);
    assert_eq!(
        allowed(2, 0),
        Target::BagSlot {
            slot: 0,
            allowed: true
        }
    );
    assert_eq!(
        allowed(2, 1),
        Target::BagSlot {
            slot: 1,
            allowed: false
        },
        "a common belt has one slot"
    );
    assert_eq!(
        allowed(3, 0),
        Target::BagSlot {
            slot: 0,
            allowed: false
        }
    );
}

#[test]
fn a_tied_bag_with_its_cells_in_use_says_why_it_stays() {
    let mut inventory = belted(&[ItemKind::Bag, ItemKind::Ring]);
    inventory.tie_bag(id(2), 0).unwrap();
    let (width, rows) = inventory.grid_size(Holder::Inventory).unwrap();
    // The ring at the very end of the grid, in the bag's cells.
    let last = inventory.grid_cells() - 1;
    let (x, y) = ((last % width as u32) as u8, (last / width as u32) as u8);
    assert_eq!(y, rows - 1);
    inventory
        .move_item(id(3), Holder::Inventory, x, y, Rotation::None)
        .unwrap();
    let off = Target::Cells {
        holder: Holder::Inventory,
        x: 0,
        y: 0,
        allowed: false,
    };
    assert_eq!(
        lift_refused(&inventory, id(2), off),
        Some(MoveRefused::BagInUse)
    );
    assert_eq!(
        lift_refused(&inventory, id(1), Target::Bin),
        Some(MoveRefused::BeltInUse)
    );
    // Over the clothes or another slot, nothing leaves.
    let slot = Target::BagSlot {
        slot: 0,
        allowed: true,
    };
    assert_eq!(lift_refused(&inventory, id(2), slot), None);
    assert!(!explain_move(MoveRefused::BagInUse).is_empty());
}
