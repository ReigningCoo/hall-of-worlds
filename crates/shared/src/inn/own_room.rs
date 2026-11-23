//! Players' own rooms upstairs (The Way Inn plan, PR 4): one a tier, behind
//! its own door. Everyone who goes in has their own copy of the room at the
//! same time: what lies in its chests and wardrobe and on its floor is theirs
//! alone, kept with their character on every world (the server's), and
//! nobody else is seen in it.

use super::UPSTAIRS;
use super::upstairs;
use crate::chest::{Chest, Wardrobe};
use crate::door::Side;
use crate::dungeon::Level;
use crate::furnishings::against_wall;
use crate::furniture::{Bed, Furniture, Material, Wood};
use crate::site::Rect;

/// One tier's own room: its floor and its door.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnRoom {
    pub tier: u8,
    pub rect: Rect,
    pub door: (i64, i64),
}

/// Every tier's own room, smallest tier first (`RoomKind::OwnRoom`).
pub fn own_rooms() -> Vec<OwnRoom> {
    (upstairs::own_rooms().into_iter())
        .map(|(tier, rect, door)| OwnRoom { tier, rect, door })
        .collect()
}

/// The tier of the own room `(x, y)` on floor `index` is in, if it is in one
/// (its door is not).
pub fn own_room_at(index: u32, (x, y): (i64, i64)) -> Option<u8> {
    if index != UPSTAIRS {
        return None;
    }
    (own_rooms().into_iter())
        .find(|room| room.rect.contains(x, y))
        .map(|room| room.tier)
}

/// What each tier's room has: its bed, its chests and its wardrobe; larger
/// tiers more.
fn furniture_of(tier: u8) -> (Bed, &'static [Chest], Wardrobe) {
    match tier {
        1 => (Bed::Single, &[Chest::Small], Wardrobe::Narrow),
        2 => (Bed::Single, &[Chest::Large], Wardrobe::Narrow),
        3 => (Bed::Double, &[Chest::Large, Chest::Small], Wardrobe::Wide),
        _ => (Bed::Double, &[Chest::Large, Chest::Large], Wardrobe::Wide),
    }
}

/// Puts each own room's furniture on upstairs' level: the bed's head against
/// the north wall in the east corner, the chests along that wall west of it,
/// the wardrobe against the east wall below the bed. Doors are in the west
/// and south walls, so nothing stands in front of one.
pub(super) fn furnish(level: &mut Level) {
    let wood = Material::Wood(Wood::Oak);
    for room in own_rooms() {
        let (bed, chests, wardrobe) = furniture_of(room.tier);
        let (y0, x1) = (room.rect.y, room.rect.x + room.rect.width - 1);
        let width = i64::from(bed.width());
        let bed_at = (x1 - width + 1, y0);
        let size = (Bed::LENGTH, bed.width());
        level.place_furniture(against_wall(
            Furniture::Bed { bed },
            bed_at,
            size,
            Side::North,
            wood,
        ));
        let mut east = x1 - width;
        for &chest in chests {
            let length = chest.length();
            let at = (east - i64::from(length) + 1, y0);
            let piece = against_wall(
                Furniture::Chest { chest },
                at,
                (length, 1),
                Side::North,
                wood,
            );
            level.place_furniture(piece);
            east -= i64::from(length);
        }
        let size = (wardrobe.length(), 1);
        let at = (x1, y0 + i64::from(Bed::LENGTH));
        let piece = against_wall(Furniture::Wardrobe { wardrobe }, at, size, Side::East, wood);
        level.place_furniture(piece);
    }
}

#[cfg(test)]
mod tests;
