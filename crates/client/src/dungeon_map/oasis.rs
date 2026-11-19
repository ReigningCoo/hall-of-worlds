//! What only oases have: grass round their pools. The trees on it are
//! `trees`' models.

use bevy::prelude::*;
use shared::dungeon::Cell;

use super::CELL_SIZE;
use super::town::{self, GRASS_TILE};
use super::walls::{Layers, Scene};

const HALF_CELL: f32 = CELL_SIZE / 2.0;

/// The grass under a grass or tree cell. False for any other cell, which is
/// drawn as usual.
pub fn build_grass_or_tree(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) -> bool {
    let cell = scene.level.cell(x, y);
    if !matches!(cell, Cell::Grass | Cell::Tree) {
        return false;
    }
    let layer = town::flat_layer(scene, x, y);
    let centre = town::cell_centre(x, y);
    let patch = Rect::from_corners(Vec2::splat(-HALF_CELL), Vec2::splat(HALF_CELL));
    town::flat_patch(layers, scene, layer, centre, patch, GRASS_TILE);
    true
}
