//! The crops standing anywhere seen: a vegetable plant's (`VegetablePlant`),
//! in farms' fields and the wilderness, by how it has grown.

use bevy::prelude::*;
use shared::dungeon::{Level, Sight};
use shared::vegetable::Vegetables;

use super::super::CELL_SIZE;
use super::super::mesh::Builder;
use super::super::town::{self};
use super::{CELL_PIXELS, CROP_ROWS, CROP_STAGES, CROP_TILES, HALF_CELL, Stage};

/// The crops of every vegetable plant on `level` anyone can see, each on its
/// cell's `ground`, grown to game minute `now` and `shift` stages on (the
/// debug menu's): nothing is drawn of them from memory, as they will have
/// grown since.
pub fn build_crops(
    level: &Level,
    (sight, remembered): (&Sight, &Sight),
    (now, shift): (u64, u32),
    ground: &dyn Fn(Vec3) -> f32,
) -> Builder {
    let mut crops = Builder::default();
    let vegetables = Vegetables::built_in();
    for ((x, y), plant) in level.vegetable_plants() {
        if !sight.is_seen(x, y) || !remembered.is_seen(x, y) {
            continue;
        }
        let Some(def) = vegetables.get(&plant.vegetable) else {
            continue;
        };
        let stage = (u32::from(plant.stage(def, now)) + shift) % CROP_STAGES as u32;
        let Some((tile, pixels)) = Stage::of_plant(stage as u8).crop() else {
            continue;
        };
        let from = crops.positions.len();
        cell_crops(&mut crops, town::cell_centre(x, y), tile, pixels);
        crops.drape(from, ground);
    }
    crops
}

/// A `#` of crops in a cell: two rows each way.
pub(super) fn cell_crops(crops: &mut Builder, centre: Vec3, tile: f32, pixels: f32) {
    let top = pixels / CELL_PIXELS;
    let up = Vec3::Y * top * CELL_SIZE;
    let uv = |u: f32, v: f32| [(tile + u) / CROP_TILES, v];
    let uvs = [
        uv(0.0, 1.0),
        uv(1.0, 1.0),
        uv(1.0, 1.0 - top),
        uv(0.0, 1.0 - top),
    ];
    let colours = [Vec3::ONE; 4];
    for row in CROP_ROWS {
        let offset = row * CELL_SIZE;
        for (across, normal, along) in [
            (Vec3::X * offset, Vec3::X, Vec3::Z),
            (Vec3::Z * offset, Vec3::Z, Vec3::X),
        ] {
            let middle = centre + across;
            let (a, b) = (middle - along * HALF_CELL, middle + along * HALF_CELL);
            crops.free([a, b, b + up, a + up], normal, uvs, colours);
        }
    }
}
