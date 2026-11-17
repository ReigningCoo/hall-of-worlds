//! `Surface`: rows of rings of points, closed round, made into triangles with
//! smooth normals facing out. Lathed bottles and the heart's pillow both are one.

use bevy::prelude::*;
use std::f32::consts::TAU;

use super::profiles::{Profile, Section};

/// Points round each ring.
pub(super) const AROUND: usize = 48;
/// How far above the shoulder a section takes to fade back to round.
const FADE: f32 = 6.0;

pub(super) struct Surface {
    /// Each row is a ring of `AROUND` points; a row of one point repeated is a pole.
    pub(super) rows: Vec<Vec<Vec3>>,
    /// How much each row has flat faces (1) rather than curves (0): no
    /// highlight is drawn on a flat face, which would light it all.
    pub(super) faceted: Vec<f32>,
}

impl Surface {
    /// A profile turned round the up axis, flattened or given sides below its
    /// shoulder.
    pub(super) fn lathe(profile: &Profile) -> Self {
        let mut rows = Vec::new();
        let mut faceted = Vec::new();
        for point in &profile.points {
            let k = (1.0 - (point.y - profile.shoulder) / FADE).clamp(0.0, 1.0);
            let ring = (0..AROUND)
                .map(|i| {
                    let angle = i as f32 / AROUND as f32 * TAU;
                    let (x, z) = across(profile.section, k, angle);
                    Vec3::new(x * point.x, point.y, z * point.x)
                })
                .collect();
            rows.push(ring);
            faceted.push(match profile.section {
                Section::Sides(_) => k,
                _ => 0.0,
            });
        }
        Self { rows, faceted }
    }

    /// Positions, normals (facing out, smooth) and the triangles' indices.
    pub(super) fn triangles(&self) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
        let positions: Vec<Vec3> = self.rows.iter().flatten().copied().collect();
        let mut indices = Vec::new();
        for row in 0..self.rows.len() - 1 {
            for i in 0..AROUND {
                let j = (i + 1) % AROUND;
                let at = |r: usize, c: usize| (r * AROUND + c) as u32;
                let (a, b) = (at(row, i), at(row, j));
                let (c, d) = (at(row + 1, j), at(row + 1, i));
                indices.extend([a, c, b, a, d, c]);
            }
        }
        // Outward if the volume it encloses comes out positive.
        let volume: f32 = indices
            .chunks(3)
            .map(|t| {
                let [a, b, c] = [0, 1, 2].map(|k| positions[t[k] as usize]);
                a.dot(b.cross(c))
            })
            .sum();
        if volume < 0.0 {
            indices.chunks_mut(3).for_each(|t| t.swap(1, 2));
        }
        let mut normals = vec![Vec3::ZERO; positions.len()];
        for t in indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|k| positions[t[k] as usize]);
            let face = (b - a).cross(c - a);
            t.iter().for_each(|&k| normals[k as usize] += face);
        }
        // A pole's points share one normal.
        for (row, ring) in self.rows.iter().enumerate() {
            if ring.iter().all(|p| p.distance(ring[0]) < 1e-4) {
                let span = row * AROUND..(row + 1) * AROUND;
                let sum: Vec3 = normals[span.clone()].iter().sum();
                normals[span].iter_mut().for_each(|n| *n = sum);
            }
        }
        let normals = normals
            .into_iter()
            .map(|n| n.normalize_or(Vec3::Y))
            .collect();
        (positions, normals, indices)
    }
}

/// Where a point round a ring of radius 1 is, `k` of the way from round to the
/// section, angle 0 at the front.
fn across(section: Section, k: f32, angle: f32) -> (f32, f32) {
    let (x, z) = (angle.sin(), angle.cos());
    match section {
        Section::Round => (x, z),
        Section::Flat(depth) => (x, z * (1.0 + (depth - 1.0) * k)),
        Section::Sides(sides) => {
            // Corners at radius 1, one at the front; flat faces between.
            let step = TAU / sides as f32;
            let within = angle.rem_euclid(step) - step / 2.0;
            let polygon = (step / 2.0).cos() / within.cos();
            let radius = 1.0 + (polygon - 1.0) * k;
            (x * radius, z * radius)
        }
    }
}
