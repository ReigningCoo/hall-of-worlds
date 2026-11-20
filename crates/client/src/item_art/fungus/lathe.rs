//! A lathe: a profile of points (distance out, height) turned round the up
//! axis into one smooth mesh with vertex colours. Every round part of a fungus
//! is one: caps with their gills, stems, rings, cups, balls, clubs. Each point
//! round can be pushed in or out (wavy rims, lumps, a bracket's fan) and
//! coloured on its own (gills, zones, pits).

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::TAU;

use super::paint::linear;

/// Points round each row.
pub(super) const AROUND: usize = 48;

/// One point round a row: how far out against the profile's distance (1 as
/// it is), and its colour (sRGB parts).
#[derive(Clone, Copy)]
pub(super) struct Round {
    pub(super) out: f32,
    pub(super) colour: Vec3,
}

/// The mesh turned from `profile` (x out from the axis, y up), from its first
/// row to its last; `closed` joins the last row back to the first (a ring
/// round a stem), otherwise a row of no width is a pole. `round(row, i)` says
/// how the `i`th of `AROUND` points round row `row` is pushed and coloured.
pub(super) fn lathe(profile: &[Vec2], closed: bool, round: impl Fn(usize, usize) -> Round) -> Mesh {
    let mut positions = Vec::with_capacity(profile.len() * AROUND);
    let mut colours = Vec::with_capacity(profile.len() * AROUND);
    for (row, point) in profile.iter().enumerate() {
        for i in 0..AROUND {
            let a = i as f32 / AROUND as f32 * TAU;
            let Round { out, colour } = round(row, i);
            let r = point.x * out;
            positions.push(Vec3::new(r * a.cos(), point.y, r * a.sin()));
            colours.push(linear(colour));
        }
    }
    let rows = profile.len();
    let at = |r: usize, c: usize| ((r % rows) * AROUND + c % AROUND) as u32;
    let mut indices = Vec::new();
    let bands = if closed { rows } else { rows - 1 };
    for row in 0..bands {
        for i in 0..AROUND {
            let (a, b, c, d) = (
                at(row, i),
                at(row, i + 1),
                at(row + 1, i + 1),
                at(row + 1, i),
            );
            indices.extend([a, b, c, a, c, d]);
        }
    }
    outward(&positions, &mut indices);
    let normals = normals(&positions, &indices);
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices))
}

/// Turns every triangle round if the whole faces in rather than out.
fn outward(positions: &[Vec3], indices: &mut [u32]) {
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
}

/// Smooth normals, each point's the sum of its triangles'; the points of a
/// pole share one.
fn normals(positions: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for t in indices.chunks(3) {
        let [a, b, c] = [0, 1, 2].map(|k| positions[t[k] as usize]);
        let face = (b - a).cross(c - a);
        t.iter().for_each(|&k| normals[k as usize] += face);
    }
    for span in (0..positions.len()).step_by(AROUND) {
        let row = span..span + AROUND;
        let first = positions[span];
        if positions[row.clone()]
            .iter()
            .all(|p| p.distance(first) < 1e-5)
        {
            let sum: Vec3 = normals[row.clone()].iter().sum();
            normals[row].iter_mut().for_each(|n| *n = sum);
        }
    }
    normals
        .into_iter()
        .map(|n| n.normalize_or(Vec3::Y))
        .collect()
}

/// The angle of the `i`th point round.
pub(super) fn angle(i: usize) -> f32 {
    i as f32 / AROUND as f32 * TAU
}
