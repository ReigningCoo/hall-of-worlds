//! Which tree someone faces, to fell: as near and as straight ahead as
//! Collect Water's water (`potion::water_faced`).

use crate::dungeon::{Cell, Level};
use crate::facing::{Heading, faced_cell};
use crate::potion::{COLLECT_REACH, FACING_HALF};
use crate::spot::Spot;

/// Whether a cell holds a tree that could be felled: a wild one, or a fruit
/// plant with a trunk (`Level::has_trunk`).
pub fn holds_tree(level: &Level, x: i64, y: i64) -> bool {
    level.cell(x, y) == Cell::Tree || level.has_trunk(x, y)
}

/// The tree cell someone at `at` facing `heading` would fell, the nearest.
pub fn tree_faced(level: &Level, at: Spot, heading: Heading) -> Option<(i64, i64)> {
    faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, holds_tree)
}
