//! The Pouch: a small leather pouch, wider than deep, its flap folded down
//! over the front and fastened with a wooden toggle, a loop at the back for
//! the belt. Two cells by two, so square.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

/// The Pouch's parts (`Look::Pouch`), against the picture's `VIEW_WIDTH` of 2.5.
pub(super) fn pouch_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let leather = |base_color| StandardMaterial {
        base_color,
        perceptual_roughness: 0.8,
        reflectance: 0.12,
        ..default()
    };
    let body = materials.add(leather(Color::srgb(0.55, 0.36, 0.2)));
    let flap = materials.add(leather(Color::srgb(0.36, 0.21, 0.11)));
    let toggle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.66, 0.46),
        perceptual_roughness: 0.7,
        ..default()
    });
    let parts = vec![
        // The body, flattened front to back and a little squat.
        Part {
            mesh: meshes.add(Sphere::new(0.8)),
            material: body.clone(),
            transform: Transform::from_xyz(0.0, -0.25, 0.0).with_scale(Vec3::new(1.05, 0.85, 0.55)),
        },
        // The flap over the top, standing a little proud of the front.
        Part {
            mesh: meshes.add(Sphere::new(0.8)),
            material: flap.clone(),
            transform: Transform::from_xyz(0.0, 0.3, 0.04).with_scale(Vec3::new(1.1, 0.45, 0.62)),
        },
        // The strap down from the flap's edge to the toggle.
        Part {
            mesh: meshes.add(Cuboid::new(0.14, 0.34, 0.05)),
            material: flap.clone(),
            transform: Transform::from_xyz(0.0, -0.02, 0.45)
                .with_rotation(Quat::from_rotation_x(-0.3)),
        },
        // The wooden toggle across it.
        Part {
            mesh: meshes.add(Capsule3d::new(0.06, 0.24)),
            material: toggle,
            transform: Transform::from_xyz(0.0, -0.18, 0.5)
                .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        },
        // The loop at the back for the belt, standing up above the flap.
        Part {
            mesh: meshes.add(Torus::new(0.2, 0.28)),
            material: body,
            transform: Transform::from_xyz(0.0, 0.78, -0.1)
                .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
                .with_scale(Vec3::new(1.3, 1.0, 1.0)),
        },
    ];
    // Built standing up, then turned to face the pictures' camera, which looks
    // down from above with -z up the picture.
    let facing = Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2));
    parts
        .into_iter()
        .map(|part| Part {
            transform: facing * part.transform,
            ..part
        })
        .collect()
}
