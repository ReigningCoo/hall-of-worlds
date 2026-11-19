//! Which tree someone faces, to fell: as near and as straight ahead as
//! Collect Water's water (`potion::water_faced`).

use crate::dungeon::Level;
use crate::facing::{Heading, faced_cell};
use crate::potion::{COLLECT_REACH, FACING_HALF};
use crate::spot::Spot;
use crate::tree::TreeStage;

/// Whether a cell holds a tree that could be felled: a standing wild one or
/// fruit plant with a trunk (`Level::stands_tree`), or one felled and grown
/// back far enough (`TreeStage::can_fell`: young or half grown).
pub fn holds_tree(level: &Level, x: i64, y: i64) -> bool {
    level.stands_tree(x, y) || level.felled_at(x, y).is_some_and(TreeStage::can_fell)
}

/// The tree cell someone at `at` facing `heading` would fell, the nearest.
pub fn tree_faced(level: &Level, at: Spot, heading: Heading) -> Option<(i64, i64)> {
    faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, holds_tree)
}
