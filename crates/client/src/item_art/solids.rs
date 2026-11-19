//! The Retort's sold solids (Alchemy plan PR 8): Sulphur, a few rough yellow
//! lumps; Mercury, a little stoppered glass phial with a silver bead of
//! quicksilver in it. The same for every one; one cell each.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

/// How far each is tipped towards the camera, so its shape shows.
const TIP: f32 = 0.6;

/// Sulphur's parts (`Look::Solid` of Sulphur), against the picture's
/// `VIEW_WIDTH` of 2.5.
pub(super) fn sulphur_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let yellow = materials.add(StandardMaterial {
        base_color: Color::srgb(0.93, 0.82, 0.16),
        perceptual_roughness: 0.85,
        ..default()
    });
    // Lumps squashed and turned this way and that, so none is round.
    let lumps = [
        (
            Vec3::new(-0.25, -0.3, 0.05),
            Vec3::new(0.5, 0.36, 0.42),
            0.4,
        ),
        (
            Vec3::new(0.38, -0.36, -0.12),
            Vec3::new(0.36, 0.28, 0.32),
            1.1,
        ),
        (Vec3::new(0.05, 0.05, 0.0), Vec3::new(0.3, 0.26, 0.28), 2.0),
        (
            Vec3::new(0.62, -0.42, 0.45),
            Vec3::new(0.16, 0.13, 0.15),
            0.7,
        ),
    ];
    let parts = lumps.map(|(at, scale, turn)| Part {
        mesh: meshes.add(Sphere::new(1.0).mesh().ico(1).unwrap()),
        material: yellow.clone(),
        transform: Transform::from_translation(at)
            .with_rotation(Quat::from_rotation_y(turn) * Quat::from_rotation_z(turn * 0.5))
            .with_scale(scale),
    });
    facing_camera(parts.into())
}

/// Mercury's parts (`Look::Solid` of Mercury).
pub(super) fn mercury_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.72, 0.85, 0.8, 0.55),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.1,
        reflectance: 0.6,
        ..default()
    });
    let silver = materials.add(StandardMaterial {
        base_color: Color::srgb(0.82, 0.84, 0.88),
        metallic: 1.0,
        perceptual_roughness: 0.15,
        ..default()
    });
    let cork = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.42, 0.24),
        perceptual_roughness: 0.9,
        ..default()
    });
    let parts = vec![
        // The quicksilver, a squat bead in the bottom.
        Part {
            mesh: meshes.add(Sphere::new(1.0)),
            material: silver,
            transform: Transform::from_xyz(0.0, -0.55, 0.0).with_scale(Vec3::new(0.3, 0.2, 0.3)),
        },
        // The phial round it, rounded at the bottom.
        Part {
            mesh: meshes.add(Capsule3d::new(0.36, 1.1)),
            material: glass,
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
        },
        Part {
            mesh: meshes.add(Cylinder::new(0.22, 0.3)),
            material: cork,
            transform: Transform::from_xyz(0.0, 0.85, 0.0),
        },
    ];
    facing_camera(parts)
}

/// Built standing up, tipped, then turned to face the pictures' camera, which
/// looks down from above with -z up the picture.
fn facing_camera(parts: Vec<Part>) -> Vec<Part> {
    let facing = Transform::from_rotation(Quat::from_rotation_x(TIP - FRAC_PI_2));
    parts
        .into_iter()
        .map(|part| Part {
            transform: facing * part.transform,
            ..part
        })
        .collect()
}
