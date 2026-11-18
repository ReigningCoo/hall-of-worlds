//! One leaf as a thin sheet: its outline by `Blade`, cupped across, arched
//! along, ruffled at the edge if curly, its midrib in a paler colour. Seen
//! from both sides.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::PI;

use super::super::herb::kit::Kit;
use super::tube::linear;

const ALONG: usize = 18;
/// Points either side of the midrib.
const SIDE: usize = 6;

/// A leaf's outline.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Blade {
    /// Widest a little below the middle (spinach, kale, lettuce).
    Oval,
    /// Narrow and pointed (chicory, leek's leaves).
    Long,
    /// Round and wide at the base (sorrel).
    Kidney,
    /// Lobed teeth pointing back (dandelion).
    Toothed,
    /// Widest a third of the way (lamb's quarters).
    Diamond,
    /// Wide lobes at the base, pointed (water spinach).
    Arrow,
}

impl Blade {
    /// Its half width at `t` along it, 0 to 1.
    fn width(self, t: f32) -> f32 {
        match self {
            Self::Oval => (PI * t.powf(0.8)).sin().max(0.0).powf(0.8),
            Self::Long => (PI * t).sin().max(0.0).powf(0.7),
            Self::Kidney => (PI * (0.25 + 0.75 * t)).sin().max(0.0).powf(0.45),
            Self::Toothed => {
                let saw = (t * 6.0).fract();
                (PI * t.powf(0.6)).sin().max(0.0) * (0.45 + 0.55 * saw)
            }
            Self::Diamond => {
                let w = if t < 0.4 { t / 0.4 } else { (1.0 - t) / 0.6 };
                w.max(0.0).sqrt()
            }
            Self::Arrow => (1.0 - t).max(0.0).powf(0.9) * (0.5 + 0.5 * (t / 0.12).min(1.0)),
        }
    }
}

/// How one leaf lies and is coloured.
pub(super) struct Leaf {
    pub(super) blade: Blade,
    /// Where its base is, and which way it points (radians about the up axis
    /// from +x).
    pub(super) base: Vec3,
    pub(super) yaw: f32,
    pub(super) length: f32,
    /// Its widest, either side together.
    pub(super) width: f32,
    /// How far its edges curl up, against its width.
    pub(super) cup: f32,
    /// How much its edge ruffles (curly kale, lettuce), against its width.
    pub(super) ruffle: f32,
    /// How high its middle arches, against its length (below 0 it droops).
    pub(super) arch: f32,
    pub(super) colour: Vec3,
    pub(super) rib: Vec3,
}

/// Adds the leaf to the kit.
pub(super) fn leaf(kit: &mut Kit, leaf: &Leaf) {
    let rotation = Quat::from_rotation_y(leaf.yaw);
    let mut positions = Vec::new();
    let mut colours = Vec::new();
    let across = 2 * SIDE + 1;
    for k in 0..=ALONG {
        let t = k as f32 / ALONG as f32;
        let w = leaf.blade.width(t) * leaf.width / 2.0;
        for j in 0..across {
            let s = j as f32 / SIDE as f32 - 1.0;
            let wave = (t * 23.0 + s * 4.0).sin() * leaf.ruffle * w * s * s;
            let y = leaf.cup * w * s * s + leaf.arch * leaf.length * (PI * t).sin() + wave;
            let local = Vec3::new(t * leaf.length, y, s * w);
            positions.push(leaf.base + rotation * local);
            let rib = (1.0 - s.abs() * 6.0).max(0.0);
            let edge = leaf.colour * (1.0 - 0.12 * s.abs());
            colours.push(linear(edge.lerp(leaf.rib, rib)));
        }
    }
    let mut indices = Vec::new();
    let at = |k: usize, j: usize| (k * across + j) as u32;
    for k in 0..ALONG {
        for j in 0..across - 1 {
            let (a, b, c, d) = (at(k, j), at(k, j + 1), at(k + 1, j + 1), at(k + 1, j));
            indices.extend([a, b, c, a, c, d]);
        }
    }
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for t in indices.chunks(3) {
        let [a, b, c] = [0, 1, 2].map(|k| positions[t[k] as usize]);
        let face = (b - a).cross(c - a);
        // Every face's normal up, towards the camera.
        let face = if face.y < 0.0 { -face } else { face };
        t.iter().for_each(|&k| normals[k as usize] += face);
    }
    let normals: Vec<Vec3> = normals
        .into_iter()
        .map(|n| n.normalize_or(Vec3::Y))
        .collect();
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices));
    let white = kit.sheet(Color::WHITE);
    kit.mesh(mesh, &white, Transform::IDENTITY);
}

impl Leaf {
    /// A plain leaf of this outline and colour, flat, from `base` towards `yaw`.
    pub(super) fn new(blade: Blade, base: Vec3, yaw: f32, length: f32, width: f32) -> Self {
        Self {
            blade,
            base,
            yaw,
            length,
            width,
            cup: 0.25,
            ruffle: 0.0,
            arch: 0.06,
            colour: Vec3::new(0.25, 0.45, 0.18),
            rib: Vec3::new(0.6, 0.72, 0.45),
        }
    }

    pub(super) fn coloured(mut self, colour: Vec3, rib: Vec3) -> Self {
        self.colour = colour;
        self.rib = rib;
        self
    }
}
