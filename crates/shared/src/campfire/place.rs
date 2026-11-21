//! Where a campfire can be lit, and which one someone faces to feed: as near
//! and as straight ahead as Collect Water's water (`potion::water_faced`).

use crate::dungeon::{Cell, Level};
use crate::facing::{Heading, faced_cell};
use crate::potion::{COLLECT_REACH, FACING_HALF};
use crate::spot::Spot;

/// Whether a fire could be lit on `(x, y)`: bare ground, grass, a floor or a
/// road surface (not boards, a field, a garden, covered ground or water),
/// with no furniture or tree on it. The server also keeps fires off stairs
/// and other fires.
pub fn can_hold_fire(level: &Level, x: i64, y: i64) -> bool {
    let ground = matches!(
        level.cell(x, y),
        Cell::Floor
            | Cell::Ground
            | Cell::Grass
            | Cell::DirtTrack
            | Cell::GravelTrack
            | Cell::Cobblestone
            | Cell::Flagstones
            | Cell::DressedStone
    );
    ground && level.furniture_at(x, y).is_none() && !level.stands_tree(x, y)
}

/// The fire someone at `at` facing `heading` would feed: the nearest faced
/// cell `has_fire` says holds a lit one.
pub fn campfire_faced(
    level: &Level,
    at: Spot,
    heading: Heading,
    has_fire: impl Fn(i64, i64) -> bool,
) -> Option<(i64, i64)> {
    faced_cell(level, at, heading, COLLECT_REACH, FACING_HALF, |_, x, y| {
        has_fire(x, y)
    })
}

/// Where someone at `at` facing `heading` would light a new fire: the nearest
/// faced cell that `can_hold_fire` and `free` (no stairs, no fire) allow.
pub fn place_faced(
    level: &Level,
    at: Spot,
    heading: Heading,
    free: impl Fn(i64, i64) -> bool,
) -> Option<(i64, i64)> {
    faced_cell(
        level,
        at,
        heading,
        COLLECT_REACH,
        FACING_HALF,
        |level, x, y| can_hold_fire(level, x, y) && free(x, y),
    )
}

/// Whether Make Campfire has something to do for someone at `at` facing
/// `heading`: a hearth faced to light or feed (`crafting_bench::
/// hearth_faced`), a fire faced to feed, or ground faced to light one on
/// (the client's guess: the server also keeps fires off stairs, and knows
/// which hearths are always lit).
pub fn can_make_fire(level: &Level, at: Spot, heading: Heading) -> bool {
    let burning = |x: i64, y: i64| level.campfire_at(x, y).is_some();
    crate::crafting_bench::hearth_faced(level, at, heading).is_some()
        || campfire_faced(level, at, heading, burning).is_some()
        || place_faced(level, at, heading, |x, y| !burning(x, y)).is_some()
}
