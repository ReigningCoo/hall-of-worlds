//! The Alembic (Alchemy plan PR 5): a copper pot (the cucurbit) on a short
//! iron stand, a domed head on it, and the long beak running down to the right
//! into a small glass receiver, as alchemists drew it. Two cells by two.

use bevy::prelude::*;

use super::{Part, iron};

/// Where the pot's middle is.
const POT: Vec3 = Vec3::new(-0.35, -0.25, 0.0);
/// Where the receiver's middle is.
const RECEIVER: Vec3 = Vec3::new(0.65, -0.55, 0.0);

pub(super) fn alembic_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let copper = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.45, 0.25),
        metallic: 0.8,
        perceptual_roughness: 0.4,
        ..default()
    });
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
    // The beak: from the side of the head, leaning down to the receiver's neck.
    let head = POT + Vec3::new(0.0, 0.62, 0.0);
    let start = head + Vec3::new(0.2, 0.0, 0.0);
    let end = RECEIVER + Vec3::new(0.0, 0.3, 0.0);
    let beak = end - start;
    let beak_turn = Quat::from_rotation_arc(Vec3::Y, beak.normalize());
    vec![
        // The pot, round below and narrowing to a neck.
        part(
            meshes.add(Sphere::new(0.42)),
            &copper,
            Transform::from_translation(POT).with_scale(Vec3::new(1.0, 0.9, 1.0)),
        ),
        part(
            meshes.add(Cylinder::new(0.16, 0.2)),
            &copper,
            Transform::from_translation(POT + Vec3::new(0.0, 0.42, 0.0)),
        ),
        // The domed head.
        part(
            meshes.add(Sphere::new(0.26)),
            &copper,
            Transform::from_translation(head).with_scale(Vec3::new(1.0, 0.7, 1.0)),
        ),
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.04,
                radius_bottom: 0.07,
                height: beak.length(),
            }),
            &copper,
            Transform::from_translation(start + beak / 2.0).with_rotation(beak_turn),
        ),
        // The receiver, a small glass flask.
        part(
            meshes.add(Sphere::new(0.22)),
            &glass,
            Transform::from_translation(RECEIVER),
        ),
        part(
            meshes.add(Cylinder::new(0.07, 0.2)),
            &glass,
            Transform::from_translation(RECEIVER + Vec3::new(0.0, 0.22, 0.0)),
        ),
        // The stand under the pot.
        part(
            meshes.add(Torus::new(0.3, 0.36)),
            &stand,
            Transform::from_translation(POT - Vec3::new(0.0, 0.3, 0.0)),
        ),
        part(
            meshes.add(Cylinder::new(0.4, 0.06)),
            &stand,
            Transform::from_translation(POT - Vec3::new(0.0, 0.52, 0.0)),
        ),
    ]
}
