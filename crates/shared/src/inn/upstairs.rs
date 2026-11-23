//! Upstairs: corridors 2 wide in an H (two legs north to south, a crossbar
//! between them with the stairs in its middle) and bedrooms on every side
//! (`bedrooms.rs`): small beside the crossbar, growing away from it. Some
//! of them, a tier each, are players' own rooms (`OwnRoom`, `own_rooms`);
//! the rest are locked for now.

use super::builder::{Built, span};
use super::spots::{MIDDLE_X, STAIRS};
use super::{DEPTH, OPEN_TIER, RoomKind, WIDTH};
use crate::site::Rect;

#[cfg(test)]
pub(super) use bedrooms::lengths;

mod bedrooms;

/// The west leg's columns; the east leg is its mirror.
const LEG_X: (i64, i64) = (9, 10);
/// The crossbar's rows.
const CROSSBAR_Y: (i64, i64) = (28, 29);

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
    for bedroom in bedrooms::plan() {
        let tier = bedroom.tier;
        let kind = match bedroom.own {
            true => RoomKind::OwnRoom { tier },
            false => RoomKind::Bedroom { tier },
        };
        let room = built.room(kind, vec![bedroom.rect]);
        // Only `OPEN_TIER`'s own room opens: nobody owns a larger tier yet,
        // and plain bedrooms are for later.
        let locked = !bedroom.own || tier > OPEN_TIER;
        built.door(bedroom.door, Some(room), locked);
    }
    built
}

/// Players' own rooms upstairs, smallest tier first: each tier's room, its
/// floor and its door.
pub(super) fn own_rooms() -> Vec<(u8, Rect, (i64, i64))> {
    let mut own: Vec<_> = (bedrooms::plan().into_iter())
        .filter(|bedroom| bedroom.own)
        .map(|bedroom| (bedroom.tier, bedroom.rect, bedroom.door))
        .collect();
    own.sort_by_key(|&(tier, ..)| tier);
    own
}

const _: () = assert!(WIDTH == 2 * MIDDLE_X + 1);
