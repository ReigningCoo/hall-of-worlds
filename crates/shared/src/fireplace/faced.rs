//! The fireplace someone faces: within reach and facing it, as a bench.

use crate::dungeon::Level;
use crate::facing::{Heading, faced_cell};
use crate::furniture::{Furniture, Piece};
use crate::potion::{COLLECT_REACH, FACING_HALF};
use crate::spot::Spot;

/// The fireplace someone at `at` facing `heading` is at, as
/// `crafting_bench::bench_faced`: to light or feed it with Make Campfire, to
/// cook at it, or to use its log basket.
pub fn fireplace_faced(level: &Level, at: Spot, heading: Heading) -> Option<Piece> {
    let is_fireplace = |level: &Level, x: i64, y: i64| {
        (level.furniture_at(x, y))
            .is_some_and(|piece| matches!(piece.furniture, Furniture::Fireplace { .. }))
    };
    let (x, y) = faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, is_fireplace)?;
    level.furniture_at(x, y).copied()
}
