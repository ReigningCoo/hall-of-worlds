use shared::item::ItemKind;
use uuid::Uuid;

use super::*;

const LIMITS: StackLimits = StackLimits {
    grams_per_cell: 2000,
    most: 99,
};

fn id(n: u128) -> Id {
    Id(Uuid::from_u128(n))
}

/// These kinds in the inventory, in the first places they fit, ids from 1.
fn carrying(kinds: &[ItemKind]) -> Inventory {
    let mut inventory = Inventory::default();
    for (index, &kind) in kinds.iter().enumerate() {
        let item = Item::new(id(index as u128 + 1), kind);
        inventory
            .give(item, StackLimits::NONE, &mut shared::id::stand_ins())
            .expect("room");
    }
    inventory
}

#[test]
fn from_the_ground_onto_a_stack_first() {
    let inventory = carrying(&[ItemKind::Stone, ItemKind::CopperCoin]);
    let coin = Item::new(id(9), ItemKind::CopperCoin);
    let from = QuickFrom::Ground(coin);
    let (held, aim) = quick_target(&inventory, &from, &[], LIMITS, true)
        .unwrap()
        .unwrap();
    assert_eq!(held.id, id(9));
    assert_eq!(
        aim,
        Target::Stack {
            onto: id(2),
            allowed: true
        }
    );
}

#[test]
fn from_the_ground_to_the_first_free_place() {
    let inventory = carrying(&[ItemKind::Stone]);
    let ring = Item::new(id(9), ItemKind::Ring);
    let from = QuickFrom::Ground(ring.clone());
    let (held, aim) = quick_target(&inventory, &from, &[], LIMITS, true)
        .unwrap()
        .unwrap();
    let (x, y, rotation) = inventory
        .first_place_for_new(&ring, Holder::Inventory, Rotation::None)
        .unwrap();
    assert_eq!(held.rotation, rotation);
    assert_eq!(
        aim,
        Target::Cells {
            holder: Holder::Inventory,
            x: x as i32,
            y: y as i32,
            allowed: true
        }
    );
    assert_eq!(
        super::super::drop_move(&inventory, held, aim),
        Some((Holder::Inventory, x, y, rotation))
    );
}

#[test]
fn from_the_own_grid_to_the_ground_or_the_latest_bag() {
    let inventory = carrying(&[ItemKind::Pouch, ItemKind::Ring, ItemKind::Bag]);
    let from = QuickFrom::Carried(id(2));
    let (_, aim) = quick_target(&inventory, &from, &[], LIMITS, true)
        .unwrap()
        .unwrap();
    assert_eq!(aim, Target::Drop { allowed: true });
    assert_eq!(
        quick_target(&inventory, &from, &[], LIMITS, false),
        Some(Err(QuickRefused::NoGround))
    );
    // The last opened or raised is the latest.
    let (held, aim) = quick_target(&inventory, &from, &[id(1), id(3)], LIMITS, true)
        .unwrap()
        .unwrap();
    assert_eq!(
        super::super::drop_move(&inventory, held, aim),
        Some((Holder::Inside(id(3)), 0, 0, Rotation::None))
    );
}

#[test]
fn a_bag_never_goes_into_itself() {
    let inventory = carrying(&[ItemKind::Bag, ItemKind::Pouch]);
    let from = QuickFrom::Carried(id(2));
    // Its own panel is the latest: the one before it is used.
    let (held, aim) = quick_target(&inventory, &from, &[id(1), id(2)], LIMITS, true)
        .unwrap()
        .unwrap();
    assert_eq!(
        super::super::drop_move(&inventory, held, aim),
        Some((Holder::Inside(id(1)), 0, 0, Rotation::None))
    );
    // Only its own panel is open: onto the ground.
    let (_, aim) = quick_target(&inventory, &from, &[id(2)], LIMITS, true)
        .unwrap()
        .unwrap();
    assert_eq!(aim, Target::Drop { allowed: true });
}

#[test]
fn from_a_bag_to_the_own_grid() {
    let mut inventory = carrying(&[ItemKind::Pouch, ItemKind::Ring]);
    inventory
        .move_item(id(2), Holder::Inside(id(1)), 0, 0, Rotation::None)
        .unwrap();
    let from = QuickFrom::Carried(id(2));
    let (held, aim) = quick_target(&inventory, &from, &[id(1)], LIMITS, true)
        .unwrap()
        .unwrap();
    let (to, ..) = super::super::drop_move(&inventory, held, aim).unwrap();
    assert_eq!(to, Holder::Inventory);
}

#[test]
fn nothing_for_what_is_worn_or_in_the_bin() {
    let mut inventory = carrying(&[ItemKind::Ring]);
    assert!(inventory.to_bin(id(1)));
    let from = QuickFrom::Carried(id(1));
    assert_eq!(quick_target(&inventory, &from, &[], LIMITS, true), None);
}
