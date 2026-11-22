//! Bar counters (`Furniture::BarCounter`, tavern furniture plan PR 3): a top
//! of boards over a front panel on the drinkers' side, open behind for the
//! barkeep. A single counter has a shelf low underneath for its vessels; a
//! double one a raised shelf on posts over the front of its top. Counters in
//! a line facing the same way join into one run: their tops meet and only
//! the run's ends have end panels. A flap is a gap in the run with its board
//! lifted up behind. Where each vessel stands (`bar_vessel_spot`) is drawn by
//! `rack_bottles.rs`. Sizes are first guesses for the owner to tune.

use bevy::prelude::*;
use shared::bar::{BarCounter, BarKind};
use shared::door::Side;
use shared::dungeon::Level;
use shared::furniture::{Furniture, Piece};

use super::super::CELL_SIZE;
use super::bottle_rack::from_back;
use super::frame::Frame;
use super::tables::Table;
use super::{Grain, INSET, Part, TABLE_TOP};

/// How high a counter's top is (a little over a metre, where a table's is
/// 0.39 of a cell), and how thick.
pub(in crate::dungeon_map) const BAR_TOP: f32 = CELL_SIZE * 0.55;
const TOP_THICKNESS: f32 = CELL_SIZE * 0.05;
/// How thick its panels and shelves are.
const BOARD: f32 = CELL_SIZE * 0.03;
/// Where a single counter's shelf underneath is, and a double one's raised
/// shelf above its top.
const LOW_SHELF: f32 = CELL_SIZE * 0.16;
const RAISED_SHELF: f32 = BAR_TOP + CELL_SIZE * 0.14;
/// How tall a lifted flap stands.
const FLAP: f32 = CELL_SIZE * 0.4;
/// The tallest vessel (a jug), for picking.
const TALLEST_VESSEL: f32 = CELL_SIZE * 0.13;
/// Twenty-fourths of a cell: how `BarPlace::along` is counted.
const TWENTY_FOURTH: f32 = CELL_SIZE / 24.0;

/// The bar piece `piece` is, if it is one.
fn counter_of(piece: &Piece) -> Option<BarCounter> {
    match piece.furniture {
        Furniture::BarCounter { counter } => Some(counter),
        _ => None,
    }
}

/// How much higher than on a table a piece standing on `piece` stands: a
/// counter's top over a table's.
pub(in crate::dungeon_map) fn lift_on(piece: &Piece) -> f32 {
    match counter_of(piece) {
        Some(_) => BAR_TOP - TABLE_TOP,
        None => 0.0,
    }
}

/// How high a bar piece and its vessels reach above the floor, for picking;
/// `None` for anything else.
pub(in crate::dungeon_map) fn bar_height(piece: &Piece) -> Option<f32> {
    Some(match counter_of(piece)?.kind {
        BarKind::Single => BAR_TOP,
        BarKind::Double => RAISED_SHELF + BOARD + TALLEST_VESSEL,
        BarKind::Flap => BAR_TOP + FLAP,
    })
}

/// Where vessel `index` of `piece`'s places stands in the world, its foot's
/// middle (on the level's flat floor: the caller raises it with the
/// ground); `None` for anything else.
pub(in crate::dungeon_map) fn bar_vessel_spot(piece: &Piece, index: usize) -> Option<Vec3> {
    let counter = counter_of(piece)?;
    let place = *counter.places().get(index)?;
    let depth = Table::of(piece).length;
    let along = f32::from(place.along) * TWENTY_FOURTH;
    let (t, y) = match counter.kind {
        BarKind::Single => (0.4, LOW_SHELF + BOARD),
        BarKind::Double => (0.72, RAISED_SHELF + BOARD),
        BarKind::Flap => return None,
    };
    let local = Vec3::new(from_back(piece, depth, t), y, along);
    Some(Frame::of(piece).place(local))
}

/// Whether the cell `ahead` of `piece`'s run (before its start, or after
/// its end) holds a counter of the same run: facing the same way, not a flap.
fn joins(level: &Level, piece: &Piece, ahead: bool) -> bool {
    let (dx, dy) = match piece.facing {
        Side::North | Side::South => (1, 0),
        Side::East | Side::West => (0, 1),
    };
    let (x, y) = piece.at;
    let (width, height) = (i64::from(piece.size.0), i64::from(piece.size.1));
    let cell = match ahead {
        false => (x - dx, y - dy),
        true => (x + dx * width, y + dy * height),
    };
    level.furniture_at(cell.0, cell.1).is_some_and(|other| {
        other.facing == piece.facing && counter_of(other).is_some_and(BarCounter::is_counter)
    })
}

