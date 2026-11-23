//! The ground floor (see `spots` for its fixed places).
//!
//! From the back (north, `y` 0): the kitchen (west half, wrapped round the
//! treasury) beside the ageing and brewing rooms, the back wall at `y` 10,
//! then the main room with the storage, gap and bar across its back, private
//! rooms down its west side and booths down its east (`ROW_COUNT` each, 4
//! rows and a wall apart), and the foyer in the middle of the south wall.

use super::builder::{Built, span};
use super::spots::*;
use super::{RoomKind, WIDTH};

/// The private rooms' columns (west, inside the outer wall) and the wall
/// their doors are in.
const ROOMS_X: (i64, i64) = (1, 6);
const ROOMS_WALL_X: i64 = 7;
/// The booths' columns (east, inside the outer wall), and the column in
/// front of them that is the main room's.
const BOOTHS_X: (i64, i64) = (34, 39);
const BOOTHS_FRONT_X: i64 = 33;

/// The ground floor.
pub(super) fn build() -> Built {
    let mut built = Built::new();
    let archway = span(ARCHWAY.0, ARCHWAY.1, ARCHWAY.0, ARCHWAY.1);
    built.room(RoomKind::Foyer, vec![FOYER, archway]);
    main_room(&mut built);
    rows(&mut built);
    back_rooms(&mut built);
    built.door(ENTRANCE, None, false);
    built
}

/// The main room: from the storage behind the bar to the south wall, round
/// the foyer, and the column in front of the booths.
fn main_room(built: &mut Built) {
    let (front_y, south_y) = (FOYER.y - 1, FOYER.y + FOYER.height - 1);
    let mut rects = vec![
        span(STORAGE.x, STORAGE.y, BAR.x + BAR.width - 1, front_y - 1),
        span(STORAGE.x, front_y, FOYER.x - 2, south_y),
        span(
            FOYER.x + FOYER.width + 1,
            front_y,
            BAR.x + BAR.width - 1,
            south_y,
        ),
    ];
    rects.extend((0..ROW_COUNT).map(|n| {
        let y = ROW_FIRST_Y + n * ROW_PITCH;
        span(BOOTHS_FRONT_X, y, BOOTHS_FRONT_X, y + ROW_PITCH - 2)
    }));
    built.room(RoomKind::Main, rects);
}

/// The private rooms, each with its door into the main room, then the
/// booths, open to it.
fn rows(built: &mut Built) {
    for n in 0..ROW_COUNT {
        let y = ROW_FIRST_Y + n * ROW_PITCH;
        let rect = span(ROOMS_X.0, y, ROOMS_X.1, y + ROW_PITCH - 2);
        let room = built.room(RoomKind::PrivateRoom, vec![rect]);
        built.door((ROOMS_WALL_X, y + 1), Some(room), false);
    }
    for n in 0..ROW_COUNT {
        let y = ROW_FIRST_Y + n * ROW_PITCH;
        let rect = span(BOOTHS_X.0, y, BOOTHS_X.1, y + ROW_PITCH - 2);
        built.room(RoomKind::Booth, vec![rect]);
    }
}

/// Behind the back wall: the kitchen round the treasury (and down beside
/// the bar's back to its door), the treasury, and the ageing and brewing
/// rooms (the brewing room also down beside the bar's back).
fn back_rooms(built: &mut Built) {
    let back_y = STORAGE.y - 1;
    let (west, east) = (TREASURY.x - 1, TREASURY.x + TREASURY.width);
    let kitchen = vec![
        span(1, 1, KITCHEN_DOOR.0 - 1, BAR.y - 1),
        span(west, TREASURY_DOOR.1 + 1, MIDDLE_X - 1, back_y - 1),
        span(east + 1, 1, MIDDLE_X - 1, TREASURY_DOOR.1),
    ];
    built.room(RoomKind::Kitchen, kitchen);
    built.door(KITCHEN_DOOR, None, false);
    let treasury = built.room(RoomKind::Treasury, vec![TREASURY]);
    built.door(TREASURY_DOOR, Some(treasury), true);
    for at in TREASURY_WINDOWS {
        built.window(at);
    }
    let brewing = vec![
        span(AGEING_DOOR.0 + 1, 1, WIDTH - 2, back_y - 1),
        span(BREWING_DOOR.0 + 1, back_y, WIDTH - 2, BAR.y - 1),
    ];
    built.room(RoomKind::Brewing, brewing);
    built.door(BREWING_DOOR, None, false);
    built.room(
        RoomKind::Ageing,
        vec![span(MIDDLE_X + 1, 1, AGEING_DOOR.0 - 1, back_y - 1)],
    );
    built.door(AGEING_DOOR, None, false);
}
