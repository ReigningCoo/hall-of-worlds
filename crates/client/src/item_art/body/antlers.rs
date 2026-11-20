//! Antlers (`shared::body::Parts::antlers`) on a measured head: from the top of the
//! skull each a beam sweeping up and back, then forwards at the top, with its tines
//! (`Parts::tines`) spread along it pointing forwards and up, and a brow tine over the
//! face when it has any.

use bevy::prelude::*;
use shared::body::{HeadShape, Layout, Side};

use super::{HeadAxes, Triangle, crown, limb};

/// The beam's bends, as shares of its length: out to its side, up, and forwards.
const BEAM: [Vec3; 5] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.2, 0.35, -0.25),
    Vec3::new(0.35, 0.7, -0.32),
    Vec3::new(0.42, 0.95, -0.12),
    Vec3::new(0.4, 1.05, 0.1),
];

/// The antlers, if the head has any.
pub(super) fn build(
    triangles: &mut Vec<Triangle>,
    layout: &Layout,
    shape: &HeadShape,
    axes: &HeadAxes,
) {
    let length = layout.parts.antlers as f32;
    if length <= 0.0 {
        return;
    }
    let skull = crown(shape);
    let (along, top, half) = (skull.along as f32, skull.back as f32, skull.half as f32);
    let thick = (length * 0.045).max(5.0);
    let tines = layout.parts.tines.clamp(0, 8) as usize;
    for side in Side::BOTH {
        let out = side.sign() as f32;
        let place = |share: Vec3| {
            axes.at(
                along + share.z * length,
                top * 0.9 + share.y * length,
                out * (half * 0.5 + share.x * length),
            )
        };
        let beam: Vec<(Vec3, f32)> = BEAM
            .iter()
            .enumerate()
            .map(|(at, &share)| {
                let t = at as f32 / (BEAM.len() - 1) as f32;
                (place(share), thick * (1.0 - 0.7 * t))
            })
            .collect();
        limb(triangles, &beam);
        for tine in 0..tines {
            // The first low over the brow, the rest spread up the beam.
            let t = if tine == 0 {
                0.12
            } else {
                0.35 + 0.55 * tine as f32 / tines as f32
            };
            let share = along_beam(t);
            let (root, long) = (place(share), length * if tine == 0 { 0.3 } else { 0.22 });
            let tip = place(share + Vec3::new(0.04, 0.12, 0.0)) + forwards(axes) * long;
            let middle = root.lerp(tip, 0.5) + Vec3::Y * long * 0.12;
            limb(
                triangles,
                &[(root, thick * 0.55), (middle, thick * 0.4), (tip, 2.0)],
            );
        }
    }
}

/// The point this share of the way along the beam, as shares of its length.
fn along_beam(t: f32) -> Vec3 {
    let scaled = t.clamp(0.0, 1.0) * (BEAM.len() - 1) as f32;
    let at = (scaled as usize).min(BEAM.len() - 2);
    BEAM[at].lerp(BEAM[at + 1], scaled - at as f32)
}

/// The head's forwards, level.
fn forwards(axes: &HeadAxes) -> Vec3 {
    let ahead = axes.at(1.0, 0.0, 0.0) - axes.at(0.0, 0.0, 0.0);
    Vec3::new(0.0, ahead.y.max(0.0) * 0.3, ahead.z).normalize_or(Vec3::Z)
}
