//! Salt (Alchemy plan PR 5): a small white mound of crystals, a few loose
//! grains beside it. The same for every Salt. One cell.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

/// How far the heap is tipped towards the camera, so its shape shows.
const TIP: f32 = 0.6;

/// The heap's parts (`Look::Solid` of Salt), against the picture's `VIEW_WIDTH` of 2.5.
pub(super) fn salt_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.93),
        perceptual_roughness: 0.6,
        reflectance: 0.4,
        ..default()
    });
    let part = |mesh, transform| Part {
        mesh,
        material: white.clone(),
        transform,
    };
    // A rounded mound, a smaller one heaped on it.
    let mut parts = vec![
        part(
            meshes.add(Sphere::new(1.0)),
            Transform::from_xyz(0.0, -0.35, 0.0).with_scale(Vec3::new(0.7, 0.45, 0.7)),
        ),
        part(
            meshes.add(Sphere::new(1.0)),
            Transform::from_xyz(0.05, -0.05, 0.0).with_scale(Vec3::new(0.38, 0.3, 0.38)),
        ),
    ];
    // Loose grains, little cubes turned this way and that.
    for (x, z, turn) in [(-0.8, 0.25, 0.3), (0.78, -0.1, 0.9), (0.45, 0.55, 1.4)] {
        parts.push(part(
            meshes.add(Cuboid::new(0.12, 0.12, 0.12)),
            Transform::from_xyz(x, -0.32, z).with_rotation(Quat::from_rotation_y(turn)),
        ));
    }
    // Built standing up, tipped, then turned to face the pictures' camera,
    // which looks down from above with -z up the picture.
    let facing = Transform::from_rotation(Quat::from_rotation_x(TIP - FRAC_PI_2));
    parts
        .into_iter()
        .map(|part| Part {
            transform: facing * part.transform,
            ..part
        })
        .collect()
}
