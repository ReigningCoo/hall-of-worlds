//! A coat beyond its colour (`shared::body::Coat`): a sheep's thick fleece, a lumpy
//! shell round the trunk thinner under the belly, drawn as skin so markings fall on
//! it; and a yak's skirt, long hair hanging from its flanks nearly to the ground with a
//! ragged hem,
//! drawn with the mane and tail hair.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::body::{Coat, Layout, RINGS};

use super::trunk::{self, Axes};
use super::{ROUND, Triangle, loft};

/// A fleece round the trunk, if it has one.
pub(super) fn fleece(triangles: &mut Vec<Triangle>, layout: &Layout) {
    let thick = layout.parts.coat_length as f32;
    if layout.parts.coat_shape != Coat::Fleece || thick <= 0.0 {
        return;
    }
    let axes = Axes::of(layout);
    let rings: Vec<Vec<Vec3>> = layout
        .spine
        .rings
        .iter()
        .enumerate()
        .map(|(at, ring)| {
            let middle = trunk::middle_of(&axes, ring);
            let (across, deep) = (ring.half as f32, (ring.belly + ring.back) as f32 / 2.0);
            // Thinner at the rump's tip and where the neck leaves it.
            let end = if at == 0 || at == RINGS - 1 { 0.4 } else { 1.0 };
            (0..ROUND)
                .map(|point| {
                    let angle = point as f32 / ROUND as f32 * TAU;
                    let (cos, sin) = (angle.cos(), angle.sin());
                    // Lumps, from ring to ring and round each; thin underneath.
                    let lump = if (point + at) % 2 == 0 { 1.0 } else { 0.6 };
                    let under = if sin > 0.5 { 0.35 } else { 1.0 };
                    let out = thick * lump * under * end;
                    middle + Vec3::X * (across + out) * cos + axes.front * (deep + out) * sin
                })
                .collect()
        })
        .collect();
    loft(triangles, &rings);
}

/// Steps along the flank for the skirt.
const STEPS: usize = 14;

/// A skirt of long hair down each flank, if it has one: both faces drawn, as a mane.
pub(in crate::item_art) fn skirt(triangles: &mut Vec<Triangle>, layout: &Layout) {
    let hang = layout.parts.coat_length as f32;
    if layout.parts.coat_shape != Coat::Skirt || hang <= 0.0 {
        return;
    }
    let axes = Axes::of(layout);
    let rings = &layout.spine.rings;
    let (first, last) = (rings[1].along as f32, rings[RINGS - 2].along as f32);
    for side in [-1.0, 1.0] {
        let rows: Vec<[Vec3; 2]> = (0..=STEPS)
            .map(|step| {
                let along = first + (last - first) * step as f32 / STEPS as f32;
                let ring = trunk::ring_at(rings, along);
                let middle =
                    trunk::middle_of(&axes, &ring) + axes.along * (along - ring.along as f32);
                let deep = (ring.belly + ring.back) as f32 / 2.0;
                // From low on the flank, out a little, down as far as it hangs.
                let top =
                    middle + Vec3::X * side * ring.half as f32 * 0.95 + axes.front * deep * 0.3;
                // A ragged hem: locks of two lengths in turn.
                let long = if step % 2 == 0 { hang } else { hang * 0.85 };
                let bottom = top + Vec3::X * side * hang * 0.15 - Vec3::Y * long;
                [top, Vec3::new(bottom.x, bottom.y.max(60.0), bottom.z)]
            })
            .collect();
        for pair in rows.windows(2) {
            let ([a, a_low], [b, b_low]) = (pair[0], pair[1]);
            for [p, q, r] in [[a, b, b_low], [a, b_low, a_low]] {
                triangles.push([p, q, r]);
                triangles.push([p, r, q]);
            }
        }
    }
}
