//! Walls and closed doors between a noise and a listener muffle it.

use crate::dungeon::Level;
use crate::spot::{CELL_SPAN, Spot};

/// What each wall cell between leaves of a noise's reach, per mille: a third.
pub const WALL_PER_MILLE: u32 = 333;

/// What each closed door between leaves, per mille: half.
pub const DOOR_PER_MILLE: u32 = 500;

/// How far `loudness` (units) reaches from `from` to `to` through what is
/// between: each wall cell and closed door crossed (not the cells either end)
/// takes its share.
pub fn muffled(level: &Level, from: Spot, to: Spot, loudness: u32) -> u32 {
    let mut left = u64::from(loudness);
    for (x, y) in cells_between(from, to) {
        if level.is_shut(x, y) {
            left = left * u64::from(DOOR_PER_MILLE) / 1000;
        } else if level.is_wall(x, y) {
            left = left * u64::from(WALL_PER_MILLE) / 1000;
        }
        if left == 0 {
            break;
        }
    }
    left as u32
}

/// The cells a straight line from `from` to `to` passes through, each once, in
/// order, leaving out the cells either end is in: checked every quarter cell.
fn cells_between(from: Spot, to: Spot) -> Vec<(i64, i64)> {
    let (ends, last) = (from.cell(), to.cell());
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    let steps = (dx.abs().max(dy.abs()) * 4 / CELL_SPAN).max(1);
    let mut cells: Vec<(i64, i64)> = Vec::new();
    for step in 1..steps {
        let at = Spot::new(from.x + dx * step / steps, from.y + dy * step / steps);
        let cell = at.cell();
        if cell != ends && cell != last && cells.last() != Some(&cell) {
            cells.push(cell);
        }
    }
    cells
}
