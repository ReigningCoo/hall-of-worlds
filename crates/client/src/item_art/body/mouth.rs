//! What is round the mouth of a measured head: a duck's flat bill past its nose
//! (`shared::body::Snout::Bill`; other snouts are in the head's rings) with any knob
//! on its base, and tusks up from the lower jaw.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::{HEAD_RINGS, HeadShape, Layout, Side, Snout};

use super::{HeadAxes, ROUND, Triangle, blob, limb, loft};

/// A bill and tusks, if the head has them.
pub(super) fn build(
    triangles: &mut Vec<Triangle>,
    layout: &Layout,
    shape: &HeadShape,
    axes: &HeadAxes,
) {
    let parts = layout.parts;
    if parts.snout == Snout::Bill && parts.snout_length > 0 {
        bill(
            triangles,
            parts.snout_length as f32,
            parts.knob as f32,
            shape,
            axes,
        );
    }
    if parts.tusks > 0 {
        tusks(triangles, parts.tusks as f32, shape, axes);
    }
}

/// Broad and flat from the face over any beak in the rings, reaching `length` past the
/// nose, widening a little and rounded at the tip, dipping a little; a knob this tall
/// on top of its base, if any.
fn bill(triangles: &mut Vec<Triangle>, length: f32, knob: f32, shape: &HeadShape, axes: &HeadAxes) {
    let (face, nose) = (shape.rings[HEAD_RINGS - 3], shape.rings[HEAD_RINGS - 1]);
    let start = face.along as f32;
    let length = nose.along as f32 + length - start;
    let mid = (nose.back + nose.belly) as f32 / 2.0;
    let wide = (face.half as f32 * 0.8).max(length * 0.22);
    let deep = length * 0.15;
    // Along, down, half width and half depth, as shares.
    let steps = [
        (0.0, 0.0, 1.0, 1.0),
        (0.5, 0.05, 1.1, 0.7),
        (0.9, 0.1, 0.95, 0.5),
        (1.0, 0.1, 0.5, 0.35),
    ];
    let rings: Vec<Vec<Vec3>> = steps
        .iter()
        .map(|&(along, down, across, thick)| {
            (0..ROUND)
                .map(|at| {
                    let angle = at as f32 / ROUND as f32 * TAU;
                    axes.at(
                        start + length * along,
                        mid - length * down + deep * thick * angle.sin(),
                        wide * across * angle.cos(),
                    )
                })
                .collect()
        })
        .collect();
    loft(triangles, &rings);
    if knob > 0.0 {
        // On the bill's top, where it leaves the face.
        let along = start + length * 0.3;
        blob(
            triangles,
            axes.at(along, mid + deep * 0.75 + knob * 0.35, 0.0),
            Vec3::new(knob * 0.7, knob * 0.75, knob * 0.9),
        );
    }
}

/// Up and out from the lower jaw near the front, curving back, as a boar's.
fn tusks(triangles: &mut Vec<Triangle>, tusk: f32, shape: &HeadShape, axes: &HeadAxes) {
    let jaw = shape.rings[HEAD_RINGS - 2];
    let (along, low, half) = (jaw.along as f32, -(jaw.belly as f32) * 0.4, jaw.half as f32);
    for side in Side::BOTH {
        let out = side.sign() as f32;
        let root = axes.at(along, low, out * half * 0.85);
        let middle = axes.at(
            along + tusk * 0.05,
            low + tusk * 0.5,
            out * (half + tusk * 0.2),
        );
        let tip = axes.at(
            along - tusk * 0.2,
            low + tusk * 0.9,
            out * (half + tusk * 0.3),
        );
        limb(
            triangles,
            &[(root, tusk * 0.12), (middle, tusk * 0.09), (tip, 2.0)],
        );
    }
}
