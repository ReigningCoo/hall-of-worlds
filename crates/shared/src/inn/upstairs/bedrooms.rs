//! Upstairs' bedrooms, where each is and its tier (owner's layout,
//! 2026-11-23). Beside the crossbar, between the legs: a row of 4 small
//! rooms along its north side and 3 along its south, their doors on it.
//! Behind those rows, and down the outer side of each leg, runs of bedrooms
//! (strips) from beside the crossbar to an outer wall, growing as they go,
//! their doors on the legs, the last taking what is left: the largest. The
//! two far north ones between the legs are one room across the middle, its
//! one door on the east leg.
//!
//! Tiers: the crossbar's rows 1; the strips between the legs 2, 3, then 4;
//! the outer strips 1 upwards; the last of every strip `CORNER_TIER`.

use super::super::builder::span;
use super::super::spots::MIDDLE_X;
use super::super::{CORNER_TIER, DEPTH};
use super::{CROSSBAR_Y, LEG_X};
use crate::site::Rect;

/// The smallest bedroom's length along its strip.
const FIRST_LENGTH: i64 = 3;
/// The crossbar's rows of rooms, west to east: their widths (walls between),
/// filling the 17 columns between the legs; both 3 deep.
const NORTH_ROW: [i64; 4] = [3, 4, 4, 3];
const SOUTH_ROW: [i64; 3] = [5, 5, 5];
const ROW_DEPTH: i64 = 3;
/// Which of the north row is the open tier's own room: the nearest the
/// stairs on the west.
const OWN_IN_ROW: usize = 1;

/// One bedroom: its tier, floor and door, and whether it is a player's own
/// room.
#[derive(Clone, Copy, Debug)]
pub(super) struct Bedroom {
    pub(super) tier: u8,
    pub(super) rect: Rect,
    pub(super) door: (i64, i64),
    pub(super) own: bool,
}

/// A run of bedrooms along a leg, from `near` (beside the crossbar) to `far`
/// (an outer wall), `x0..=x1` across, their doors in the wall at `door_x`;
/// the first is tier `first_tier`.
struct Strip {
    across: (i64, i64),
    near: i64,
    far: i64,
    door_x: i64,
    first_tier: u8,
}

fn mirror(x: i64) -> i64 {
    2 * MIDDLE_X - x
}

/// Every bedroom upstairs: the crossbar's rows, then the strips, then the
/// joined room in the north between the legs.
pub(super) fn plan() -> Vec<Bedroom> {
    let inner_x0 = LEG_X.1 + 2;
    let north_wall = CROSSBAR_Y.0 - 1;
    let south_wall = CROSSBAR_Y.1 + 1;
    let mut out = row(&NORTH_ROW, inner_x0, north_wall - ROW_DEPTH, north_wall);
    out[OWN_IN_ROW].own = true;
    out.extend(row(&SOUTH_ROW, inner_x0, south_wall + 1, south_wall));
    let (outer, inner) = ((1, LEG_X.0 - 2), (inner_x0, MIDDLE_X - 1));
    let behind_north = north_wall - ROW_DEPTH - 2;
    let behind_south = south_wall + ROW_DEPTH + 2;
    let south_far = DEPTH - 2;
    let mut far_north = Vec::new();
    for west in [true, false] {
        let side = |(a, b): (i64, i64)| if west { (a, b) } else { (mirror(b), mirror(a)) };
        let door = |x: i64| if west { x } else { mirror(x) };
        let (outer, inner) = (side(outer), side(inner));
        let (outer_door, inner_door) = (door(LEG_X.0 - 1), door(LEG_X.1 + 1));
        let strips = [
            strip(outer, (north_wall, 1), outer_door, 1),
            strip(outer, (CROSSBAR_Y.1, south_far), outer_door, 1),
            strip(inner, (behind_south, south_far), inner_door, 2),
        ];
        for one in &strips {
            out.extend(strip_rooms(one));
        }
        let mut north = strip_rooms(&strip(inner, (behind_north, 1), inner_door, 2));
        far_north.push(north.pop().expect("a last room"));
        if west {
            // The west one's rooms of tiers 2 and 3 are own rooms.
            north[0].own = true;
            north[1].own = true;
        }
        out.extend(north);
    }
    out.push(joined(&far_north));
    out
}

/// A row of rooms side by side from `x0` east, `ROW_DEPTH` from row `y0`,
/// their doors in the wall at row `wall`: tier 1.
fn row(widths: &[i64], x0: i64, y0: i64, wall: i64) -> Vec<Bedroom> {
    let mut x = x0;
    let mut out = Vec::new();
    for &width in widths {
        out.push(Bedroom {
            tier: 1,
            rect: span(x, y0, x + width - 1, y0 + ROW_DEPTH - 1),
            door: (x + (width - 1) / 2, wall),
            own: false,
        });
        x += width + 1;
    }
    out
}

/// The far north rooms between the legs (west, east) as one, across the
/// middle wall, its one door the east one's: the largest tier, an own room.
fn joined(rooms: &[Bedroom]) -> Bedroom {
    let (west, east) = (rooms[0].rect, rooms[1]);
    let x1 = east.rect.x + east.rect.width - 1;
    Bedroom {
        tier: CORNER_TIER,
        rect: span(west.x, west.y, x1, west.y + west.height - 1),
        door: east.door,
        own: true,
    }
}

fn strip(across: (i64, i64), (near, far): (i64, i64), door_x: i64, first_tier: u8) -> Strip {
    Strip {
        across,
        near,
        far,
        door_x,
        first_tier,
    }
}

/// The lengths of a strip's bedrooms from its near end: 3, 4, 5... while
/// one at least a row longer still fits after each, the last taking the rest.
pub(in super::super) fn lengths(mut left: i64) -> Vec<i64> {
    let mut out = Vec::new();
    let mut length = FIRST_LENGTH;
    while left > 2 * length + 1 {
        out.push(length);
        left -= length + 1;
        length += 1;
    }
    out.push(left);
    out
}

/// A strip's bedrooms, a wall between each, from its near end; tiers from
/// its first up to `CORNER_TIER`, the last always that.
fn strip_rooms(strip: &Strip) -> Vec<Bedroom> {
    let step = (strip.far - strip.near).signum();
    let all = lengths((strip.far - strip.near).abs() + 1);
    let (x0, x1) = strip.across;
    let mut from = strip.near;
    let mut out = Vec::new();
    for (n, &length) in all.iter().enumerate() {
        let to = from + step * (length - 1);
        let tier = match n + 1 == all.len() {
            true => CORNER_TIER,
            false => (strip.first_tier + n as u8).min(CORNER_TIER),
        };
        out.push(Bedroom {
            tier,
            rect: span(x0, from.min(to), x1, from.max(to)),
            door: (strip.door_x, from + step * ((length - 1) / 2)),
            own: false,
        });
        from = to + 2 * step;
    }
    out
}
