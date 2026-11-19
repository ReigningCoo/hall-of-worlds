//! The Filter (Alchemy plan PR 6): a glass funnel lined with a folded paper
//! cone, its spout below. One cell.

use bevy::prelude::*;

use super::{Part, matte};

/// Where the funnel's mouth is.
const RIM: f32 = 0.5;

pub(super) fn filter_parts(
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
    let paper = materials.add(matte(0.95, 0.93, 0.86, 0.95));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    // Cones point up: turned over, they open upwards like a funnel.
    let over = Quat::from_rotation_x(std::f32::consts::PI);
    vec![
        // The funnel's bowl and its rim.
        part(
            meshes.add(Cone {
                radius: 0.62,
                height: 0.75,
            }),
            &glass,
            Transform::from_xyz(0.0, RIM - 0.375, 0.0).with_rotation(over),
        ),
        part(
            meshes.add(Torus::new(0.58, 0.64)),
            &glass,
            Transform::from_xyz(0.0, RIM, 0.0),
        ),
        // The paper inside it, standing a little above the rim.
        part(
            meshes.add(Cone {
                radius: 0.5,
                height: 0.7,
            }),
            &paper,
            Transform::from_xyz(0.0, RIM - 0.27, 0.0).with_rotation(over),
        ),
        // The spout.
        part(
            meshes.add(Cylinder::new(0.08, 0.6)),
            &glass,
            Transform::from_xyz(0.0, RIM - 0.95, 0.0),
        ),
    ]
}
