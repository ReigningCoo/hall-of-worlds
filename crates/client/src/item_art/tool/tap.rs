//! The Tap: a brass spigot knocked into a cask to draw from it. A tapering
//! barrel lying across, a collar where it goes into the cask, a spout bent
//! down at its end and a T-shaped key on top to turn it. One cell.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

pub(super) fn tap_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let brass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.6, 0.28),
        metallic: 0.85,
        perceptual_roughness: 0.35,
        ..default()
    });
    let part = |mesh, transform| Part {
        mesh,
        material: brass.clone(),
        transform,
    };
    let across = Quat::from_rotation_z(FRAC_PI_2);
    vec![
        // The barrel, narrowing towards the cask on the left.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.12,
                radius_bottom: 0.2,
                height: 1.5,
            }),
            Transform::from_xyz(-0.1, 0.0, 0.0).with_rotation(across),
        ),
        // The collar against the cask.
        part(
            meshes.add(Cylinder::new(0.3, 0.1)),
            Transform::from_xyz(-0.55, 0.0, 0.0).with_rotation(across),
        ),
        // The spout, bent down at the right end.
        part(
            meshes.add(Sphere::new(0.2)),
            Transform::from_xyz(0.65, 0.0, 0.0),
        ),
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.15,
                radius_bottom: 0.11,
                height: 0.5,
            }),
            Transform::from_xyz(0.65, -0.3, 0.0),
        ),
        // The key: a stem up and a bar across its top.
        part(
            meshes.add(Cylinder::new(0.07, 0.45)),
            Transform::from_xyz(0.15, 0.3, 0.0),
        ),
        part(
            meshes.add(Cuboid::new(0.6, 0.12, 0.12)),
            Transform::from_xyz(0.15, 0.55, 0.0),
        ),
    ]
}
