//! Games tables' and crates' parts (tavern furniture plan PR 6). A games
//! table is a small joined table; its board and men are drawn with decor
//! (`decor/games.rs`), in their own colours. Crates are boxes of boards with
//! corner battens and a band round them, stacked by how many there are.

use bevy::prelude::*;
use shared::furniture::{Crates, Furniture, Piece};

use super::super::CELL_SIZE;
use super::tables::Table;
use super::{Grain, Part, wooden_tables};

const C: f32 = CELL_SIZE;
/// A crate's side, and how tall one is.
const CRATE: f32 = C * 0.38;
const CRATE_HEIGHT: f32 = C * 0.3;
/// How far battens stand out of a crate's sides, and how wide they are.
const BATTEN: f32 = C * 0.012;
const BATTEN_WIDTH: f32 = C * 0.04;

/// A games table's or crates' parts, or `None` for anything else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    match piece.furniture {
        Furniture::GamesTable { .. } => Some(wooden_tables::joined(&Table::of(piece))),
        Furniture::Crates { crates } => Some(crate_stack(&Table::of(piece), crates)),
        _ => None,
    }
}

/// Where each crate's near bottom corner is, by how they are stacked.
fn corners(crates: Crates) -> Vec<Vec3> {
    let middle = (C - CRATE) / 2.0;
    let (left, right) = (C * 0.09, C - C * 0.09 - CRATE);
    let up = CRATE_HEIGHT + BATTEN;
    match crates {
        Crates::One => vec![Vec3::new(middle, 0.0, middle)],
        Crates::Two => vec![
            Vec3::new(middle, 0.0, middle),
            Vec3::new(middle + C * 0.03, up, middle - C * 0.02),
        ],
        Crates::Three => vec![
            Vec3::new(left, 0.0, middle),
            Vec3::new(right, 0.0, middle + C * 0.02),
            Vec3::new(middle, up, middle + C * 0.01),
        ],
        Crates::Four => vec![
            Vec3::new(left, 0.0, middle),
            Vec3::new(right, 0.0, middle + C * 0.02),
            Vec3::new(left + C * 0.02, up, middle - C * 0.01),
            Vec3::new(right - C * 0.01, up, middle + C * 0.03),
        ],
    }
}

/// The crates stacked on the cell.
fn crate_stack(table: &Table, crates: Crates) -> Vec<Part> {
    let mut parts = Vec::new();
    for (i, low) in corners(crates).into_iter().enumerate() {
        let i = i as i64 * 10;
        let high = low + Vec3::new(CRATE, CRATE_HEIGHT, CRATE);
        parts.push(table.vary(Part::block(low, high, Grain::Along), i));
        // A batten up each corner, standing out of both sides.
        for (cx, cz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            let x = if cx == 0 {
                low.x - BATTEN
            } else {
                high.x - BATTEN_WIDTH + BATTEN
            };
            let z = if cz == 0 {
                low.z - BATTEN
            } else {
                high.z - BATTEN_WIDTH + BATTEN
            };
            let batten = Part::block(
                Vec3::new(x, low.y, z),
                Vec3::new(x + BATTEN_WIDTH, high.y, z + BATTEN_WIDTH),
                Grain::Up,
            );
            parts.push(table.vary(batten, i + 1 + cx + 2 * cz));
        }
        // A band round the middle.
        let band = low.y + CRATE_HEIGHT * 0.45;
        let around = Part::block(
            Vec3::new(low.x - BATTEN / 2.0, band, low.z - BATTEN / 2.0),
            Vec3::new(
                high.x + BATTEN / 2.0,
                band + BATTEN_WIDTH,
                high.z + BATTEN / 2.0,
            ),
            Grain::Along,
        );
        parts.push(table.vary(around, i + 7));
    }
    parts
}
