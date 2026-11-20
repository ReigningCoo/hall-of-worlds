//! A cock's sickle feathers (`shared::body::Tail::Sickle`): from the rump's tip, a
//! few long narrow feathers arching up and back and falling past their root, spread
//! a little across, drawn apart with the wings' feathers, both faces as the vanes are
//! thin.

use std::f32::consts::PI;

use bevy::prelude::*;
use shared::body::Layout;

use super::Triangle;
use super::trunk::{self, Axes};

/// Steps along each feather.
const STEPS: usize = 8;

/// Each feather's place across (in its widths), and its length as a share.
const FEATHERS: [(f32, f32); 5] = [
    (-1.6, 0.8),
    (-0.8, 0.95),
    (0.0, 1.0),
    (0.8, 0.95),
    (1.6, 0.8),
];

/// The sickles, the longest `length` along its curve.
pub(super) fn build(triangles: &mut Vec<Triangle>, layout: &Layout, length: f32) {
    let axes = Axes::of(layout);
    let root = trunk::middle_of(&axes, &layout.spine.rings[0]);
    let wide = length * 0.07;
    for (across, share) in FEATHERS {
        let long = length * share;
        // Up and back, over the top and down below where it began.
        let at = |t: f32| {
            root + Vec3::new(
                across * wide * (0.4 + t),
                long * (0.55 * (t * PI).sin() - 0.3 * t),
                -long * 0.75 * t,
            )
        };
        let edge: Vec<(Vec3, Vec3)> = (0..=STEPS)
            .map(|step| {
                let t = step as f32 / STEPS as f32;
                let half = wide * (1.0 - t * 0.7) / 2.0;
                let (p, next) = (at(t), at((t + 0.05).min(1.0)));
                // Across the feather: square to its run, in the upright plane.
                let run = (next - p).normalize_or(Vec3::NEG_Z);
                let side = Vec3::new(0.0, -run.z, run.y).normalize_or(Vec3::Y);
                (p + side * half, p - side * half)
            })
            .collect();
        for pair in edge.windows(2) {
            let [(a, b), (c, d)] = [pair[0], pair[1]];
            for [p, q, r] in [[a, c, d], [a, d, b]] {
                triangles.push([p, q, r]);
                triangles.push([p, r, q]);
            }
        }
    }
}
