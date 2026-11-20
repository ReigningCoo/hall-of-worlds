//! The shapes bodies are built from: skin lofted over rings, round limbs along a
//! path, and stretched balls.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use super::{HEAD_RINGS, ROUND, Triangle};

/// Skin over rings of equally many points, with the ends closed.
pub(in crate::item_art) fn loft(triangles: &mut Vec<Triangle>, rings: &[Vec<Vec3>]) {
    let middle = |ring: &[Vec3]| ring.iter().copied().sum::<Vec3>() / ring.len() as f32;
    let mut face = |a: Vec3, b: Vec3, c: Vec3, out: Vec3| {
        let forward = (b - a).cross(c - a).dot(out) >= 0.0;
        triangles.push(if forward { [a, b, c] } else { [a, c, b] });
    };
    for pair in rings.windows(2) {
        let centre = (middle(&pair[0]) + middle(&pair[1])) / 2.0;
        let points = pair[0].len();
        for at in 0..points {
            let next = (at + 1) % points;
            let corners = [pair[0][at], pair[0][next], pair[1][next], pair[1][at]];
            let out = corners.iter().copied().sum::<Vec3>() / 4.0 - centre;
            face(corners[0], corners[1], corners[2], out);
            face(corners[0], corners[2], corners[3], out);
        }
    }
    for (end, inner) in [(0, 1), (rings.len() - 1, rings.len() - 2)] {
        let centre = middle(&rings[end]);
        let out = centre - middle(&rings[inner]);
        for at in 0..rings[end].len() {
            let next = (at + 1) % rings[end].len();
            face(centre, rings[end][at], rings[end][next], out);
        }
    }
}

/// A flat ring round a level, half widths across and front to back.
fn level(y: f32, (across, deep): (f32, f32), forward: f32) -> Vec<Vec3> {
    (0..ROUND)
        .map(|at| {
            let angle = at as f32 / ROUND as f32 * TAU;
            Vec3::new(across * angle.cos(), y, forward + deep * angle.sin())
        })
        .collect()
}

/// A round limb from one point to another, its radius changing along it.
pub(in crate::item_art) fn limb(triangles: &mut Vec<Triangle>, path: &[(Vec3, f32)]) {
    let rings: Vec<Vec<Vec3>> = (0..path.len())
        .map(|at| {
            let (place, radius) = path[at];
            let along = (path[(at + 1).min(path.len() - 1)].0 - path[at.saturating_sub(1)].0)
                .normalize_or(Vec3::Y);
            // Measured the same way at every ring, so the skin doesn't twist.
            let across = along.cross(Vec3::Z).normalize_or(Vec3::X);
            let other = along.cross(across);
            (0..ROUND)
                .map(|point| {
                    let angle = point as f32 / ROUND as f32 * TAU;
                    place + (across * angle.cos() + other * angle.sin()) * radius
                })
                .collect()
        })
        .collect();
    loft(triangles, &rings);
}

/// A ball stretched to these half sizes.
pub(in crate::item_art) fn blob(triangles: &mut Vec<Triangle>, centre: Vec3, half: Vec3) {
    let rings: Vec<Vec<Vec3>> = (0..=HEAD_RINGS)
        .map(|ring| {
            let down = ring as f32 / HEAD_RINGS as f32 * PI;
            let y = centre.y + half.y * down.cos();
            let shrink = down.sin().max(0.02);
            level(y, (half.x * shrink, half.z * shrink), centre.z)
                .into_iter()
                .map(|p| Vec3::new(centre.x + p.x, p.y, p.z))
                .collect()
        })
        .collect();
    loft(triangles, &rings);
}
