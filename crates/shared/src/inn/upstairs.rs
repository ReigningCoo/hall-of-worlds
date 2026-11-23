//! Upstairs: corridors 2 wide in an H (two legs north to south, a crossbar
//! between them with the stairs in its middle) and bedrooms on every side.
//! Each run of bedrooms (a strip) starts beside the stairs' end with the
//! smallest and grows away from it, the last taking what is left: the
//! largest, at the corners.

use super::builder::{Built, span};
use super::spots::{MIDDLE_X, STAIRS};
use super::{CORNER_TIER, DEPTH, RoomKind, WIDTH};

/// The west leg's columns; the east leg is its mirror.
const LEG_X: (i64, i64) = (9, 10);
/// The crossbar's rows.
const CROSSBAR_Y: (i64, i64) = (28, 29);
/// The smallest bedroom's length along its strip.
const FIRST_LENGTH: i64 = 3;

/// A run of bedrooms along a corridor, from `near` (beside the crossbar) to
/// `far` (an outer wall), `x0..=x1` across, their doors in the wall at
/// `door_x`, but the first one's in the crossbar's wall at `crossbar_wall`
/// if it borders it.
struct Strip {
    x0: i64,
    x1: i64,
    near: i64,
    far: i64,
    door_x: i64,
    crossbar_wall: Option<i64>,
}

/// Upstairs.
pub(super) fn build() -> Built {
    let mut built = Built::new();
    let mirror = |x: i64| 2 * MIDDLE_X - x;
    let legs = [
        span(LEG_X.0, 1, LEG_X.1, DEPTH - 2),
        span(mirror(LEG_X.1), 1, mirror(LEG_X.0), DEPTH - 2),
        span(LEG_X.1 + 1, CROSSBAR_Y.0, mirror(LEG_X.1) - 1, CROSSBAR_Y.1),
    ];
    built.room(RoomKind::Corridor, legs.to_vec());
    debug_assert!(legs[2].contains(STAIRS.0, STAIRS.1));
    for strip in strips() {
        bedrooms(&mut built, &strip);
    }
    built
}

/// Every strip: the outer ones west and east of the legs, north and south of
/// the crossbar's rows, and the inner ones between the legs, either side of
/// the middle wall, north and south of the crossbar.
fn strips() -> Vec<Strip> {
    let mirror = |x: i64| 2 * MIDDLE_X - x;
    let (north, south) = (CROSSBAR_Y.0 - 1, CROSSBAR_Y.1 + 1);
    let (outer, inner) = ((1, LEG_X.0 - 2), (LEG_X.1 + 2, MIDDLE_X - 1));
    let mut out = Vec::new();
    for west in [true, false] {
        let side = |(a, b): (i64, i64)| if west { (a, b) } else { (mirror(b), mirror(a)) };
        let door = |x: i64| if west { x } else { mirror(x) };
        let ((ox0, ox1), (ix0, ix1)) = (side(outer), side(inner));
        out.extend([
            strip(ox0, ox1, CROSSBAR_Y.0 - 1, 1, door(LEG_X.0 - 1), None),
            strip(ox0, ox1, CROSSBAR_Y.1, DEPTH - 2, door(LEG_X.0 - 1), None),
            strip(ix0, ix1, north - 1, 1, door(LEG_X.1 + 1), Some(north)),
            strip(
                ix0,
                ix1,
                south + 1,
                DEPTH - 2,
                door(LEG_X.1 + 1),
                Some(south),
            ),
        ]);
    }
    out
}

fn strip(x0: i64, x1: i64, near: i64, far: i64, door_x: i64, crossbar_wall: Option<i64>) -> Strip {
    Strip {
        x0,
        x1,
        near,
        far,
        door_x,
        crossbar_wall,
    }
}

/// The lengths of a strip's bedrooms from its near end: 3, 4, 5... while
/// one at least a row longer still fits after each, the last taking the rest.
pub(super) fn lengths(mut left: i64) -> Vec<i64> {
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

/// A strip's bedrooms, a wall between each, with their tiers and doors.
fn bedrooms(built: &mut Built, strip: &Strip) {
    let step = (strip.far - strip.near).signum();
    let all = lengths((strip.far - strip.near).abs() + 1);
    let mut from = strip.near;
    for (n, &length) in all.iter().enumerate() {
        let to = from + step * (length - 1);
        let tier = if n + 1 == all.len() {
            CORNER_TIER
        } else {
            n as u8 + 1
        };
        let rect = span(strip.x0, from.min(to), strip.x1, from.max(to));
        let room = built.room(RoomKind::Bedroom { tier }, vec![rect]);
        let middle_x = match strip.x0 > MIDDLE_X {
            true => (strip.x0 + strip.x1 + 1) / 2,
            false => (strip.x0 + strip.x1) / 2,
        };
        let door = match (n, strip.crossbar_wall) {
            (0, Some(wall)) => (middle_x, wall),
            _ => (strip.door_x, from + step * ((length - 1) / 2)),
        };
        built.door(door, Some(room), false);
        from = to + 2 * step;
    }
}

const _: () = assert!(WIDTH == 2 * MIDDLE_X + 1);
