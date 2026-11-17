use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Holder, Inventory, Item, ItemKind, MoveRefused, Rotation, StackLimits};

use super::super::hold::Target;
use super::StrengthRoom;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A player with 40 cells wearing a belt (1), an apple (2) on the last cell.
fn wearing_a_belt() -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        ..Inventory::default()
    };
    inventory.start_sizing(40, StackLimits::NONE, &mut shared::id::stand_ins());
    inventory.worn.push(Item::new(id(1), ItemKind::Belt));
    let apple = Item::new(id(2), ItemKind::Apple);
    let at = (7, 4);
    let to = Holder::Inventory;
    inventory
        .add(apple, to, at.0, at.1, Rotation::None)
        .unwrap();
    inventory
}

#[test]
fn taking_off_what_the_grid_needs_is_refused_with_words() {
    let inventory = wearing_a_belt();
    // Without the belt, as if it gave Strength, the player would have 32.
    let room = StrengthRoom(vec![(id(1), 32)]);
    let off = Target::Cells {
        holder: Holder::Inventory,
        x: 0,
        y: 0,
        allowed: true,
    };
    assert_eq!(
        room.refused(&inventory, id(1), off),
        Some(MoveRefused::StrengthInUse)
    );
    assert_eq!(
        room.refused(&inventory, id(1), Target::Bin),
        Some(MoveRefused::StrengthInUse)
    );
    // Back on the body, or never leaving, nothing is refused.
    let on = Target::Body { allowed: true };
    assert_eq!(room.refused(&inventory, id(1), on), None);
    assert_eq!(room.refused(&inventory, id(2), Target::Bin), None);
    let words = super::super::bag_slots::explain_move(MoveRefused::StrengthInUse);
    assert!(words.contains("Empty the end of your inventory"));
}

#[test]
fn with_the_end_empty_it_comes_off() {
    let mut inventory = wearing_a_belt();
    inventory.items.clear();
    let room = StrengthRoom(vec![(id(1), 32)]);
    assert_eq!(room.refused(&inventory, id(1), Target::Bin), None);
}
