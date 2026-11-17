//! The Keyring (owner, 2026-11-17): an iron ring with three keys hanging from it,
//! fanned out to the right, lying flat along the picture's width (x), two cells
//! wide and one long. It looks the same however many keys it holds.

use bevy::prelude::*;

use super::Part;

/// The Keyring's parts (`Look::Keyring`), against the picture's `VIEW_WIDTH` of 2.5.
pub(super) fn keyring_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let metal = |materials: &mut Assets<StandardMaterial>, red, green, blue| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(red, green, blue),
            metallic: 0.8,
            perceptual_roughness: 0.45,
            ..default()
        })
    };
    let iron = metal(materials, 0.42, 0.42, 0.44);
    let brass = metal(materials, 0.78, 0.6, 0.28);
    let steel = metal(materials, 0.62, 0.64, 0.68);
    // The ring itself, lying flat (a torus lies in the x-z plane).
    let ring_at = Vec3::new(-0.7, 0.0, 0.0);
    let mut parts = vec![Part {
        mesh: meshes.add(Torus::new(0.3, 0.38)),
        material: iron,
        transform: Transform::from_translation(ring_at),
    }];
    // Three keys through it, fanned out: each a bow round the ring's edge, a
    // shaft, and a bit at the end.
    for (turn, material) in [(-0.35, brass.clone()), (0.0, steel), (0.4, brass)] {
        let along = Transform::from_translation(ring_at + Vec3::new(0.0, 0.05, 0.0))
            .with_rotation(Quat::from_rotation_y(turn));
        let key = [
            // The bow, a flat loop the ring passes through.
            (
                meshes.add(Torus::new(0.07, 0.15)),
                Transform::from_xyz(0.45, 0.0, 0.0),
            ),
            // The shaft.
            (
                meshes.add(Cuboid::new(0.95, 0.06, 0.07)),
                Transform::from_xyz(1.05, 0.0, 0.0),
            ),
            // The bit, its teeth to one side.
            (
                meshes.add(Cuboid::new(0.18, 0.06, 0.2)),
                Transform::from_xyz(1.42, 0.0, 0.12),
            ),
        ];
        for (mesh, transform) in key {
            parts.push(Part {
                mesh,
                material: material.clone(),
                transform: along * transform,
            });
        }
    }
    parts
}
