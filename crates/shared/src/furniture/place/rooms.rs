//! A level's rooms as furniture sees them: their floor cells, and which of
//! those may be furnished.

use std::collections::{BTreeMap, BTreeSet};

use crate::dungeon::{Cell, Level, RoomId};

/// A room's floor.
pub(crate) struct Room {
    pub(crate) id: RoomId,
    /// Its `Cell::Floor` cells, row by row.
    pub(crate) cells: Vec<(i64, i64)>,
}

impl Room {
    /// Its middle, in half cells (twice the mean of its cells).
    pub(super) fn middle_halves(&self) -> (i64, i64) {
        let count = self.cells.len().max(1) as i64;
        let (sx, sy) = (self.cells.iter()).fold((0, 0), |(sx, sy), &(x, y)| (sx + x, sy + y));
        (2 * sx / count, 2 * sy / count)
    }
}

/// Every room with floor on the level, smallest id first.
pub(crate) fn rooms_of(level: &Level) -> Vec<Room> {
    let mut rooms: BTreeMap<RoomId, Vec<(i64, i64)>> = BTreeMap::new();
    let (min_x, min_y, width, height) = level.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) == Cell::Floor
                && let Some(room) = level.room(x, y)
            {
                rooms.entry(room).or_default().push((x, y));
            }
        }
    }
    (rooms.into_iter())
        .map(|(id, cells)| Room { id, cells })
        .collect()
}

/// The cells furniture may stand on: room floor, but never beside a door
/// (the cell inside it), within a cell of stairs, or where anyone arrives.
pub(crate) fn usable_cells(
    level: &Level,
    rooms: &[Room],
    stairs: &[(i64, i64)],
    arrivals: &[(i64, i64)],
) -> BTreeSet<(i64, i64)> {
    let by_door = |(x, y): (i64, i64)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(dx, dy)| {
            matches!(
                level.cell(x + dx, y + dy),
                Cell::Door | Cell::SecretDoor | Cell::BuildingDoor
            )
        })
    };
    let by_stairs = |(x, y): (i64, i64)| {
        (stairs.iter()).any(|&(sx, sy)| (sx - x).abs() <= 1 && (sy - y).abs() <= 1)
    };
    (rooms.iter())
        .flat_map(|room| room.cells.iter().copied())
        .filter(|&cell| !by_door(cell) && !by_stairs(cell) && !arrivals.contains(&cell))
        .collect()
}
