//! Tables: about one per `Setting::cells_per_table` floor cells of a room, the
//! largest in the largest rooms, nearest the middle, each with seats.

use std::cmp::Reverse;

use super::super::{Furniture, Piece, TableStyle};
use super::placer::{Placer, RoomSet};
use crate::door::Side;

/// Tables' sizes, long side by short, largest first.
const SIZES: [(u8, u8); 5] = [(3, 2), (2, 2), (3, 1), (2, 1), (1, 1)];

/// Purpose numbers for this file's hashes.
const COUNT: i64 = 0x7461;
const SIZE: i64 = 0x737a;
const PLACE: i64 = 0x706c;

impl Placer<'_> {
    /// Tables and their seats in every room, largest room first.
    pub(super) fn place_tables(&mut self) {
        let mut order: Vec<usize> = (0..self.rooms.len()).collect();
        order.sort_by_key(|&r| (Reverse(self.rooms[r].cells.len()), self.rooms[r].id));
        for r in order {
            self.furnish_room(r);
        }
    }

    fn furnish_room(&mut self, r: usize) {
        let id = self.rooms[r].id;
        let cells = self.rooms[r].cells.len() as u64;
        let per = u64::from(self.setting.cells_per_table);
        let extra = self.roll(&[i64::from(id.0), COUNT]) % per < cells % per;
        let tables = cells / per + u64::from(extra);
        let set = self.set_of(id);
        for k in 0..tables {
            // Later tables are smaller, as if the room were shared out.
            let share = cells / (k + 1);
            let roll = self.roll(&[i64::from(id.0), SIZE, k as i64]);
            let start = match share {
                90.. => 0,
                60..90 => 1 + roll % 2,
                36..60 => 2 + roll % 2,
                _ => 3 + roll % 2,
            } as usize;
            let along_x = (roll >> 8).is_multiple_of(2);
            for &(long, short) in &SIZES[start..] {
                let size = if along_x {
                    (long, short)
                } else {
                    (short, long)
                };
                if let Some(table) = self.place_table(r, size, set) {
                    self.seat_table(&table, set, id);
                    break;
                }
            }
        }
    }

    /// A table of `size` in room `r` with room for seats along its long
    /// sides, nothing within two cells, as near the room's middle as it fits.
    fn place_table(&mut self, r: usize, size: (u8, u8), set: RoomSet) -> Option<Piece> {
        let style = if set.table.fits(size) {
            set.table
        } else {
            TableStyle::Joined
        };
        let facing = if size.1 > size.0 {
            Side::South
        } else {
            Side::East
        };
        let room = &self.rooms[r];
        let (mx, my) = room.middle_halves();
        let id = room.id;
        let (w, h) = (i64::from(size.0), i64::from(size.1));
        let mut spots: Vec<(i64, u64, (i64, i64))> = (room.cells.iter())
            .map(|&(x, y)| {
                let (dx, dy) = (2 * x + w - 1 - mx, 2 * y + h - 1 - my);
                let tie = self.roll(&[i64::from(id.0), PLACE, x, y]);
                (dx * dx + dy * dy, tie, (x, y))
            })
            .collect();
        spots.sort_unstable();
        for (_, _, at) in spots {
            let table = Piece {
                at,
                size,
                facing,
                material: set.material,
                furniture: Furniture::Table { style },
            };
            let sides_free = long_sides(&table)
                .iter()
                .flatten()
                .all(|&cell| self.is_free(cell));
            if sides_free && self.clear_round(&table, 2) && self.try_place(table) {
                return Some(table);
            }
        }
        None
    }
}

/// The cells along a table's two long sides (either pair for a square one):
/// north and south of it if it is wider than deep, else west and east.
pub(super) fn long_sides(table: &Piece) -> [Vec<(i64, i64)>; 2] {
    sides(table, table.size.0 >= table.size.1)
}

/// The cells along a table's north and south sides (`north_south`), or its
/// west and east sides.
pub(super) fn sides(table: &Piece, north_south: bool) -> [Vec<(i64, i64)>; 2] {
    let (x, y) = table.at;
    let (w, h) = (i64::from(table.size.0), i64::from(table.size.1));
    if north_south {
        [
            (x..x + w).map(|cx| (cx, y - 1)).collect(),
            (x..x + w).map(|cx| (cx, y + h)).collect(),
        ]
    } else {
        [
            (y..y + h).map(|cy| (x - 1, cy)).collect(),
            (y..y + h).map(|cy| (x + w, cy)).collect(),
        ]
    }
}
