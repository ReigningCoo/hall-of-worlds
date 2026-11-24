//! Teleport's rule (magic plan PR 13): a character a Teleport lies on
//! (`Sheet::may_teleport`) makes their next move straight to a cell they
//! chose, at once: within `TELEPORT_RANGE`, in a clear line from where they
//! stand (so in sight), and with room for them to stand there. The client
//! asks only what this allows; the server checks it again and also that the
//! cell is one the player has seen.

use crate::clearance::check_segment;
use crate::creature::Size;
use crate::dungeon::{Level, spots_are_clear};
use crate::spot::{CELL_SPAN, Spot};

/// The furthest a Teleport goes, in cells. First guess.
pub const TELEPORT_RANGE: i64 = 10;

/// Whether a creature of `size` at `from` may teleport to `to`.
pub fn may_teleport_to(level: &Level, size: Size, from: Spot, to: Spot) -> bool {
    let range = (TELEPORT_RANGE * CELL_SPAN) as u64;
    from != to
        && from.distance(to) <= range
        && spots_are_clear(level, from, to)
        && check_segment(level, size, to, to).is_some()
}
