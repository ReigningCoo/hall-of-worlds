//! Bananas and pods: long fruits built as a chain of blobs along a curve.

use bevy::prelude::*;

use super::super::herb::kit::Kit;
use super::colour::{Colours, colour};
use super::group::PodKind;
use super::variety::Variety;

/// Points along an arc bending `bend` radians in all, `length` long, lying
/// across the picture, with the way along at each.
fn arc(length: f32, bend: f32, steps: usize) -> Vec<(Vec3, f32)> {
    (0..=steps)
        .map(|k| {
            let t = k as f32 / steps as f32 - 0.5;
            if bend.abs() < 1e-3 {
                return (Vec3::new(t * length, 0.0, 0.0), 0.0);
            }
            let radius = length / bend;
            let a = t * bend;
            let at = Vec3::new(
                a.sin() * radius,
                0.0,
                (1.0 - a.cos()) * radius - radius * 0.1,
            );
            (at, -a)
        })
        .collect()
}

/// One banana: curved, thick in the middle, a dark tip and its stalk.
pub(super) fn banana(colours: &Colours, variety: &Variety, room: Vec2, kit: &mut Kit) {
    let skin = kit.material(colour(colours.skin), true);
    let dark = kit.material(colour(colours.stalk * 0.6), false);
    let size = room.min_element() * variety.size;
    let points = arc(size * 1.6, 1.1 + variety.wobble(0, 30) * 0.2, 8);
    let last = points.len() - 1;
    for (k, &(at, yaw)) in points.iter().enumerate() {
        let t = k as f32 / last as f32;
        let thick = size * (0.1 + (t * std::f32::consts::PI).sin() * 0.12);
        let at = at + Vec3::Y * thick;
        kit.blob(
            &skin,
            at,
            Vec3::new(size * 0.16, thick * 0.9, thick),
            (yaw, 0.0),
        );
    }
    let (tip, _) = points[0];
    kit.blob(
        &dark,
        tip + Vec3::Y * size * 0.08,
        Vec3::splat(size * 0.05),
        (0.0, 0.0),
    );
    let (end, _) = points[last];
    let stalk = end + Vec3::new(size * 0.15, size * 0.12, -size * 0.06);
    kit.rod(&dark, end + Vec3::Y * size * 0.1, stalk, size * 0.045);
}

/// Pods: carob's flat curved ones, tamarind's lumpy ones, or a cacao pod.
pub(super) fn pod(kind: PodKind, colours: &Colours, variety: &Variety, room: Vec2, kit: &mut Kit) {
    let skin = kit.material(colour(colours.skin), false);
    let size = room.min_element() * variety.size;
    match kind {
        PodKind::Flat | PodKind::Beaded => {
            let count = if kind == PodKind::Flat { 1 } else { 2 };
            for n in 0..count {
                let shift = Vec3::Z * (n as f32 - (count as f32 - 1.0) / 2.0) * size * 0.45;
                let bend = 0.5 + variety.wobble(n, 31) * 0.3;
                let points = arc(size * 1.65, bend, 9);
                for (k, &(at, yaw)) in points.iter().enumerate() {
                    let lump = kind == PodKind::Beaded && k % 2 == 1;
                    let (wide, high) = match kind {
                        PodKind::Flat => (size * 0.2, size * 0.05),
                        _ if lump => (size * 0.15, size * 0.12),
                        _ => (size * 0.1, size * 0.08),
                    };
                    let at = at + shift + Vec3::Y * high;
                    kit.blob(&skin, at, Vec3::new(size * 0.13, high, wide), (yaw, 0.0));
                }
            }
        }
        PodKind::Ridged => {
            let half = Vec3::new(size * 0.92, size * 0.4, size * 0.42);
            let centre = Vec3::Y * half.y;
            kit.blob(&skin, centre, half, (0.0, 0.0));
            let ridge = kit.material(colour(colours.skin * 0.93), false);
            for k in 0..7 {
                let a = (k as f32 - 3.0) * 0.32;
                let d = Vec3::new(0.0, a.cos(), a.sin());
                let at = centre + half * d * 0.97;
                let scale = Vec3::new(half.x * 0.85, size * 0.05, size * 0.045);
                kit.blob(&ridge, at, scale, (0.0, 0.0));
            }
            for end in [-1.0, 1.0] {
                let at = centre + Vec3::X * end * half.x * 0.85;
                kit.cone(&skin, at, at + Vec3::X * end * size * 0.12, size * 0.12);
            }
        }
    }
}
