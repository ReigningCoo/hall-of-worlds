//! The Hall's hidden ways into The Way Inn (`shared::inn`): stepping off
//! either edge of the walkway halfway between two gates on that side. Nothing
//! is drawn there and the tile is void like any other: only stepping into it
//! is different (The Way Inn plan, PR 2).

use super::{GATE_SPACING, Side, WALKWAY_ROWS, wrap_x};

/// How far along from a gate the way in is: halfway to the next.
pub const INN_WAY_OFFSET: i64 = GATE_SPACING / 2;

/// Whether `(x, y)` is a way into the inn, and on which side of the walkway:
/// a void tile in a gate's row (just beyond the walkway's edge), halfway
/// between two gates.
pub fn inn_way(x: i64, y: i64) -> Option<Side> {
    let side = match y {
        -1 => Side::North,
        WALKWAY_ROWS => Side::South,
        _ => return None,
    };
    (wrap_x(x) % GATE_SPACING == INN_WAY_OFFSET).then_some(side)
}

/// The walkway tile in front of a way in, where whoever steps through it came
/// from (and goes back to on leaving the inn).
pub fn inn_way_front(x: i64, side: Side) -> (i64, i64) {
    match side {
        Side::North => (x, 0),
        Side::South => (x, WALKWAY_ROWS - 1),
    }
}

/// The way in that the walkway tile `(x, y)` stands in front of, if any: the
/// tile beyond its edge, and on which side. Walking off the walkway from here
/// goes into the inn.
pub fn inn_way_behind(x: i64, y: i64) -> Option<((i64, i64), Side)> {
    let side = match y {
        0 => Side::North,
        y if y == WALKWAY_ROWS - 1 => Side::South,
        _ => return None,
    };
    let way = match side {
        Side::North => (x, -1),
        Side::South => (x, WALKWAY_ROWS),
    };
    inn_way(way.0, way.1).map(|_| (way, side))
}
