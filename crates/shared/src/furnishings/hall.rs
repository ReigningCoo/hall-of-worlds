//! The great hall: the ground floor's largest room, with a fireplace against
//! one of its walls (an inglenook in the largest halls).

use super::Setting;
use super::furnisher::Furnisher;
use crate::dungeon::RoomId;
use crate::fireplace::Fireplace;
use crate::furniture::Furniture;

/// The fewest cells a room needs to be a great hall.
const HALL_CELLS: usize = 20;
/// From this many cells a hall's fireplace is 2 long, and from this many an
/// inglenook (3).
const WIDE_CELLS: usize = 40;
const INGLENOOK_CELLS: usize = 80;
/// Purpose numbers for the hashes.
const HAS: i64 = 10;
const SPOT: i64 = 11;

/// The great hall, if the level has a room large enough, with a fireplace
/// if it rolls one and one fits (the longest that does).
pub(super) fn place_fireplace(furnisher: &mut Furnisher, setting: &Setting) -> Option<RoomId> {
    let (id, cells) = (furnisher.rooms.iter())
        .filter(|room| room.cells.len() >= HALL_CELLS)
        .max_by_key(|room| (room.cells.len(), std::cmp::Reverse(room.id)))
        .map(|room| (room.id, room.cells.clone()))?;
    if furnisher.roll(&[HAS]) % 100 >= setting.fireplace_percent {
        return Some(id);
    }
    let most = match cells.len() {
        n if n >= INGLENOOK_CELLS => 3,
        n if n >= WIDE_CELLS => 2,
        _ => 1,
    };
    let stone = furnisher.stone(&[SPOT]);
    for length in (1..=most).rev() {
        let free = |cell| furnisher.is_free(cell);
        let spots = furnisher.spots(&cells, (length, 1), free, SPOT);
        let fireplace = Furniture::Fireplace {
            fireplace: Fireplace { length },
        };
        if furnisher.place_first(spots, fireplace, stone).is_some() {
            break;
        }
    }
    Some(id)
}
