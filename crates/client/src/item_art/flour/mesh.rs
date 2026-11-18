//! A flour sack's shapes as rings of rounded rectangles stacked up the sack
//! (owner, 2026-11-18): sacks are rectangular, their sides pushed outwards by
//! what is in them, so they read as `()` rather than `O`.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::TAU;

/// Points round each ring.
const AROUND: usize = 32;
/// How square the rings are: 2 is an ellipse, more is a squarer rectangle.
const SQUARENESS: f32 = 5.0;
/// The sack's half width and half depth, for a ring of scale 1.
pub(super) const HALF_WIDTH: f32 = 0.62;
pub(super) const HALF_DEPTH: f32 = 0.42;

/// One ring: its height, and how big it is against the sack's
/// `HALF_WIDTH` and `HALF_DEPTH` (0 closes the shape to a point).
pub(super) type Ring = (f32, f32);

/// The body: a flat-ish slumped bottom, sides bowed out a little in the
/// middle, open at the top under the cuff.
pub(super) const BODY: [Ring; 10] = [
    (-1.0, 0.0),
    (-0.99, 0.7),
    (-0.95, 0.96),
    (-0.88, 1.06),
    (-0.7, 1.08),
    (-0.4, 1.09),
    (-0.1, 1.07),
    (0.12, 1.03),
    (0.25, 1.0),
    (0.32, 0.98),
];

/// The top rolled down into a thick cuff, its lip turning in at the mouth.
pub(super) const CUFF: [Ring; 6] = [
    (0.18, 1.02),
    (0.24, 1.13),
    (0.34, 1.18),
    (0.44, 1.13),
    (0.5, 1.0),
    (0.49, 0.9),
];

/// The flour heaped above the rim.
pub(super) const HEAP: [Ring; 6] = [
    (0.42, 0.92),
    (0.55, 0.86),
    (0.67, 0.66),
    (0.75, 0.4),
    (0.79, 0.16),
    (0.8, 0.0),
];

/// A smooth mesh through these rings, bottom to top.
pub(super) fn rings_mesh(rings: &[Ring]) -> Mesh {
    let mut positions = Vec::with_capacity(rings.len() * AROUND);
    for &(y, scale) in rings {
        for step in 0..AROUND {
            let (sin, cos) = (step as f32 / AROUND as f32 * TAU).sin_cos();
            let x = squared(cos) * HALF_WIDTH * scale;
            let z = squared(sin) * HALF_DEPTH * scale;
            positions.push([x, y, z]);
        }
    }
    let mut indices = Vec::new();
    for ring in 0..rings.len().saturating_sub(1) {
        for step in 0..AROUND {
            let next = (step + 1) % AROUND;
            let a = (ring * AROUND + step) as u32;
            let b = (ring * AROUND + next) as u32;
            let c = ((ring + 1) * AROUND + step) as u32;
            let d = ((ring + 1) * AROUND + next) as u32;
            // Facing outwards: round the ring is towards +z from +x.
            indices.extend([a, c, b, b, c, d]);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(indices))
    .with_computed_smooth_normals()
}

/// A cosine or sine pushed towards ±1, for a rounded rectangle.
fn squared(value: f32) -> f32 {
    value.signum() * value.abs().powf(2.0 / SQUARENESS)
}
