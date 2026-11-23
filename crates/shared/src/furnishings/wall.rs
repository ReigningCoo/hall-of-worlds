//! Pieces with their backs to a wall: making one (`against_wall`, for
//! layouts placed by code) and finding where one could stand in a room.

use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::{Furniture, Material, Piece, footprint};

/// A piece whose back is to the wall on its `wall` side, its north-west cell
/// `at`: `(long, short)` its length and depth as `footprint` takes them (a
/// bed's 2 and its width; a rack's, chest's or fireplace's its length and
/// 1). It faces away from the wall, as a seat, a bench or a bed does.
pub fn against_wall(
    furniture: Furniture,
    at: (i64, i64),
    (long, short): (u8, u8),
    wall: Side,
    material: Material,
) -> Piece {
    let facing = wall.opposite();
    Piece {
        at,
        size: footprint(furniture, (long, short), facing),
        facing,
        material,
        furniture,
    }
}

/// Where a piece could stand against a wall: its north-west cell, its size
/// east–west and north–south, the way it faces, and its cells.
#[derive(Clone, Debug)]
pub(super) struct WallSpot {
    pub(super) at: (i64, i64),
    pub(super) size: (u8, u8),
    pub(super) facing: Side,
}

/// Each spot a piece `along` cells along a wall and `depth` cells out from
/// it could take with its back cells on `anchors`: every cell `free`, a
/// plain `Wall` behind each back cell, and open, unfurnished floor in front
/// of each front cell, so it can be used. In the order `hash` gives them.
pub(super) fn wall_spots(
    level: &Level,
    anchors: &[(i64, i64)],
    (along, depth): (u8, u8),
    free: impl Fn((i64, i64)) -> bool,
    hash: impl Fn(&[i64]) -> u64,
) -> Vec<WallSpot> {
    let mut found: Vec<(u64, (i64, i64), WallSpot)> = Vec::new();
    for &(x, y) in anchors {
        for wall in [Side::North, Side::East, Side::South, Side::West] {
            let (wx, wy) = wall.beside((0, 0));
            // Along the wall, east or south; out from it, away from the wall.
            let (ax, ay) = (wy.abs(), wx.abs());
            let cell = |i: i64, j: i64| (x + ax * i - wx * j, y + ay * i - wy * j);
            let (along, depth) = (i64::from(along), i64::from(depth));
            let cells: Vec<(i64, i64)> = (0..along)
                .flat_map(|i| (0..depth).map(move |j| (i, j)))
                .map(|(i, j)| cell(i, j))
                .collect();
            let backed = (0..along).all(|i| {
                let (cx, cy) = cell(i, 0);
                level.cell(cx + wx, cy + wy) == Cell::Wall
            });
            let front = (0..along).all(|i| {
                let (cx, cy) = cell(i, depth);
                level.cell(cx, cy) == Cell::Floor && level.furniture_at(cx, cy).is_none()
            });
            if !backed || !front || !cells.iter().all(|&c| free(c)) {
                continue;
            }
            let min_x = cells.iter().map(|c| c.0).min().unwrap_or(x);
            let min_y = cells.iter().map(|c| c.1).min().unwrap_or(y);
            let max_x = cells.iter().map(|c| c.0).max().unwrap_or(x);
            let max_y = cells.iter().map(|c| c.1).max().unwrap_or(y);
            let spot = WallSpot {
                at: (min_x, min_y),
                size: ((max_x - min_x + 1) as u8, (max_y - min_y + 1) as u8),
                facing: wall.opposite(),
            };
            found.push((hash(&[x, y, wall as i64]), (x, y), spot));
        }
    }
    found.sort_by_key(|(tie, at, _)| (*tie, *at));
    found.into_iter().map(|(_, _, spot)| spot).collect()
}

impl WallSpot {
    /// The piece of `furniture` in this spot, of `material`.
    pub(super) fn piece(&self, furniture: Furniture, material: Material) -> Piece {
        Piece {
            at: self.at,
            size: self.size,
            facing: self.facing,
            material,
            furniture,
        }
    }
}
