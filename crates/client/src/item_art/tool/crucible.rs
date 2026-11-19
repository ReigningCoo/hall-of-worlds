//! The Crucible (Alchemy plan PR 6): a tall clay pot, narrow at the foot and
//! wide at the mouth, with a pinched pouring lip, its inside glowing from the
//! heat. One cell.

use bevy::prelude::*;

use super::{Part, matte};

/// Where the mouth is.
const RIM: f32 = 0.45;

pub(super) fn crucible_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let clay = materials.add(matte(0.78, 0.74, 0.66, 0.9));
    let scorched = materials.add(matte(0.3, 0.27, 0.25, 0.95));
    let glow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.55, 0.15),
        emissive: LinearRgba::rgb(2.0, 0.7, 0.1),
        ..default()
    });
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    vec![
        // The pot, wider at the mouth.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.5,
                radius_bottom: 0.28,
                height: 1.1,
            }),
            &clay,
            Transform::from_xyz(0.0, RIM - 0.55, 0.0),
        ),
        // Its foot, blackened by the fire.
        part(
            meshes.add(Cylinder::new(0.3, 0.12)),
            &scorched,
            Transform::from_xyz(0.0, RIM - 1.1, 0.0),
        ),
        // The rim, and the glowing inside.
        part(
            meshes.add(Torus::new(0.42, 0.52)),
            &clay,
            Transform::from_xyz(0.0, RIM, 0.0),
        ),
        part(
            meshes.add(Cylinder::new(0.44, 0.02)),
            &glow,
            Transform::from_xyz(0.0, RIM + 0.005, 0.0),
        ),
        // The pouring lip, pinched out to the right.
        part(
            meshes.add(Sphere::new(0.12)),
            &clay,
            Transform::from_xyz(0.5, RIM, 0.0).with_scale(Vec3::new(1.2, 0.5, 0.8)),
        ),
    ]
}
