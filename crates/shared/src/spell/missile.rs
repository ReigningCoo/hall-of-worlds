//! Missile: flies to a fixed spot, hitting the first creature in its way that
//! the step's filters allow (passing through the rest), stopped by walls.
//! Faster for each count.

use crate::dungeon::Level;
use crate::spot::CELL_SPAN;

use super::filters::Filters;
use super::flight::path;
use super::hit::{Creature, Hit, Origin};
use super::toward::Toward;

/// The furthest a missile flies, in units: 8 cells. First guess.
pub const MISSILE_RANGE: u64 = 8 * CELL_SPAN as u64;

/// How fast a missile ×1 flies, in units a second: 16 cells, about four times
/// an ordinary walk (`energy::SITE_CELLS_PER_SECOND`). First guess.
pub const MISSILE_SPEED: u64 = 16 * CELL_SPAN as u64;

/// How much faster each count after the first makes it, in units a second:
/// half as fast again. First guess.
pub const MISSILE_FASTER: u64 = MISSILE_SPEED / 2;

/// Where a missile ends and how far it flew to get there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flight {
    /// Whom it hit, or the spot it stopped at (a wall, or the end of its way).
    pub hit: Hit,
    /// Units flown, for how long it takes (`flight_millis`).
    pub length: u64,
}

/// A missile ×`count`'s speed, in units a second.
pub fn missile_speed(count: u8) -> u64 {
    MISSILE_SPEED + MISSILE_FASTER * u64::from(count.max(1) - 1)
}

/// How long a missile ×`count` takes to fly `length` units, in milliseconds
/// (rounded up, so a missile is never early).
pub fn flight_millis(length: u64, count: u8) -> u64 {
    (length * 1000).div_ceil(missile_speed(count))
}

/// A missile from `origin` at `toward`, flown all at once: to the spot aimed
/// at (no further than `MISSILE_RANGE`, along the same way if that is
/// further), or straight ahead that far. It stops at the first wall and the
/// first creature `filters` allows whose circle it meets, never its origin's.
/// The server flies it a stretch at a time instead (`flight::Path`), so those
/// moving meanwhile are hit or missed as they go.
pub fn missile(
    level: &Level,
    origin: &Origin,
    toward: Toward,
    filters: Filters,
    creatures: &[Creature],
) -> Flight {
    let path = path(level, origin, toward, 1, filters, creatures);
    match path.meets((0, path.length), creatures, origin.creature) {
        Some((along, index)) => Flight {
            hit: path.hit(creatures, index),
            length: along,
        },
        None => Flight {
            hit: path.end(),
            length: path.length,
        },
    }
}
