//! Chairs and benches as placeholder boxes until their models come: each
//! cell of a piece a block of the furniture picture in its wood or stone, to
//! a seat's height.

use bevy::prelude::*;
use shared::furniture::Piece;

use super::super::CELL_SIZE;
use super::super::mesh::{MEMORY_SHADE, Surface, corner_light, with_ambient};
use super::super::walls::{Layer, Layers, Scene};
use super::{atlas, colour};

/// How high a seat stands.
const SEAT_HEIGHT: f32 = CELL_SIZE * 0.23;
/// How far a piece stands in from the edges of its cells.
const INSET: f32 = CELL_SIZE * 0.1;
/// How much darker a box's sides are than its top.
const SIDE_SHADE: f32 = 0.65;

/// The part of a seat at `(x, y)`: lit if in sight, else grey.
pub(super) fn build_placeholder(
    layers: &mut Layers,
    scene: &Scene,
    piece: &Piece,
    (x, y): (i64, i64),
) {
    let in_sight = scene.sight.is_seen(x, y);
    let layer = if in_sight {
        Layer::Solid
    } else {
        Layer::Memory
    };
    let tint = colour::of(piece.material);
    let tile = atlas::tile(piece.material);
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
    let centre = Vec3::new(x as f32 * CELL_SIZE, 0.0, y as f32 * CELL_SIZE);
    let (low, high) = (
        centre + Vec3::new(-west, 0.0, -north),
        centre + Vec3::new(east, SEAT_HEIGHT, south),
    );
    let builder = layers.builder(layer, Surface::Furniture);
    let uv = |u: f32, v: f32| atlas::uv(tile, u, v);
    let at = Vec3::new;
    let (l, h) = (low, high);
    let top = [
        at(l.x, h.y, l.z),
        at(h.x, h.y, l.z),
        at(h.x, h.y, h.z),
        at(l.x, h.y, h.z),
    ];
    let uvs = [uv(0.0, 0.0), uv(1.0, 0.0), uv(1.0, 1.0), uv(0.0, 1.0)];
    builder.free(top, Vec3::Y, uvs, top.map(|p| lit(p, 1.0)));
    // Only the outside of the piece has sides.
    let sides = [
        (Vec3::X, [at(h.x, l.y, l.z), at(h.x, l.y, h.z)], (1, 0)),
        (Vec3::NEG_X, [at(l.x, l.y, l.z), at(l.x, l.y, h.z)], (-1, 0)),
        (Vec3::Z, [at(l.x, l.y, h.z), at(h.x, l.y, h.z)], (0, 1)),
        (Vec3::NEG_Z, [at(l.x, l.y, l.z), at(h.x, l.y, l.z)], (0, -1)),
    ];
    let up = Vec3::Y * SEAT_HEIGHT;
    let side_uvs = [uv(0.0, 1.0), uv(1.0, 1.0), uv(1.0, 0.7), uv(0.0, 0.7)];
    for (normal, [a, b], (dx, dy)) in sides {
        if piece.covers((x + dx, y + dy)) {
            continue;
        }
        let corners = [a, b, b + up, a + up];
        let colours = corners.map(|p| lit(p, SIDE_SHADE));
        builder.free(corners, normal, side_uvs, colours);
    }
}
