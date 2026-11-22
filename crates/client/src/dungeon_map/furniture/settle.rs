//! A high-backed settle (tavern furniture plan PR 5): a bench boxed in, its
//! front panelled down to the floor, solid ends rising to arm height, and a
//! back of boards as tall as someone sitting, standing behind the seat. Two
//! facing across a table make a booth. Sizes are first guesses.

use super::super::CELL_SIZE;
use super::seats::{SEAT_TOP, Seat};
use super::{Grain, Part};

/// How thick its boards are.
const BOARD: f32 = CELL_SIZE * 0.035;
/// How tall its back stands, and its ends.
const BACK_TOP: f32 = CELL_SIZE * 0.8;
const END_TOP: f32 = CELL_SIZE * 0.4;

/// A settle's parts in the seat's terms.
pub(super) fn parts(seat: &Seat) -> Vec<Part> {
    let (depth, length) = (seat.depth, seat.length);
    let mut parts = vec![
        // The seat, and the panel under its front.
        seat.part(
            (0.0, SEAT_TOP - BOARD, BOARD),
            (depth, SEAT_TOP, length - BOARD),
            Grain::Along,
            1,
        ),
        seat.part(
            (depth - BOARD, 0.0, BOARD),
            (depth, SEAT_TOP - BOARD, length - BOARD),
            Grain::Along,
            2,
        ),
    ];
    // The back, in boards up its height, behind the seat.
    let boards = 3;
    let step = BACK_TOP / boards as f32;
    for i in 0..boards {
        parts.push(seat.part(
            (-BOARD, step * i as f32, 0.0),
            (0.0, step * (i + 1) as f32, length),
            Grain::Along,
            10 + i64::from(i),
        ));
    }
    // Solid ends from the back to the front, to arm height.
    for (i, u) in [0.0, length - BOARD].into_iter().enumerate() {
        parts.push(seat.part(
            (0.0, 0.0, u),
            (depth, END_TOP, u + BOARD),
            Grain::Up,
            20 + i as i64,
        ));
    }
    parts
}
