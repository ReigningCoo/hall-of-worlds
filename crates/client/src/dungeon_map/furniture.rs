//! Furniture (`shared::furniture`) as models: each piece is a set of boards,
//! legs and blocks (`Part`s) in its own frame, which every cell it covers
//! draws its share of, so sight, memory and the edge of sight work cell by
//! cell as for floors. Wood grain and stone come from one grey picture
//! (`assets/furniture_atlas.png`, made by `tools/make_furniture_atlas.py`),
//! tinted per vertex by the wood or stone and lit where they stand. Tops stay
//! level on sloping ground, their legs reaching down to it. Black outlines.
//! Seats facing a table reach a little under its edge.

// Split into files (`cargo xtask lint` keeps files small).
mod atlas;
mod benches;
mod chairs;
mod colour;
mod draw;
mod frame;
mod part;
mod seats;
mod stone_tables;
mod tables;
mod wooden_tables;

use bevy::prelude::*;
use part::{Grain, Part};

use super::CELL_SIZE;
use super::walls::{Layers, Scene};

/// How high a table's top is, and how thick.
const TABLE_TOP: f32 = CELL_SIZE * 0.39;
const TOP_THICKNESS: f32 = CELL_SIZE * 0.05;
/// How far a table's top stands in from the edges of its cells.
const INSET: f32 = CELL_SIZE * 0.07;

/// The part of any piece of furniture at `(x, y)`: lit if in sight, else grey.
pub(super) fn build_furniture(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) {
    let Some(piece) = scene.level.furniture_at(x, y) else {
        return;
    };
    let mut beyond = (Vec3::ZERO, Vec3::ZERO);
    let parts = match tables::parts(piece) {
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
