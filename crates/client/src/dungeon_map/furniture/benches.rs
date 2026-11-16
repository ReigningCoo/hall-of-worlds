//! Benches by style, 1 to 3 cells long and never with arms: a plank on
//! splayed legs, a seat between solid end boards, a long stool with rails
//! between its legs, and a stone slab on blocks.

use shared::furniture::BenchStyle;

use super::super::CELL_SIZE;
use super::seats::{SEAT_TOP, Seat};
use super::{Grain, Part};

/// How deep a bench is, from back to front.
pub(super) const DEPTH: f32 = CELL_SIZE * 0.24;
/// How far its ends stand in from the ends of its cells.
const END: f32 = CELL_SIZE * 0.06;
/// How thick its seat is, and its legs.
const BOARD: f32 = CELL_SIZE * 0.04;
const LEG: f32 = CELL_SIZE * 0.045;

/// A bench's parts in the seat's terms.
pub(super) fn parts(seat: &Seat, style: BenchStyle) -> Vec<Part> {
    match style {
        BenchStyle::SplayedPlank => splayed(seat),
        BenchStyle::BoardEnded => board_ended(seat),
        BenchStyle::LongStool => long_stool(seat),
        BenchStyle::StoneSlab => stone_slab(seat),
    }
}

/// The seat: one board along its length, or a slab of `thick`.
fn top(seat: &Seat, thick: f32, grain: Grain) -> Part {
    seat.part(
        (0.0, SEAT_TOP - thick, END),
        (seat.depth, SEAT_TOP, seat.length - END),
        grain,
        1,
    )
}

/// Where legs or supports stand along it: near each end, and in the middle of
/// a long one.
fn stations(seat: &Seat, inset: f32, width: f32) -> Vec<f32> {
    let mut at = vec![END + inset, seat.length - END - inset - width];
    if seat.length >= CELL_SIZE * 2.5 {
        at.push((seat.length - width) / 2.0);
    }
    at
}

/// A thick plank on four legs (six on a long one), splayed out at the ends
/// and to the front and back.
fn splayed(seat: &Seat) -> Vec<Part> {
    let mut parts = vec![top(seat, BOARD * 1.4, Grain::Along)];
    let splay = CELL_SIZE * 0.04;
    let ends = stations(seat, CELL_SIZE * 0.08, LEG);
    let last = ends[1];
    for &u in &ends {
        // Feet out towards the nearer end; the middle pair straight.
        let out = if u == ends[0] {
            -splay
        } else if u == last {
            splay
        } else {
            0.0
        };
        // Feet at the seat's edges (never out under a table), tops leaning
        // in towards its middle.
        for (f, lean) in [(0.0, splay), (seat.depth - LEG, -splay)] {
            parts.push(seat.leaning(
                (f, 0.0, u + out),
                (f + LEG, SEAT_TOP - BOARD * 1.4, u + out + LEG),
                Grain::Up,
                (lean, -out),
            ));
        }
    }
    parts
}

/// A seat between two solid boards at its ends (and one in the middle of a
/// long one), with a rail under its front edge.
fn board_ended(seat: &Seat) -> Vec<Part> {
    let mut parts = vec![top(seat, BOARD, Grain::Along)];
    let thick = CELL_SIZE * 0.035;
    for (i, u) in stations(seat, CELL_SIZE * 0.02, thick)
        .into_iter()
        .enumerate()
    {
        parts.push(seat.part(
            (CELL_SIZE * 0.01, 0.0, u),
            (seat.depth - CELL_SIZE * 0.01, SEAT_TOP - BOARD, u + thick),
            Grain::Up,
            20 + i as i64,
        ));
    }
    let rail = CELL_SIZE * 0.06;
    parts.push(seat.part(
        (
            seat.depth - CELL_SIZE * 0.04,
            SEAT_TOP - BOARD - rail,
            END + thick,
        ),
        (
            seat.depth - CELL_SIZE * 0.015,
            SEAT_TOP - BOARD,
            seat.length - END - thick,
        ),
        Grain::Along,
        30,
    ));
    parts
}

/// Straight legs at the corners (and the middle of a long one), joined by
/// low rails along the front and back.
fn long_stool(seat: &Seat) -> Vec<Part> {
    let mut parts = vec![top(seat, BOARD, Grain::Along)];
    let inset = CELL_SIZE * 0.02;
    let (back, front) = (inset, seat.depth - inset - LEG);
    for (i, u) in stations(seat, CELL_SIZE * 0.03, LEG)
        .into_iter()
        .enumerate()
    {
        for f in [back, front] {
            parts.push(seat.part(
                (f, 0.0, u),
                (f + LEG, SEAT_TOP - BOARD, u + LEG),
                Grain::Up,
                40 + i as i64,
            ));
        }
    }
    let (low, rail) = (CELL_SIZE * 0.07, CELL_SIZE * 0.025);
    for f in [back, front] {
        let middle = f + LEG / 2.0;
        parts.push(seat.part(
            (middle - rail / 2.0, low, END + CELL_SIZE * 0.03),
            (
                middle + rail / 2.0,
                low + rail,
                seat.length - END - CELL_SIZE * 0.03,
            ),
            Grain::Along,
            50,
        ));
    }
    parts
}

/// A stone slab on a block near each end (and one in the middle of a long
/// one).
fn stone_slab(seat: &Seat) -> Vec<Part> {
    let slab = CELL_SIZE * 0.06;
    let mut parts = vec![top(seat, slab, Grain::Up)];
    let thick = CELL_SIZE * 0.13;
    for (i, u) in stations(seat, CELL_SIZE * 0.05, thick)
        .into_iter()
        .enumerate()
    {
        parts.push(seat.part(
            (CELL_SIZE * 0.03, 0.0, u),
            (seat.depth - CELL_SIZE * 0.03, SEAT_TOP - slab, u + thick),
            Grain::Up,
            60 + i as i64,
        ));
    }
    parts
}
