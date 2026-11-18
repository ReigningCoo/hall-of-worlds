//! `Ring`s along a path made into one smooth tube: roots, stalks, pods, gourds
//! and spears are all one. Each ring has its own size and colour (in the
//! mesh's vertex colours), and the whole may be ribbed or striped round.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::TAU;

use super::super::herb::kit::Kit;

/// Points round each ring.
const AROUND: usize = 28;

/// One ring of a tube: its middle, its half sizes across (sideways, then up)
/// and its colour (sRGB parts).
#[derive(Clone, Copy)]
pub(super) struct Ring {
    pub(super) at: Vec3,
    pub(super) half: Vec2,
    pub(super) colour: Vec3,
}

/// What changes round a tube.
#[derive(Clone, Copy, Default)]
pub(super) struct Round {
    /// Ribs round it, and how deep (a share of its size).
    pub(super) ribs: u32,
    pub(super) depth: f32,
    /// Stripes along it between the ribs: their colour, and how much of the
    /// way round they cover (0 to 1).
    pub(super) stripes: Option<(Vec3, f32)>,
}

/// `steps + 1` rings along a path from `t` 0 to 1: where, how big and what
/// colour each is.
pub(super) fn rings(
    steps: usize,
    path: impl Fn(f32) -> Vec3,
    half: impl Fn(f32) -> Vec2,
    colour: impl Fn(f32) -> Vec3,
) -> Vec<Ring> {
    (0..=steps)
        .map(|k| {
            let t = k as f32 / steps as f32;
            Ring {
                at: path(t),
                half: half(t),
                colour: colour(t),
            }
        })
        .collect()
}

/// A rounded end: 0 at `t` 0, rising like a circle to 1 at `t` = `k`.
pub(super) fn cap(t: f32, k: f32) -> f32 {
    let s = (t / k).clamp(0.0, 1.0);
    (1.0 - (1.0 - s) * (1.0 - s)).sqrt()
}

/// The tube through `rings`, added to the kit, a little shiny if `shine`.
pub(super) fn tube(kit: &mut Kit, rings: &[Ring], round: Round, shine: bool) {
    if rings.len() < 2 {
        return;
    }
    let mut positions = Vec::new();
    let mut colours = Vec::new();
    for (k, ring) in rings.iter().enumerate() {
        let before = rings[k.saturating_sub(1)].at;
        let after = rings[(k + 1).min(rings.len() - 1)].at;
        let along = (after - before).normalize_or(Vec3::X);
        let side = along.cross(Vec3::Y).normalize_or(Vec3::Z);
        let up = side.cross(along);
        for i in 0..AROUND {
            let a = i as f32 / AROUND as f32 * TAU;
            let rib = (a * round.ribs as f32).cos();
            let size = 1.0 + round.depth * rib;
            let out = side * a.cos() * ring.half.x + up * a.sin() * ring.half.y;
            positions.push(ring.at + out * size);
            let colour = match round.stripes {
                Some((stripe, share)) if rib < share * 2.0 - 1.0 => stripe,
                _ => ring.colour,
            };
            colours.push(linear(colour));
        }
    }
    let mesh = closed(positions, colours, rings.len());
    let white = kit.material(Color::WHITE, shine);
    kit.mesh(mesh, &white, Transform::IDENTITY);
}

/// sRGB parts as a linear vertex colour.
pub(super) fn linear(colour: Vec3) -> [f32; 4] {
    let c = colour.clamp(Vec3::ZERO, Vec3::ONE);
    Color::srgb(c.x, c.y, c.z).to_linear().to_f32_array()
}

/// The triangles between `rows` rings of `AROUND` points, facing out, with
/// smooth normals; a ring shrunk to a point is a pole.
fn closed(positions: Vec<Vec3>, colours: Vec<[f32; 4]>, rows: usize) -> Mesh {
    let at = |r: usize, c: usize| (r * AROUND + c % AROUND) as u32;
    let mut indices = Vec::new();
    for row in 0..rows - 1 {
        for i in 0..AROUND {
            let (a, b, c, d) = (
                at(row, i),
                at(row, i + 1),
                at(row + 1, i + 1),
                at(row + 1, i),
            );
            indices.extend([a, c, b, a, d, c]);
        }
    }
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
    for row in 0..rows {
        let span = row * AROUND..(row + 1) * AROUND;
        let first = positions[span.start];
        if positions[span.clone()]
            .iter()
            .all(|p| p.distance(first) < 1e-5)
        {
            let sum: Vec3 = normals[span.clone()].iter().sum();
            normals[span].iter_mut().for_each(|n| *n = sum);
        }
    }
    let normals: Vec<Vec3> = normals
        .into_iter()
        .map(|n| n.normalize_or(Vec3::Y))
        .collect();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices))
}
