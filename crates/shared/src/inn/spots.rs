//! Fixed places on the inn's floors, in its own cells (`x` east from the west
//! wall, `y` south from the back wall): where the doors, stairs and openings
//! are, and where furniture will go (The Way Inn plan, PR 5), kept clear of
//! anything else until then.

use super::builder::span;
use crate::site::Rect;

/// The middle column: the entrance, the archway, the stairs and the back
/// fireplace are on it, and both floors are the same either side of it.
pub const MIDDLE_X: i64 = 20;
/// The staircase between the floors: the same cell on both, in the middle of
/// the main room below and of the H's crossbar above.
pub const STAIRS: (i64, i64) = (MIDDLE_X, 29);

/// The entrance door, in the south wall: the one way in and out.
pub const ENTRANCE: (i64, i64) = (MIDDLE_X, 49);
/// Where you arrive, inside the entrance door.
pub const ARRIVAL: (i64, i64) = (MIDDLE_X, 48);
/// The foyer's floor, 3 × 3 inside the entrance.
pub const FOYER: Rect = span(19, 46, 21, 48);
/// The archway from the foyer into the main room: an opening with no door.
pub const ARCHWAY: (i64, i64) = (MIDDLE_X, 45);
/// The foyer's furniture along its west wall (a chair, a 1 × 1 table, a
/// chest) and its 1 × 3 chest along the east; the middle lane stays clear.
pub const FOYER_WEST: Rect = span(19, 46, 19, 48);
pub const FOYER_EAST: Rect = span(21, 46, 21, 48);

/// The private rooms down the main room's west side and the booths down its
/// east, from the back: their first row, and how far each next one is
/// (4 rows and a wall).
pub const ROW_FIRST_Y: i64 = 15;
pub const ROW_PITCH: i64 = 5;
/// How many of each.
pub const ROW_COUNT: i64 = 7;

/// Behind the bar: drink storage against the back wall (racks, kept stocked)
/// and a gap 2 wide in front of it.
pub const STORAGE: Rect = span(8, 11, 32, 11);
pub const BAR_GAP: Rect = span(8, 12, 32, 13);
/// The bar counter across the main room in front of the gap, wall to wall,
/// and its flap to get behind.
pub const BAR: Rect = span(8, 14, 32, 14);
pub const BAR_FLAP: (i64, i64) = (30, 14);
/// The fireplaces: one in the middle of the back wall (within the storage
/// row) and one either side of the entrance against the south wall.
pub const BACK_FIREPLACE: Rect = span(19, 11, 21, 11);
pub const FRONT_FIREPLACES: [Rect; 2] = [span(12, 48, 14, 48), span(26, 48, 28, 48)];
/// Where the scattered tables go (with walking room round each group, and
/// the stairs' cells kept clear): the main room between the bar's front and
/// the foyer.
pub const TABLES: Rect = span(9, 16, 31, 43);

/// The doors at the back wall's two ends, from behind the bar: into the
/// kitchen (west) and into the brewing room (east); and the ageing room's,
/// from the brewing room.
pub const KITCHEN_DOOR: (i64, i64) = (7, 12);
pub const BREWING_DOOR: (i64, i64) = (33, 12);
pub const AGEING_DOOR: (i64, i64) = (30, 5);
/// The kitchen's back-left corner, kept empty for a stair down later.
pub const STAIR_DOWN_CORNER: Rect = span(1, 1, 3, 3);
/// The treasury, 5 × 5 behind the kitchen: its locked door, and a window
/// either side of it for the kitchen to see in.
pub const TREASURY: Rect = span(8, 1, 12, 5);
pub const TREASURY_DOOR: (i64, i64) = (10, 6);
pub const TREASURY_WINDOWS: [(i64, i64); 2] = [(9, 6), (11, 6)];
