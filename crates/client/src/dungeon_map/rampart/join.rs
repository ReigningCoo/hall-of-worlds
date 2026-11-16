//! Where a wall round a site meets a building's wall on its ring (a castle's
//! tower): the building's wall doesn't reach out along the middle of the
//! wall's cell, and the wall's run comes on into the building's cell instead,
//! up to its wall's face.

use shared::dungeon::{Level, WallShape};
use shared::site::WallLook;

use super::SIDES;

/// Whether the run of `wall` at `run` (not a gate) meets a building's wall at
/// `other`, the cell beside it on the wall's ring.
pub(super) fn meets(wall: &WallLook, level: &Level, run: (i64, i64), other: (i64, i64)) -> bool {
    wall.has(run.0, run.1)
        && !wall.is_gate(run.0, run.1)
        && wall.on_ring(other.0, other.1)
        && !wall.has(other.0, other.1)
        && !wall.gaps.contains(&other)
        && level.is_wall(other.0, other.1)
}

/// The shape of a building's wall at `(x, y)` without its joins towards
/// walls round a site that meet it there.
pub fn without_ramparts(
    walls: &[WallLook],
    level: &Level,
    (x, y): (i64, i64),
    shape: WallShape,
) -> WallShape {
    let met = SIDES
        .map(|(dx, dy)| (walls.iter()).any(|wall| meets(wall, level, (x + dx, y + dy), (x, y))));
    WallShape {
        north: shape.north && !met[0],
        east: shape.east && !met[1],
        south: shape.south && !met[2],
        west: shape.west && !met[3],
    }
}
