//! Bedrooms: rooms of a fitting size, more often upstairs, each with a bed
//! (its head against a wall), a chest and a wardrobe, all of one wood.

use super::Setting;
use super::furnisher::Furnisher;
use crate::chest::{Chest, Wardrobe};
use crate::dungeon::RoomId;
use crate::furniture::{Bed, Furniture};

/// The fewest and most cells a bedroom has.
const FEWEST_CELLS: usize = 6;
const MOST_CELLS: usize = 80;
/// From this many cells a bedroom may have a double bed.
const DOUBLE_CELLS: usize = 16;
/// Purpose numbers for the hashes.
const IS: i64 = 20;
const BED: i64 = 21;
const CHEST: i64 = 22;
const WARDROBE: i64 = 23;
const SIZES: i64 = 24;

/// Makes bedrooms of the level's rooms (not those in `taken`: the great
/// hall and the cellar), returning those that got a bed.
pub(super) fn place_bedrooms(
    furnisher: &mut Furnisher,
    setting: &Setting,
    taken: &[RoomId],
) -> Vec<RoomId> {
    let upstairs = furnisher.floor.index > 0 && !furnisher.floor.underground;
    let percent = match upstairs {
        true => setting.bedroom_upstairs_percent,
        false => setting.bedroom_downstairs_percent,
    };
    let rooms: Vec<(RoomId, Vec<(i64, i64)>)> = (furnisher.rooms.iter())
        .filter(|room| !taken.contains(&room.id))
        .filter(|room| (FEWEST_CELLS..=MOST_CELLS).contains(&room.cells.len()))
        .map(|room| (room.id, room.cells.clone()))
        .collect();
    let mut bedrooms = Vec::new();
    for (id, cells) in rooms {
        let room = i64::from(id.0);
        if furnisher.roll(&[room, IS]) % 100 >= percent {
            continue;
        }
        if furnish_bedroom(furnisher, room, &cells) {
            bedrooms.push(id);
        }
    }
    bedrooms
}

/// A bed, then a chest and a wardrobe where they fit. Whether the bed went in.
fn furnish_bedroom(furnisher: &mut Furnisher, room: i64, cells: &[(i64, i64)]) -> bool {
    let sizes = furnisher.roll(&[room, SIZES]);
    let bed = match cells.len() >= DOUBLE_CELLS && sizes.is_multiple_of(2) {
        true => Bed::Double,
        false => Bed::Single,
    };
    let wood = furnisher.wood(&[room]);
    let free = |cell| furnisher.is_free(cell);
    let spots = furnisher.spots(cells, (bed.width(), Bed::LENGTH), free, BED + room * 8);
    if furnisher
        .place_first(spots, Furniture::Bed { bed }, wood)
        .is_none()
    {
        return false;
    }
    let chest = Chest::ALL[(sizes / 2 % 2) as usize];
    let wardrobe = Wardrobe::ALL[(sizes / 4 % 2) as usize];
    let free = |cell| furnisher.is_free(cell);
    let spots = furnisher.spots(cells, (chest.length(), 1), free, CHEST + room * 8);
    furnisher.place_first(spots, Furniture::Chest { chest }, wood);
    let free = |cell| furnisher.is_free(cell);
    let spots = furnisher.spots(cells, (wardrobe.length(), 1), free, WARDROBE + room * 8);
    furnisher.place_first(spots, Furniture::Wardrobe { wardrobe }, wood);
    true
}
