//! Where furniture may be put by hand (dev tools): the server checks Place
//! Furniture's and Create Crafting Bench's pieces by these, and the client
//! shows Create Crafting Bench's ghost red where they say no.

use crate::dungeon::{Cell, Level};

/// Whether furniture may be put on a cell: floor or open ground, not a
/// doorway, a gate, water or a bridge.
pub fn takes_furniture(cell: Cell) -> bool {
    cell.is_walkable()
        && !matches!(
            cell,
            Cell::Door | Cell::FieldGate | Cell::WorldGate | Cell::Shallows | Cell::Bridge
        )
}

/// Whether a crafting bench made by hand may cover `(x, y)`: on the level,
/// a cell that takes furniture, with no other piece on it. Stairs are left
/// to the caller, as the server and client each keep their own.
pub fn free_for_bench(level: &Level, (x, y): (i64, i64)) -> bool {
    level.contains(x, y) && takes_furniture(level.cell(x, y)) && level.furniture_at(x, y).is_none()
}
