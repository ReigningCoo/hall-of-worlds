//! The Pelican (Alchemy plan PR 7): a round glass flask with a tall stoppered
//! neck, two curved arms looping from the neck back into its belly (its
//! vapours running down them again and again), a little liquid inside. 2 × 2
//! cells.

use bevy::prelude::*;

use super::{Part, matte};

/// Where the belly's middle is.
const BELLY: f32 = -0.3;

pub(super) fn pelican_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.72, 0.85, 0.8, 0.5),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.08,
        reflectance: 0.6,
        ..default()
    });
    let liquid = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.55, 0.2, 0.85),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.2,
        ..default()
    });
    let cork = materials.add(matte(0.55, 0.4, 0.25, 0.95));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    // Tori lie flat; stood up about x, they loop in the picture's plane.
    let upright = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let arm = |x: f32| {
        Transform::from_xyz(x, BELLY + 0.45, 0.0)
            .with_rotation(upright)
            .with_scale(Vec3::new(1.0, 1.0, 1.4))
    };
    vec![
        // The belly, and the liquid low in it.
        part(
            meshes.add(Sphere::new(0.48)),
            &glass,
            Transform::from_xyz(0.0, BELLY, 0.0),
        ),
        part(
            meshes.add(Sphere::new(0.36)),
            &liquid,
            Transform::from_xyz(0.0, BELLY - 0.1, 0.0).with_scale(Vec3::new(1.0, 0.6, 1.0)),
        ),
        // The tall neck and its stopper.
        part(
            meshes.add(Cylinder::new(0.1, 0.9)),
            &glass,
            Transform::from_xyz(0.0, BELLY + 0.8, 0.0),
        ),
        part(
            meshes.add(Cylinder::new(0.12, 0.12)),
            &cork,
            Transform::from_xyz(0.0, BELLY + 1.28, 0.0),
        ),
        // The two arms, from the neck round into the belly.
        part(meshes.add(Torus::new(0.26, 0.34)), &glass, arm(-0.32)),
        part(meshes.add(Torus::new(0.26, 0.34)), &glass, arm(0.32)),
    ]
}
