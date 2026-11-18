//! The Oil Press: a small wooden screw press. A board for its base, two posts
//! and a beam across their tops, a wooden screw through the beam with a bar to
//! turn it, pressing a plate down into a hooped tub, and a spout at the front
//! where the oil runs out. Two cells by two.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::{Part, iron, matte};

/// Where the base board's top is.
const BASE: f32 = -0.75;

pub(super) fn oil_press_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let wood = materials.add(matte(0.56, 0.39, 0.22, 0.8));
    let dark = materials.add(matte(0.38, 0.25, 0.13, 0.85));
    let iron = iron(materials);
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The base board.
        part(
            meshes.add(Cuboid::new(1.9, 0.2, 1.1)),
            &dark,
            Transform::from_xyz(0.0, BASE - 0.1, 0.0),
        ),
        // The beam across the posts' tops.
        part(
            meshes.add(Cuboid::new(1.9, 0.24, 0.28)),
            &wood,
            Transform::from_xyz(0.0, BASE + 1.4, 0.0),
        ),
        // The screw through the beam, and the bar through its top to turn it.
        part(
            meshes.add(Cylinder::new(0.08, 0.95)),
            &wood,
            Transform::from_xyz(0.0, BASE + 1.3, 0.0),
        ),
        part(
            meshes.add(Capsule3d::new(0.05, 0.9)),
            &dark,
            Transform::from_xyz(0.0, BASE + 1.72, 0.0)
                .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        ),
        // The plate pressing down on what is in the tub.
        part(
            meshes.add(Cylinder::new(0.4, 0.1)),
            &wood,
            Transform::from_xyz(0.0, BASE + 0.82, 0.0),
        ),
        // The tub, and the spout at its foot.
        part(
            meshes.add(Cylinder::new(0.46, 0.55)),
            &dark,
            Transform::from_xyz(0.0, BASE + 0.275, 0.0),
        ),
        part(
            meshes.add(Cuboid::new(0.14, 0.07, 0.35)),
            &wood,
            Transform::from_xyz(0.0, BASE + 0.05, 0.6),
        ),
    ];
    // The two posts.
    for side in [-1.0, 1.0] {
        parts.push(part(
            meshes.add(Cuboid::new(0.2, 1.5, 0.22)),
            &wood,
            Transform::from_xyz(side * 0.8, BASE + 0.75, 0.0),
        ));
    }
    // Iron hoops round the tub.
    for height in [0.1, 0.45] {
        parts.push(part(
            meshes.add(Torus::new(0.44, 0.5)),
            &iron,
            Transform::from_xyz(0.0, BASE + height, 0.0),
        ));
    }
    parts
}
