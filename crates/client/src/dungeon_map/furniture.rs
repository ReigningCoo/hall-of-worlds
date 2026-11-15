//! Furniture (`shared::furniture`) as placeholder boxes for now: each cell of
//! a piece a block of the floor's picture, brown for wood and grey for stone,
//! to a table top's height or a seat's. Real models come later.

use bevy::prelude::*;
use shared::furniture::Material;

use super::CELL_SIZE;
use super::mesh::{MEMORY_SHADE, Surface, corner_light, with_ambient};
use super::walls::{Layer, Layers, Scene};

/// How high a table's top and a seat stand.
const TABLE_HEIGHT: f32 = CELL_SIZE * 0.45;
const SEAT_HEIGHT: f32 = CELL_SIZE * 0.28;
/// How far a piece stands in from the edges of its cells.
const INSET: f32 = CELL_SIZE * 0.1;
/// Tints on the floor's picture.
const WOOD_TINT: Vec3 = Vec3::new(0.75, 0.5, 0.3);
const STONE_TINT: Vec3 = Vec3::new(0.85, 0.85, 0.8);
/// How much darker a box's sides are than its top.
const SIDE_SHADE: f32 = 0.65;

/// The part of any piece of furniture at `(x, y)`: lit if in sight, else grey.
pub(super) fn build_furniture(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) {
    let Some(piece) = scene.level.furniture_at(x, y) else {
        return;
    };
    let height = if piece.furniture.is_table() {
        TABLE_HEIGHT
    } else {
        SEAT_HEIGHT
    };
    let in_sight = scene.sight.is_seen(x, y);
    let layer = if in_sight {
        Layer::Solid
    } else {
        Layer::Memory
    };
    let tint = match piece.material {
        Material::Wood(_) => WOOD_TINT,
        Material::Stone(_) => STONE_TINT,
    };
    let lit = |at: Vec3, shade: f32| {
        if !in_sight {
            return Vec3::splat(MEMORY_SHADE * shade);
        }
        let light = corner_light(scene.level, scene.light, at).unwrap_or(scene.light.at(x, y));
        with_ambient(light) * tint * shade
    };
    // How far the box reaches from the cell's middle each way: to the edge
    // where the piece goes on, else in by `INSET`.
    let half = CELL_SIZE / 2.0;
    let reach = |dx: i64, dy: i64| {
        if piece.covers((x + dx, y + dy)) {
            half
        } else {
            half - INSET
        }
    };
    let (east, west, south, north) = (reach(1, 0), reach(-1, 0), reach(0, 1), reach(0, -1));
    let middle = Vec3::new(
        x as f32 * CELL_SIZE + (east - west) / 2.0,
        height,
        y as f32 * CELL_SIZE + (south - north) / 2.0,
    );
    let (half_x, half_z) = ((east + west) / 2.0, (south + north) / 2.0);
    let builder = layers.builder(layer, Surface::Floor);
    builder.quad(middle, Vec3::Y, Vec3::NEG_Z, (half_x, half_z), |at| {
        lit(at, 1.0)
    });
    let half_y = height / 2.0;
    let sides = [
        (Vec3::X, half_x, half_z, (1, 0)),
        (Vec3::NEG_X, half_x, half_z, (-1, 0)),
        (Vec3::Z, half_z, half_x, (0, 1)),
        (Vec3::NEG_Z, half_z, half_x, (0, -1)),
    ];
    // Only the outside of the piece has sides.
    for (normal, out, across, (dx, dy)) in sides {
        if piece.covers((x + dx, y + dy)) {
            continue;
        }
        let centre = Vec3::new(middle.x, half_y, middle.z) + normal * out;
        builder.quad(centre, normal, Vec3::Y, (across, half_y), |at| {
            lit(at, SIDE_SHADE)
        });
    }
}
