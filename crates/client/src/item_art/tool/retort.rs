//! The Retort (Alchemy plan PR 3): a round glass bulb on a small ring stand,
//! its long neck rising and bending down to the right, as alchemists drew it.
//! One cell wide, two long.

use bevy::prelude::*;

use super::{Part, iron};

/// The bulb's radius, and where its middle is.
const BULB: f32 = 0.42;
const MIDDLE: Vec3 = Vec3::new(-0.2, -0.35, 0.0);

pub(super) fn retort_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.72, 0.85, 0.8, 0.55),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.08,
        reflectance: 0.6,
        ..default()
    });
    let stand = iron(materials);
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    // The neck: up out of the bulb, then a long arm leaning down to the right.
    let top = MIDDLE + Vec3::new(0.0, BULB + 0.12, 0.0);
    let lean = 2.1_f32;
    let along = Vec3::new(lean.sin(), lean.cos(), 0.0);
    let arm = 1.1;
    vec![
        part(
            meshes.add(Sphere::new(BULB)),
            &glass,
            Transform::from_translation(MIDDLE),
        ),
        part(
            meshes.add(Cylinder::new(0.1, 0.3)),
            &glass,
            Transform::from_translation(MIDDLE + Vec3::new(0.0, BULB, 0.0)),
        ),
        part(
            meshes.add(Capsule3d::new(0.08, 0.2)),
            &glass,
            Transform::from_translation(top),
        ),
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.09,
                radius_bottom: 0.05,
                height: arm,
            }),
            &glass,
            Transform::from_translation(top + along * arm / 2.0)
                .with_rotation(Quat::from_rotation_z(-lean)),
        ),
        // The ring stand under the bulb.
        part(
            meshes.add(Torus::new(0.26, 0.32)),
            &stand,
            Transform::from_translation(MIDDLE - Vec3::new(0.0, BULB * 0.75, 0.0)),
        ),
        part(
            meshes.add(Cylinder::new(0.36, 0.05)),
            &stand,
            Transform::from_translation(MIDDLE - Vec3::new(0.0, BULB + 0.2, 0.0)),
        ),
    ]
}
