//! Loose seats: chairs and benches against a room's walls, back to the wall,
//! about one per `Setting::cells_per_loose` floor cells.

use super::super::{Furniture, Piece};
use super::placer::Placer;
use crate::door::Side;
use crate::dungeon::Cell;

/// The longest bench against a wall.
const LONGEST_BENCH: i64 = 3;

const COUNT: i64 = 0x6c63;
const SPOT: i64 = 0x6c73;
const KIND: i64 = 0x6c6b;

/// Each side a wall can be behind a seat: where the wall is from the seat,
/// and the way the seat then faces.
const BACKS: [((i64, i64), Side); 4] = [
    ((0, -1), Side::South),
    ((1, 0), Side::West),
    ((0, 1), Side::North),
    ((-1, 0), Side::East),
];

impl Placer<'_> {
    /// Loose seats in every room, smallest id first.
    pub(super) fn place_loose(&mut self) {
        for r in 0..self.rooms.len() {
            self.loose_in(r);
        }
    }

    fn loose_in(&mut self, r: usize) {
        let id = self.rooms[r].id;
        let cells = self.rooms[r].cells.len() as u64;
        let per = u64::from(self.setting.cells_per_loose);
        let extra = self.roll(&[i64::from(id.0), COUNT]) % per < cells % per;
        let wanted = cells / per + u64::from(extra);
        if wanted == 0 {
            return;
        }
        let set = self.set_of(id);
        let mut spots: Vec<(u64, (i64, i64), usize)> = Vec::new();
        for &cell in &self.rooms[r].cells {
            for (back, &((dx, dy), _)) in BACKS.iter().enumerate() {
                if self.level.cell(cell.0 + dx, cell.1 + dy) == Cell::Wall {
                    let tie = self.roll(&[cell.0, cell.1, back as i64, SPOT]);
                    spots.push((tie, cell, back));
                }
            }
        }
        spots.sort_unstable();
        let mut placed = 0;
        for (tie, (x, y), back) in spots {
            if placed == wanted {
                break;
            }
            let ((dx, dy), facing) = BACKS[back];
            let bench = set.benches == (tie >> 32).is_multiple_of(2);
            let piece = if bench {
                // Along the wall, east or south, while the wall goes on.
                let (ax, ay) = (dy.abs(), dx.abs());
                let most = 1 + (self.roll(&[x, y, KIND]) % LONGEST_BENCH as u64) as i64;
                let length = (1..most)
                    .take_while(|&i| {
                        let (cx, cy) = (x + ax * i, y + ay * i);
                        self.is_free((cx, cy)) && self.level.cell(cx + dx, cy + dy) == Cell::Wall
                    })
                    .count() as i64
                    + 1;
                let size = (1 + ax * (length - 1), 1 + ay * (length - 1));
                Piece {
                    at: (x, y),
                    size: (size.0 as u8, size.1 as u8),
                    facing,
                    material: set.material,
                    furniture: Furniture::Bench { style: set.bench },
                }
            } else {
                self.chair((x, y), facing, set, false, id)
            };
            if self.clear_round(&piece, 1) && self.try_place(piece) {
                placed += 1;
            }
        }
    }
}
