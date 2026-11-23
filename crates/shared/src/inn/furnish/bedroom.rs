//! Upstairs' bedrooms, own rooms and the rest alike (The Way Inn plan, PR 4
//! for own rooms, PR 5 for the rest), furnished by their tier: a bed with
//! its head to a wall in the corner away from the door, the chests along
//! that wall beside it, the wardrobe on the side wall past the bed's foot.
//! The head wall is the north one unless the door is in it, then the south;
//! so nothing stands in front of a door.

use super::super::{InnRoom, RoomKind};
use super::OAK;
use crate::chest::{Chest, Wardrobe};
use crate::door::Side;
use crate::furnishings::against_wall;
use crate::furniture::{Bed, Furniture, Piece};
use crate::site::Rect;

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

/// Every bedroom's and own room's pieces among `rooms`.
pub(super) fn pieces(rooms: &[InnRoom]) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for room in rooms {
        let (RoomKind::Bedroom { tier } | RoomKind::OwnRoom { tier }) = room.kind else {
            continue;
        };
        let (Some(&rect), Some(door)) = (room.rects.first(), room.door) else {
            continue;
        };
        pieces.extend(bedroom(tier, rect, door));
    }
    pieces
}

/// A tier `tier` bedroom's pieces: its floor `rect`, its door `door`.
pub(in crate::inn) fn bedroom(tier: u8, rect: Rect, door: (i64, i64)) -> Vec<Piece> {
    let (bed, chests, wardrobe) = furniture_of(tier);
    let (x0, y0) = (rect.x, rect.y);
    let (x1, y1) = (x0 + rect.width - 1, y0 + rect.height - 1);
    // From the corner away from the door along the head wall, towards the
    // door's side: the west corner when the door is east of the room, or
    // in its north or south wall nearer its east end.
    let inside = (x0..=x1).contains(&door.0);
    let west_corner = door.0 > x1 || (inside && door.0 - x0 > x1 - door.0);
    let (corner, toward, side) = match west_corner {
        true => (x0, 1, Side::West),
        false => (x1, -1, Side::East),
    };
    let head = if door.1 < y0 {
        Side::South
    } else {
        Side::North
    };
    let length = i64::from(Bed::LENGTH);
    // The north cell of something `deep` cells north to south against the
    // head wall, and of the wardrobe `long` cells past the bed's foot.
    let against_head = |deep: i64| match head {
        Side::South => y1 - deep + 1,
        _ => y0,
    };
    let past_foot = |long: i64| match head {
        Side::South => y1 - length - long + 1,
        _ => y0 + length,
    };
    // The west cell of something `long` starting at `from` and running
    // `toward`.
    let west_of = |from: i64, long: i64| from.min(from + toward * (long - 1));
    let width = i64::from(bed.width());
    let mut pieces = Vec::new();
    let bed_at = (west_of(corner, width), against_head(length));
    let bed_size = (Bed::LENGTH, bed.width());
    pieces.push(against_wall(
        Furniture::Bed { bed },
        bed_at,
        bed_size,
        head,
        OAK,
    ));
    let mut next = corner + toward * width;
    for &chest in chests {
        let long = i64::from(chest.length());
        let at = (west_of(next, long), against_head(1));
        let size = (chest.length(), 1);
        pieces.push(against_wall(
            Furniture::Chest { chest },
            at,
            size,
            head,
            OAK,
        ));
        next += toward * long;
    }
    let long = i64::from(wardrobe.length());
    let at = (corner, past_foot(long));
    let size = (wardrobe.length(), 1);
    pieces.push(against_wall(
        Furniture::Wardrobe { wardrobe },
        at,
        size,
        side,
        OAK,
    ));
    pieces
}