/// A bar piece's parts, or `None` for anything else.
pub(super) fn parts(level: &Level, piece: &Piece) -> Option<Vec<Part>> {
    let counter = counter_of(piece)?;
    let table = Table::of(piece);
    let (depth, length) = (table.length, table.width);
    let mut parts = match counter.kind {
        BarKind::Flap => flap_parts(piece, depth, length),
        BarKind::Single | BarKind::Double => {
            let ends = (joins(level, piece, false), joins(level, piece, true));
            counter_parts(piece, counter.kind, (depth, length), ends)
        }
    };
    for (choice, part) in parts.iter_mut().enumerate() {
        *part = table.vary(*part, choice as i64);
    }
    Some(parts)
}

/// A box from the share `t0` to `t1` of the depth from the back, `y0` to
/// `y1` up and `z0` to `z1` along.
fn slab(piece: &Piece, depth: f32, (t0, t1): (f32, f32), ys: (f32, f32), zs: (f32, f32)) -> Part {
    let (a, b) = (from_back(piece, depth, t0), from_back(piece, depth, t1));
    let grain = if (ys.1 - ys.0) > (zs.1 - zs.0) {
        Grain::Up
    } else {
        Grain::Across
    };
    Part::block(Vec3::new(a, ys.0, zs.0), Vec3::new(b, ys.1, zs.1), grain)
}

/// A counter's top, front panel, end panels where its run ends, back posts,
/// and its shelf: underneath (single) or raised over the front (double).
fn counter_parts(
    piece: &Piece,
    kind: BarKind,
    (depth, length): (f32, f32),
    (joined_start, joined_end): (bool, bool),
) -> Vec<Part> {
    let z0 = if joined_start { 0.0 } else { INSET };
    let z1 = length - if joined_end { 0.0 } else { INSET };
    let front = 1.0 - BOARD / depth;
    let mut parts = vec![
        // The top, overhanging its front panel towards the drinkers.
        slab(
            piece,
            depth,
            (0.04, 1.0),
            (BAR_TOP - TOP_THICKNESS, BAR_TOP),
            (z0, z1),
        ),
        // The front panel and its kick board.
        slab(
            piece,
            depth,
            (front - 0.08, front - 0.05),
            (0.0, BAR_TOP - TOP_THICKNESS),
            (z0, z1),
        ),
        slab(
            piece,
            depth,
            (front - 0.12, front - 0.05),
            (0.0, CELL_SIZE * 0.05),
            (z0, z1),
        ),
    ];
    let post = BOARD * 1.6;
    for (joined, z) in [(joined_start, z0), (joined_end, z1 - BOARD)] {
        if !joined {
            parts.push(slab(
                piece,
                depth,
                (0.06, front - 0.05),
                (0.0, BAR_TOP - TOP_THICKNESS),
                (z, z + BOARD),
            ));
        }
    }
    // Back posts at each end of the piece (shared with a joined neighbour).
    for z in [z0, z1 - post] {
        parts.push(slab(
            piece,
            depth,
            (0.06, 0.06 + post / depth),
            (0.0, BAR_TOP - TOP_THICKNESS),
            (z, z + post),
        ));
    }
    match kind {
        BarKind::Single => {
            parts.push(slab(
                piece,
                depth,
                (0.1, front - 0.08),
                (LOW_SHELF, LOW_SHELF + BOARD),
                (z0, z1),
            ));
        }
        BarKind::Double => {
            parts.push(slab(
                piece,
                depth,
                (0.55, 0.92),
                (RAISED_SHELF, RAISED_SHELF + BOARD),
                (z0, z1),
            ));
            // Posts under it at each end and between cells.
            let cells = (length / CELL_SIZE).round() as i32;
            for i in 0..=cells {
                let z = (i as f32 * CELL_SIZE).clamp(z0, z1 - post);
                parts.push(slab(
                    piece,
                    depth,
                    (0.84, 0.84 + post / depth),
                    (BAR_TOP, RAISED_SHELF),
                    (z, z + post),
                ));
            }
        }
        BarKind::Flap => {}
    }
    parts
}

/// A flap: its board lifted upright at the back on two hinge posts, the
/// way through left open.
fn flap_parts(piece: &Piece, depth: f32, length: f32) -> Vec<Part> {
    let post = BOARD * 1.6;
    let mut parts = vec![slab(
        piece,
        depth,
        (0.05, 0.05 + BOARD / depth),
        (BAR_TOP, BAR_TOP + FLAP),
        (INSET, length - INSET),
    )];
    for z in [INSET, length - INSET - post] {
        parts.push(slab(
            piece,
            depth,
            (0.06, 0.06 + post / depth),
            (0.0, BAR_TOP + FLAP * 0.1),
            (z, z + post),
        ));
    }
    parts
}

#[cfg(test)]
mod tests;
