//! A serving cut from a whole one, showing its inside: a wedge of a round one
//! (swede, pumpkin with its stringy middle and seeds, agave heart) or a round
//! slice of a long one (cassava with its woody strand, yam).

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::TAU;

use super::Build;
use super::colours::colour;
use super::tube::linear;

/// Steps round the arc, and out from the middle.
const ARC: usize = 24;
const OUT: usize = 8;

pub(super) fn slice(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let (skin, inside) = (c.skin, c.inside);
    match build.def.key.as_str() {
        "cassava" | "yam" => {
            let rind = if build.def.key == "cassava" {
                Vec3::new(0.85, 0.62, 0.6)
            } else {
                skin * 1.2
            };
            let flesh = move |r: f32| match r {
                r if r > 0.93 => skin,
                r if r > 0.86 => rind,
                _ => inside,
            };
            sector(kit, TAU, 0.18, flesh, skin);
            if build.def.key == "cassava" {
                let strand = kit.material(colour(inside * 0.85), false);
                kit.blob(
                    &strand,
                    Vec3::new(0.0, 0.18, 0.0),
                    Vec3::new(0.06, 0.01, 0.06),
                    (0.0, 0.0),
                );
            }
        }
        key => {
            let pumpkin = key == "pumpkin";
            let pulp = inside.lerp(Vec3::new(0.95, 0.85, 0.55), 0.6);
            let flesh = move |r: f32| match r {
                r if r > 0.9 => skin,
                r if r < 0.3 && pumpkin => pulp,
                _ => inside,
            };
            sector(kit, 1.0, 0.3, flesh, skin);
            if pumpkin {
                let seed = kit.material(colour(Vec3::new(0.95, 0.92, 0.82)), false);
                for k in 0..3 {
                    let a = -0.25 + k as f32 * 0.25;
                    let at = Vec3::new(a.cos(), 0.0, -a.sin()) * (0.12 + 0.05 * (k % 2) as f32);
                    let half = Vec3::new(0.05, 0.012, 0.03);
                    kit.blob(&seed, at + Vec3::Y * 0.3, half, (a, 0.0));
                }
            }
        }
    }
}

/// A slice lying flat `height` thick, a sector of a disc of radius 1 spanning
/// `angle` (centred on +x; a whole round for `TAU`): its top coloured by how
/// far out (0 to 1), its rounded side in `skin`, its cut sides as its top.
fn sector(kit: &mut super::Kit, angle: f32, height: f32, flesh: impl Fn(f32) -> Vec3, skin: Vec3) {
    let whole = angle >= TAU - 0.001;
    let point = |r: f32, k: usize| {
        let a = (k as f32 / ARC as f32 - 0.5) * angle;
        Vec3::new(a.cos() * r, height, -a.sin() * r)
    };
    let (mut positions, mut colours, mut indices) = (Vec::new(), Vec::new(), Vec::new());
    let mut quad = |corners: [Vec3; 4], tint: [Vec3; 4]| {
        let first = positions.len() as u32;
        positions.extend(corners);
        colours.extend(tint.map(linear));
        indices.extend([0, 1, 2, 0, 2, 3].map(|i| first + i));
    };
    for o in 0..OUT {
        let (r0, r1) = (o as f32 / OUT as f32, (o + 1) as f32 / OUT as f32);
        for k in 0..ARC {
            let corners = [
                point(r0, k),
                point(r1, k),
                point(r1, k + 1),
                point(r0, k + 1),
            ];
            let tint = [r0, r1, r1, r0].map(&flesh);
            quad(corners, tint);
        }
    }
    let down = Vec3::Y * height;
    for k in 0..ARC {
        let (a, b) = (point(1.0, k), point(1.0, k + 1));
        quad([a, a - down, b - down, b], [skin * 0.9; 4]);
    }
    if !whole {
        for k in [0, ARC] {
            let (a, b) = (point(0.0, k), point(1.0, k));
            quad([a, b, b - down, a - down], [flesh(0.5); 4]);
        }
    }
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for t in indices.chunks_mut(3) {
        let [a, b, c] = [0, 1, 2].map(|k| positions[t[k] as usize]);
        let middle = (a + b + c) / 3.0;
        let mut face = (b - a).cross(c - a);
        // Facing out of the slice: up on top, away from its middle round the side.
        let outward = if (middle.y - height).abs() < 1e-4 {
            Vec3::Y
        } else {
            Vec3::new(middle.x - 0.4 * f32::from(!whole), 0.0, middle.z)
        };
        if face.dot(outward) < 0.0 {
            t.swap(1, 2);
            face = -face;
        }
        t.iter()
            .for_each(|&k| normals[k as usize] = face.normalize_or(Vec3::Y));
    }
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
