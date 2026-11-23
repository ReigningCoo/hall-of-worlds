//! Lines across a level for missiles and beams, in whole units: how far one
//! gets before a wall, and where along it a creature's circle is first met.

use crate::dungeon::{Level, spots_are_clear};
use crate::facing::{Heading, UNIT};
use crate::spot::{Spot, root};

/// The spot `length` units from `from` going `heading`.
pub(super) fn ahead(from: Spot, heading: Heading, length: u64) -> Spot {
    let (dx, dy) = heading.unit();
    let length = length as i64;
    from.offset(dx * length / UNIT, dy * length / UNIT)
}

/// How far along the line from `from` to `to` stays clear of walls, as the
/// spot where it stops: `to` itself when nothing is in the way. A line clear
/// to a point is clear to every point before it, so this halves its way there.
pub(super) fn wall_stop(level: &Level, from: Spot, to: Spot) -> Spot {
    if spots_are_clear(level, from, to) {
        return to;
    }
    let length = from.distance(to) as i64;
    let at = |along: i64| {
        from.offset(
            (to.x - from.x) * along / length.max(1),
            (to.y - from.y) * along / length.max(1),
        )
    };
    // `low` is always clear, `high` never.
    let (mut low, mut high) = (0i64, length);
    while high - low > 1 {
        let middle = (low + high) / 2;
        if spots_are_clear(level, from, at(middle)) {
            low = middle;
        } else {
            high = middle;
        }
    }
    at(low)
}

/// How far along the line from `from` to `to` (in units from `from`) a point
/// moving along it first comes within `radius` of `centre`; `None` if it never
/// does before `to`. One already inside meets it at 0.
pub(super) fn entry_along(from: Spot, to: Spot, centre: Spot, radius: u64) -> Option<u64> {
    let (dx, dy) = (i128::from(to.x - from.x), i128::from(to.y - from.y));
    let (cx, cy) = (i128::from(centre.x - from.x), i128::from(centre.y - from.y));
    let radius = i128::from(radius);
    if cx * cx + cy * cy <= radius * radius {
        return Some(0);
    }
    let length_squared = dx * dx + dy * dy;
    if length_squared == 0 {
        return None;
    }
    let length = root(length_squared as u128) as i128;
    // Along the line to the nearest point, and the square of the distance
    // across from it, both in units.
    let along = (cx * dx + cy * dy) / length.max(1);
    let across_squared = (cx * cx + cy * cy - along * along).max(0);
    if along <= 0 || across_squared > radius * radius {
        return None;
    }
    let back = root((radius * radius - across_squared) as u128) as i128;
    let entry = along - back;
    (entry <= length).then_some(entry.max(0) as u64)
}
