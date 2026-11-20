//! The bench someone is at: within reach and facing it, as Collect Water's
//! water and Fell Tree's tree.

use crate::dungeon::Level;
use crate::facing::{Heading, faced_cell};
use crate::furniture::{Furniture, Piece};
use crate::potion::{COLLECT_REACH, FACING_HALF};
use crate::spot::Spot;

/// The crafting bench someone at `at` facing `heading` is at: any of its
/// cells within `COLLECT_REACH` and `FACING_HALF` of straight ahead, the
/// nearest if several.
pub fn bench_faced(level: &Level, at: Spot, heading: Heading) -> Option<Piece> {
    let is_bench = |level: &Level, x: i64, y: i64| {
        (level.furniture_at(x, y))
            .is_some_and(|piece| matches!(piece.furniture, Furniture::CraftingBench { .. }))
    };
    let (x, y) = faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, is_bench)?;
    level.furniture_at(x, y).copied()
}
