//! The Herb Pouch (owner, 2026-11-17): a soft pouch of green-dyed linen, wide
//! and shallow, gathered at the top by a drawstring, with a leaf stitched on the
//! front and a sprig poking out of the mouth. Two cells wide by one long. It
//! looks the same however many herbs it holds.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

/// The Herb Pouch's parts (`Look::HerbPouch`), against the picture's
/// `VIEW_WIDTH` of 2.5.
pub(super) fn herb_pouch_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let matte = |base_color| StandardMaterial {
        base_color,
        perceptual_roughness: 0.9,
        reflectance: 0.08,
        ..default()
    };
    let linen = materials.add(matte(Color::srgb(0.46, 0.55, 0.32)));
    let cord = materials.add(matte(Color::srgb(0.72, 0.6, 0.4)));
    let leaf = materials.add(matte(Color::srgb(0.2, 0.52, 0.18)));
    let vein = materials.add(matte(Color::srgb(0.55, 0.75, 0.35)));
    let parts = vec![
        // The body, wide and squat, flattened front to back.
        Part {
            mesh: meshes.add(Sphere::new(0.5)),
            material: linen.clone(),
            transform: Transform::from_xyz(0.0, -0.08, 0.0).with_scale(Vec3::new(2.1, 0.8, 0.5)),
        },
        // The gathered neck above it.
        Part {
            mesh: meshes.add(Cylinder::new(0.3, 0.16)),
            material: linen,
            transform: Transform::from_xyz(0.0, 0.36, 0.0).with_scale(Vec3::new(1.6, 1.0, 0.6)),
        },
        // The drawstring round the neck, its ends hanging to one side.
        Part {
            mesh: meshes.add(Torus::new(0.27, 0.32)),
            material: cord.clone(),
            transform: Transform::from_xyz(0.0, 0.33, 0.0).with_scale(Vec3::new(1.55, 1.0, 0.65)),
        },
        Part {
            mesh: meshes.add(Capsule3d::new(0.025, 0.3)),
            material: cord,
            transform: Transform::from_xyz(0.55, 0.15, 0.2)
                .with_rotation(Quat::from_rotation_z(0.35)),
        },
        // A sprig poking out of the mouth.
        Part {
            mesh: meshes.add(Sphere::new(0.14)),
            material: leaf.clone(),
            transform: Transform::from_xyz(-0.18, 0.52, 0.0)
                .with_rotation(Quat::from_rotation_z(0.5))
                .with_scale(Vec3::new(0.5, 1.4, 0.3)),
        },
        // The leaf stitched on the front, pointing up and to the left, with
        // its middle vein.
        Part {
            mesh: meshes.add(Sphere::new(0.22)),
            material: leaf,
            transform: Transform::from_xyz(0.0, -0.08, 0.24)
                .with_rotation(Quat::from_rotation_z(0.6))
                .with_scale(Vec3::new(0.55, 1.2, 0.15)),
        },
        Part {
            mesh: meshes.add(Cuboid::new(0.03, 0.42, 0.02)),
            material: vein,
            transform: Transform::from_xyz(0.0, -0.08, 0.27)
                .with_rotation(Quat::from_rotation_z(0.6)),
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
