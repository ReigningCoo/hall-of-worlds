//! A player's own grid sized by Strength: the formula, growing, shrinking by
//! choice (refused while used) or not (moved, else dropped), short last rows,
//! tied bags after them, and changing body.

use super::*;
use crate::creature::CreatureKind;
use crate::id::{Id, stand_ins};

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A human's old 6 × 4 grid sized by Strength to `cells`, then carrying
/// `count` apples with ids from 1, each in the first room.
fn player(count: u128, cells: u32) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    let dropped = inventory.start_sizing(cells, StackLimits::NONE, &mut stand_ins());
    assert!(dropped.is_empty());
    for n in 1..=count {
        let apple = Item::new(id(n), ItemKind::Apple);
        inventory
            .give(apple, StackLimits::NONE, &mut stand_ins())
            .unwrap();
    }
    inventory
}

fn put(inventory: &mut Inventory, n: u128, x: u8, y: u8) {
    let to = Holder::Inventory;
    inventory
        .move_item(id(n), to, x, y, Rotation::None)
        .unwrap();
}

#[test]
fn cells_are_a_quarter_of_strength_and_at_least_32() {
    assert_eq!(cells_for_strength(0), 32);
    assert_eq!(cells_for_strength(100), 32);
    assert_eq!(cells_for_strength(131), 32);
    assert_eq!(cells_for_strength(132), 33);
    assert_eq!(cells_for_strength(200), 50);
    assert_eq!(cells_for_strength(u16::MAX), PLAYER_MAX_CELLS);
}

#[test]
fn an_old_grid_grows_to_8_wide_keeping_what_is_on_it() {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (6, 4),
        ..Inventory::default()
    };
    for n in 1..=3 {
        let apple = Item::new(id(n), ItemKind::Apple);
        inventory
            .give(apple, StackLimits::NONE, &mut stand_ins())
            .unwrap();
    }
    put(&mut inventory, 3, 5, 3);
    let dropped = inventory.start_sizing(32, StackLimits::NONE, &mut stand_ins());
    assert!(dropped.is_empty());
    assert!(inventory.sized_by_strength());
    assert_eq!(inventory.size, (8, 4));
    assert_eq!(inventory.own_cells(), 32);
    assert_eq!(inventory.items.len(), 3);
    assert_eq!(
        inventory.placed(id(3)).map(|(_, at)| (at.x, at.y)),
        Some((5, 3))
    );
    assert!(inventory.all_on_cells());
}

#[test]
fn a_mob_keeps_its_kind_s_grid() {
    let mut mob = Inventory {
        size: (4, 3),
        ..Inventory::default()
    };
    assert!(!mob.sized_by_strength());
    assert_eq!(mob.own_cells(), 12);
    assert!(
        mob.fit_own_cells(40, StackLimits::NONE, &mut stand_ins())
            .is_empty()
    );
    assert_eq!(mob.size, (4, 3));
    assert_eq!(mob.check_own_cells(1), Ok(()));
}

#[test]
fn the_last_row_may_be_short() {
    let mut inventory = player(1, 35);
    assert_eq!(inventory.size, (8, 5));
    assert_eq!(inventory.grid_cells(), 35);
    assert_eq!(inventory.grid_size(Holder::Inventory), Some((8, 5)));
    assert!(inventory.has_cell(2, 4));
    assert!(!inventory.has_cell(3, 4));
    let to = Holder::Inventory;
    let refused = inventory.move_item(id(1), to, 3, 4, Rotation::None);
    assert_eq!(refused, Err(MoveRefused::NoRoom));
    put(&mut inventory, 1, 2, 4);
}

#[test]
fn tied_bags_cells_follow_the_short_row_in_rows_of_8() {
    let mut inventory = player(0, 35);
    inventory.worn.push(Item::new(id(10), ItemKind::Belt));
    let pouch = Item::new(id(11), ItemKind::Pouch);
    inventory
        .give(pouch, StackLimits::NONE, &mut stand_ins())
        .unwrap();
    inventory.tie_bag(id(11), 0).unwrap();
    // 35 own cells and the pouch's 16: 51, in rows of 8.
    assert_eq!(inventory.grid_cells(), 51);
    assert_eq!(inventory.grid_size(Holder::Inventory), Some((8, 7)));
    assert!(inventory.has_cell(2, 6));
    assert!(!inventory.has_cell(3, 6));
}

#[test]
fn shrinking_by_choice_is_refused_while_the_cells_cut_hold_something() {
    let mut inventory = player(1, 40);
    assert_eq!(inventory.check_own_cells(32), Ok(()), "the end is empty");
    put(&mut inventory, 1, 7, 4);
    assert_eq!(
        inventory.check_own_cells(32),
        Err(MoveRefused::StrengthInUse)
    );
    assert_eq!(inventory.check_own_cells(40), Ok(()));
    assert_eq!(inventory.check_own_cells(50), Ok(()), "growing");
    // Checking changes nothing.
    assert_eq!(inventory.own_cells(), 40);
}

#[test]
fn growing_adds_cells_at_the_end() {
    let mut inventory = player(1, 32);
    let dropped = inventory.fit_own_cells(50, StackLimits::NONE, &mut stand_ins());
    assert!(dropped.is_empty());
    assert_eq!((inventory.own_cells(), inventory.size), (50, (8, 7)));
    put(&mut inventory, 1, 1, 6);
}

#[test]
fn a_loss_nobody_chose_moves_what_lies_on_cut_cells_to_free_room() {
    let mut inventory = player(1, 40);
    put(&mut inventory, 1, 7, 4);
    let dropped = inventory.fit_own_cells(32, StackLimits::NONE, &mut stand_ins());
    assert!(dropped.is_empty());
    assert_eq!(inventory.own_cells(), 32);
    assert_eq!(inventory.items.len(), 1);
    assert!(inventory.all_on_cells());
}

#[test]
fn a_loss_nobody_chose_drops_what_has_no_room() {
    // 40 apples fill 40 cells; at 32, eight have nowhere to go.
    let mut inventory = player(40, 40);
    let dropped = inventory.fit_own_cells(32, StackLimits::NONE, &mut stand_ins());
    assert_eq!(dropped.len(), 8);
    assert_eq!(inventory.items.len(), 32);
    assert!(inventory.all_on_cells());
}

#[test]
fn changing_body_keeps_a_player_s_grid() {
    let mut inventory = player(1, 40);
    let dropped = inventory.change_body(CreatureKind::Dwarf, StackLimits::NONE, &mut stand_ins());
    assert!(dropped.is_empty());
    assert_eq!((inventory.own_cells(), inventory.size), (40, (8, 5)));
    let mut mob = Inventory::default();
    mob.change_body(CreatureKind::Dwarf, StackLimits::NONE, &mut stand_ins());
    assert_eq!(mob.size, CreatureKind::Dwarf.def().inventory);
    assert!(!mob.sized_by_strength());
}
