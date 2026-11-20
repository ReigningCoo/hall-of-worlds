//! A bird's tail feathers spread in an upright fan (`shared::body::Tail::Fan`), as a
//! displaying turkey's: from the rump's tip, a half circle of feathers leaning a
//! little back, drawn apart with the wings' feathers, both faces as the vanes are
//! thin, their edge scalloped feather by feather.

use bevy::prelude::*;
use shared::body::Layout;

use super::Triangle;
use super::trunk::{self, Axes};

/// Feathers across the fan.
const FEATHERS: usize = 13;

/// The fan, as long as `length` from its root to its edge.
pub(super) fn build(triangles: &mut Vec<Triangle>, layout: &Layout, length: f32) {
    let axes = Axes::of(layout);
    let root = trunk::middle_of(&axes, &layout.spine.rings[0]);
    // Upright and leaning back, whichever way the back leans.
    let up = (Vec3::Y * 0.92 + Vec3::NEG_Z * 0.4).normalize();
    let spread = |share: f32, reach: f32| {
        // From one side, round over the top, to the other.
        let angle = (share - 0.5) * std::f32::consts::PI * 0.95;
        root + (Vec3::X * angle.sin() + up * angle.cos()) * reach
    };
    let mut push = |a: Vec3, b: Vec3, c: Vec3| {
        triangles.push([a, b, c]);
        triangles.push([a, c, b]);
    };
    for feather in 0..FEATHERS {
        let (from, to) = (
            feather as f32 / FEATHERS as f32,
            (feather + 1) as f32 / FEATHERS as f32,
        );
        let middle = (from + to) / 2.0;
        let (left, right, tip) = (
            spread(from, length * 0.92),
            spread(to, length * 0.92),
            spread(middle, length),
        );
        let inner = (spread(from, length * 0.2), spread(to, length * 0.2));
        push(inner.0, left, right);
        push(inner.0, right, inner.1);
        push(left, tip, right);
    }
}
