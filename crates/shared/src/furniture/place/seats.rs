//! Seats round a table: always along both long sides, sometimes at the ends
//! too; a bench along a side or a chair to each cell, matching the table.

use super::super::{Furniture, Material, Piece};
use super::placer::{Placer, RoomSet};
use super::tables::sides;
use crate::door::Side;
use crate::dungeon::RoomId;

/// In 100ths: how often a table has seats at its ends too.
const ENDS_PERCENT: u64 = 40;
/// In 100ths: how often a wooden chair has arms, at a table's end (its head)
/// and elsewhere.
const HEAD_ARMS_PERCENT: u64 = 60;
const ARMS_PERCENT: u64 = 15;

const ENDS: i64 = 0x656e;
const ARMS: i64 = 0x6172;

impl Placer<'_> {
    /// Seats round `table`, each kept only if it cuts nothing off.
    pub(super) fn seat_table(&mut self, table: &Piece, set: RoomSet, room: RoomId) {
        let along_x = table.size.0 >= table.size.1;
        let [first, second] = sides(table, along_x);
        let (towards_first, towards_second) = if along_x {
            (Side::South, Side::North)
        } else {
            (Side::East, Side::West)
        };
        self.seat_side(first, towards_first, set, false, room);
        self.seat_side(second, towards_second, set, false, room);
        let roll = self.roll(&[table.at.0, table.at.1, ENDS]);
        if roll % 100 < ENDS_PERCENT {
            let [first, second] = sides(table, !along_x);
            let (towards_first, towards_second) = if along_x {
                (Side::East, Side::West)
            } else {
                (Side::South, Side::North)
            };
            self.seat_side(first, towards_first, set, true, room);
            self.seat_side(second, towards_second, set, true, room);
        }
    }

    /// Seats along `cells` (a table's side, in a row), facing `facing`: a
    /// bench the side's length if the room has benches, else (or if the bench
    /// won't go) a chair to each cell. `head`: at a table's end.
    fn seat_side(
        &mut self,
        cells: Vec<(i64, i64)>,
        facing: Side,
        set: RoomSet,
        head: bool,
        room: RoomId,
    ) {
        let Some(&at) = cells.first() else {
            return;
        };
        if set.benches && cells.len() > 1 {
            let length = cells.len() as u8;
            let size = match facing {
                Side::North | Side::South => (length, 1),
                Side::East | Side::West => (1, length),
            };
            let bench = Piece {
                at,
                size,
                facing,
                material: set.material,
                furniture: Furniture::Bench { style: set.bench },
            };
            if self.try_place(bench) {
                return;
            }
        }
        for cell in cells {
            let chair = self.chair(cell, facing, set, head, room);
            self.try_place(chair);
        }
    }

    /// A chair of the room's set at `at`, facing `facing`: wooden ones
    /// sometimes have arms, more often at a table's head.
    pub(super) fn chair(
        &self,
        at: (i64, i64),
        facing: Side,
        set: RoomSet,
        head: bool,
        room: RoomId,
    ) -> Piece {
        let percent = if head {
            HEAD_ARMS_PERCENT
        } else {
            ARMS_PERCENT
        };
        let roll = self.roll(&[i64::from(room.0), at.0, at.1, ARMS]);
        let arms = matches!(set.material, Material::Wood(_)) && roll % 100 < percent;
        Piece {
            at,
            size: (1, 1),
            facing,
            material: set.material,
            furniture: Furniture::Chair {
                style: set.chair,
                arms,
            },
        }
    }
}
