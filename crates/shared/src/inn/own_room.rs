//! Players' own rooms upstairs (The Way Inn plan, PR 4): one a tier, behind
//! its own door. Everyone who goes in has their own copy of the room at the
//! same time: what lies in its chests and wardrobe and on its floor is theirs
//! alone, kept with their character on every world (the server's), and
//! nobody else is seen in it. Furnished as every bedroom is, by its tier
//! (`furnish/bedroom.rs`).

use super::UPSTAIRS;
use super::upstairs;
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

#[cfg(test)]
mod tests;
