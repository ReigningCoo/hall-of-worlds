//! The Way Inn: the inn off the Hall of Worlds where travellers meet, the
//! starting place for new players. Like the Hall it is made by no generator:
//! its two floors are fixed shapes written in code, the same for everyone, on
//! every world. It stands in the void with no windows out; you always come in
//! through its one entrance door, from the Hall's hidden way-ins
//! (`hall::inn_way`).
//!
//! The ground floor: a foyer inside the entrance, the main room with private
//! rooms down its left side and open booths down its right, a staircase up in
//! its middle and the bar across its back, storage behind the bar, the
//! kitchen (with the treasury behind it) through a door at the back's left
//! end and the brewing and ageing rooms through one at its right. Upstairs:
//! corridors in an H, the stairs arriving in the middle of its crossbar, and
//! bedrooms on every side, small near the stairs and larger further out.
//!
//! Only the shapes for now (cells, rooms, doors, stairs); the fixed places
//! furniture will go (`spots`) are kept clear for it. See
//! `docs/context/shared/inn.md` and the plan, `docs/context/the-way-inn.md`.

mod builder;
mod ground;
mod room;
pub mod spots;
mod upstairs;

pub use room::{CORNER_TIER, InnRoom, RoomKind};

use crate::site::{Exit, SiteLevel, Stairs, StairsDirection};

/// The inn's floors: the ground floor, `0`, and upstairs, `1`.
pub const FLOORS: u32 = 2;
/// The ground floor's index, entered from the Hall.
pub const GROUND: u32 = 0;
/// Upstairs' index.
pub const UPSTAIRS: u32 = 1;
/// Its width east to west in cells (`x` from `0`), walls included; both
/// floors are the same.
pub const WIDTH: i64 = 41;
/// Its depth north to south in cells (`y` from `0`, the back), walls
/// included. The entrance is in the south wall.
pub const DEPTH: i64 = 50;

/// Floor `index` (`GROUND` or `UPSTAIRS`) as a site's level, or `None` past
/// the top: its cells and rooms, its doors closed (the treasury's locked),
/// the stairs between the floors, and on the ground floor where you arrive
/// (inside the entrance door).
pub fn level(index: u32) -> Option<SiteLevel> {
    let (built, stairs, entrance) = match index {
        GROUND => (ground::build(), StairsDirection::Up, Some(spots::ARRIVAL)),
        UPSTAIRS => (upstairs::build(), StairsDirection::Down, None),
        _ => return None,
    };
    let to = Exit::Level(FLOORS - 1 - index);
    let (x, y) = spots::STAIRS;
    Some(SiteLevel {
        level: built.level,
        stairs: vec![Stairs {
            x,
            y,
            direction: stairs,
            to,
        }],
        entrance,
        buildings: Vec::new(),
        fields: Vec::new(),
        walls: Vec::new(),
    })
}

/// Floor `index`'s rooms, in the order their ids were given (empty past the
/// top).
pub fn rooms(index: u32) -> Vec<InnRoom> {
    match index {
        GROUND => ground::build().rooms,
        UPSTAIRS => upstairs::build().rooms,
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests;
