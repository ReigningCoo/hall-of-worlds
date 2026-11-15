//! The Bag (owner, 2026-11-15): a cloth bag, plump at the bottom, gathered at
//! the neck by a drawstring whose two ends hang down its front, the cloth's edge
//! frilled above the tie. Three cells by three, so square.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Part;

/// How thick the drawstring is.
const CORD: f32 = 0.045;

/// The Bag's parts (`Look::Bag`), against the picture's `VIEW_WIDTH` of 2.5.
pub(super) fn bag_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let cloth = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.52, 0.38),
        perceptual_roughness: 0.95,
        reflectance: 0.08,
        ..default()
    });
    let cord = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.24, 0.14),
        perceptual_roughness: 0.85,
        ..default()
    });
    let mut parts = vec![
        // The body, sagging a little and flatter front to back.
        Part {
            mesh: meshes.add(Sphere::new(0.78)),
            material: cloth.clone(),
            transform: Transform::from_xyz(0.0, -0.3, 0.0).with_scale(Vec3::new(1.05, 0.9, 0.85)),
        },
        // Narrowing up to the neck.
        Part {
            mesh: meshes.add(ConicalFrustum {
                radius_top: 0.2,
                radius_bottom: 0.55,
                height: 0.45,
            }),
            material: cloth.clone(),
            transform: Transform::from_xyz(0.0, 0.42, 0.0),
        },
        // The frill above the tie, opening out.
        Part {
            mesh: meshes.add(ConicalFrustum {
                radius_top: 0.36,
                radius_bottom: 0.2,
                height: 0.28,
            }),
            material: cloth,
            transform: Transform::from_xyz(0.0, 0.78, 0.0),
        },
        // The drawstring round the neck.
        Part {
            mesh: meshes.add(Torus::new(0.2, 0.2 + CORD * 2.0)),
            material: cord.clone(),
            transform: Transform::from_xyz(0.0, 0.66, 0.0),
        },
    ];
    // Its two ends hanging down the front, a little apart, each knotted.
    for side in [-1.0, 1.0] {
        let tilt = Quat::from_rotation_z(side * 0.25) * Quat::from_rotation_x(-0.35);
        let end = Vec3::new(side * 0.12, 0.66, 0.24);
        let length = 0.55;
        let middle = end + tilt * Vec3::new(0.0, -length / 2.0, 0.0);
        parts.push(Part {
            mesh: meshes.add(Cylinder::new(CORD, length)),
            material: cord.clone(),
            transform: Transform::from_translation(middle).with_rotation(tilt),
        });
        let knot = end + tilt * Vec3::new(0.0, -length, 0.0);
        parts.push(Part {
            mesh: meshes.add(Sphere::new(CORD * 2.2)),
            material: cord.clone(),
            transform: Transform::from_translation(knot)
                .with_rotation(Quat::from_rotation_y(PI / 4.0)),
        });
    }
    parts
}
