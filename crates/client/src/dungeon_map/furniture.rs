//! Furniture (`shared::furniture`) as models: each piece is a set of boards,
//! legs and blocks (`Part`s) in its own frame, which every cell it covers
//! draws its share of, so sight, memory and the edge of sight work cell by
//! cell as for floors. Wood grain and stone come from one grey picture
//! (`assets/furniture_atlas.png`, made by `tools/make_furniture_atlas.py`),
//! tinted per vertex by the wood or stone and lit where they stand. Tops stay
//! level on sloping ground, their legs reaching down to it. Black outlines.
//! Seats facing a table reach a little under its edge. Crafting benches are
//! a table with a shelf over its back (`crafting_bench.rs`); hearths a stone
//! stove with a hood, and a counter (`hearth.rs`). Cask racks are rails,
//! saddles or a stand under their casks (`cask_rack.rs`). Wine racks are a
//! diamond lattice against a wall, spirit racks three steps standing on the
//! table under them (`bottle_rack.rs`).

// Split into files (`cargo xtask lint` keeps files small).
mod atlas;
mod benches;
mod bottle_rack;
mod cask_rack;
mod chairs;
mod colour;
mod crafting_bench;
mod draw;
mod frame;
mod hearth;
mod part;
mod seats;
mod stone_tables;
mod tables;
mod wooden_tables;

use bevy::prelude::*;
use part::{Grain, Part};

use super::CELL_SIZE;
use super::walls::{Layers, Scene};

pub(super) use bottle_rack::{TALLEST as TALLEST_BOTTLE, bottle_rack_height, bottle_spot};
pub(super) use cask_rack::{cask_scale, cask_spot, rack_height};
pub(super) use crafting_bench::{ghost_boxes, tool_spot};
pub(super) use hearth::firebox;

/// How high a table's top is, and how thick.
const TABLE_TOP: f32 = CELL_SIZE * 0.39;
const TOP_THICKNESS: f32 = CELL_SIZE * 0.05;
/// How far a table's top stands in from the edges of its cells.
const INSET: f32 = CELL_SIZE * 0.07;

/// The part of any piece of furniture at `(x, y)`, and of any standing on
/// it: lit if in sight, else sepia.
pub(super) fn build_furniture(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) {
    if let Some(piece) = scene.level.furniture_at(x, y) {
        build_piece(layers, scene, piece, (x, y));
    }
    if let Some(piece) = scene.level.furniture_on_top_at(x, y) {
        build_piece(layers, scene, piece, (x, y));
    }
}

/// The part of `piece` at `(x, y)`.
fn build_piece(
    layers: &mut Layers,
    scene: &Scene,
    piece: &shared::furniture::Piece,
    (x, y): (i64, i64),
) {
    let mut beyond = (Vec3::ZERO, Vec3::ZERO);
    let parts = tables::parts(piece)
        .or_else(|| crafting_bench::parts(piece))
        .or_else(|| cask_rack::parts(piece))
        .or_else(|| bottle_rack::parts(piece));
    let parts = match parts {
        Some(parts) => parts,
        None => {
            let under_table = seats::faces_table(scene.level, piece);
            if under_table {
                let (dx, dy) = piece.facing.beside((0, 0));
                let ahead = Vec3::new(dx as f32, 0.0, dy as f32) * seats::OVERHANG;
                beyond = (ahead.min(Vec3::ZERO).abs(), ahead.max(Vec3::ZERO));
            }
            seats::parts(piece, under_table).unwrap_or_default()
        }
    };
    draw::draw_parts(layers, scene, (piece, &parts), (x, y), beyond);
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod seats_tests;
