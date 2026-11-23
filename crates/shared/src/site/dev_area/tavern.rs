//! The Tavern Dev Area (tavern furniture plan PR 8): a stone building on the
//! area's open ground holding a whole tavern (a taproom with its bar, the
//! back-bar's wine, spirit and cask racks, an inglenook, booths, games
//! tables and tables, decorations and lights) and three inn bedrooms off it,
//! to see the tavern furniture together. The owner copies from it into the
//! starting area off the Hall later. Fixed, so every seed makes the same one;
//! the server stocks its racks and lights its fire as generated ones.
//!
//! In the building's own cells (`Shell`): the taproom is x 0 to 17, y 0 to
//! 14, the bedrooms x 19 to 25, one above another, each with a door into it.

use super::DevArea;
use super::layout::{clear_round, walled, with_stairs};
use super::tavern_decor::decorate;
use super::tavern_furniture::furnish;
use crate::dungeon::{Cell, Level, RoomId};
use crate::site::SiteLevel;

/// The smallest area it fits in.
const LEAST_SIZE: i64 = 32;
/// Its rooms' ids: the taproom, then the bedrooms from the north.
const TAPROOM: u16 = 1;
const BEDROOMS: [(u16, i64, i64); 3] = [(2, 0, 3), (3, 5, 9), (4, 11, 14)];
/// Its doors: the way in on the west, then each bedroom's.
const DOORS: [(i64, i64); 4] = [(-1, 10), (18, 2), (18, 7), (18, 12)];

/// Where the building's own cells are on the level.
#[derive(Clone, Copy)]
pub(super) struct Shell {
    origin: (i64, i64),
}

impl Shell {
    /// The level cell of the building's cell `(x, y)`.
    pub(super) fn at(self, (x, y): (i64, i64)) -> (i64, i64) {
        (self.origin.0 + x, self.origin.1 + y)
    }
}

/// The area: at least 32 cells a side, the tavern in its middle and the way
/// out on the west side as in every plain area.
pub(super) fn make(area: &DevArea) -> SiteLevel {
    let size = i64::from(area.size).max(LEAST_SIZE);
    let mut level = walled((0, 0), size);
    let shell = Shell {
        origin: (size / 2 - 12, size / 2 - 8),
    };
    build(&mut level, shell);
    let stairs = (1, size / 2);
    clear_round(&mut level, stairs);
    let mut made = with_stairs(level, stairs);
    furnish(&mut made.level, shell);
    decorate(&mut made.level, shell);
    made
}

/// Its walls, floors, rooms and doors.
fn build(level: &mut Level, shell: Shell) {
    for y in -1..=15 {
        for x in -1..=26 {
            let (lx, ly) = shell.at((x, y));
            let outside = x == -1 || x == 26 || y == -1 || y == 15;
            let partition = x == 18 || (x > 18 && (y == 4 || y == 10));
            if outside || partition {
                level.set(lx, ly, Cell::Wall);
                level.set_room(lx, ly, None);
                continue;
            }
            level.set(lx, ly, Cell::Floor);
            let room = match x < 18 {
                true => TAPROOM,
                false => (BEDROOMS.iter())
                    .find(|&&(_, from, to)| (from..=to).contains(&y))
                    .map_or(TAPROOM, |&(id, _, _)| id),
            };
            level.set_room(lx, ly, Some(RoomId(room)));
        }
    }
    for door in DOORS {
        let (x, y) = shell.at(door);
        level.set(x, y, Cell::Door);
    }
}
