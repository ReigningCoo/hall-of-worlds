//! The inn's rooms: what each is for, where it is and its door.

use crate::dungeon::RoomId;
use crate::site::Rect;

/// The tier of the largest bedrooms, at the corners furthest from the
/// stairs. Tiers count up from 1 beside the stairs.
pub const CORNER_TIER: u8 = 4;

/// What a room of the inn is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RoomKind {
    /// Inside the entrance door, 3 × 3, opening into the main room through
    /// an archway.
    Foyer,
    /// The main room, the bar and what is behind it included (the bar fills
    /// vessels from what is stored in its own room).
    Main,
    /// A room off the main room's left side, closed by a door.
    PrivateRoom,
    /// An open booth along the main room's right side, the private rooms'
    /// size.
    Booth,
    Kitchen,
    Brewing,
    Ageing,
    /// Behind the kitchen, locked, seen through its windows: never entered.
    Treasury,
    /// Upstairs' corridors (an H).
    Corridor,
    /// A bedroom upstairs, by tier: 1 beside the stairs, larger further out,
    /// `CORNER_TIER` the largest.
    Bedroom {
        tier: u8,
    },
}

impl RoomKind {
    /// Whether everyone in an instance shares it: all but private rooms and
    /// bedrooms, which become each player's own (The Way Inn plan, PR 4).
    /// Things left lying in these are cleared after a while.
    pub fn is_public(self) -> bool {
        !matches!(self, Self::PrivateRoom | Self::Bedroom { .. })
    }
}

/// One room of the inn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnRoom {
    /// Its id on its level (`Level::room`).
    pub id: RoomId,
    pub kind: RoomKind,
    /// Its floor's cells, as rectangles that don't overlap.
    pub rects: Vec<Rect>,
    /// Its own door, if it has one (private rooms, bedrooms, the treasury).
    pub door: Option<(i64, i64)>,
}

impl InnRoom {
    /// Whether `(x, y)` is one of its floor's cells.
    pub fn contains(&self, x: i64, y: i64) -> bool {
        self.rects.iter().any(|rect| rect.contains(x, y))
    }

    /// How many floor cells it has.
    pub fn area(&self) -> i64 {
        self.rects.iter().map(|rect| rect.width * rect.height).sum()
    }
}
