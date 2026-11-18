//! The Cauldron: a round-bellied iron pot, empty and dark inside, on three
//! short legs, with a lug either side and a rolled rim. Two cells by two.

use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

use super::{Part, iron, matte};

/// How wide the pot is, and where its mouth is.
const RADIUS: f32 = 0.72;
const MOUTH: f32 = 0.4;

pub(super) fn cauldron_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let iron = iron(materials);
    let inside = materials.add(matte(0.05, 0.05, 0.05, 0.9));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The straight upper wall, and the round belly under it.
        part(
            meshes.add(Cylinder::new(RADIUS, 0.5)),
            &iron,
            Transform::from_xyz(0.0, MOUTH - 0.25, 0.0),
        ),
        part(
            meshes.add(Sphere::new(RADIUS)),
            &iron,
            Transform::from_xyz(0.0, MOUTH - 0.56, 0.0).with_scale(Vec3::new(1.0, 0.75, 1.0)),
        ),
        // The dark inside, just under the rim.
        part(
            meshes.add(Cylinder::new(RADIUS - 0.06, 0.02)),
            &inside,
            Transform::from_xyz(0.0, MOUTH + 0.005, 0.0),
        ),
        // The rolled rim.
        part(
            meshes.add(Torus::new(RADIUS - 0.07, RADIUS + 0.06)),
            &iron,
            Transform::from_xyz(0.0, MOUTH, 0.0),
        ),
    ];
    // Three short legs under the belly.
    for leg in 0..3 {
        let angle = FRAC_PI_2 + TAU * leg as f32 / 3.0;
        let at = Vec3::new(angle.cos() * 0.5, MOUTH - 1.1, angle.sin() * 0.5);
        parts.push(part(
            meshes.add(Cylinder::new(0.07, 0.36)),
            &iron,
            Transform::from_translation(at),
        ));
    }
    // A lug either side, standing up like ears.
    for side in [-1.0, 1.0] {
        parts.push(part(
            meshes.add(Torus::new(0.09, 0.16)),
            &iron,
            Transform::from_xyz(side * (RADIUS + 0.1), MOUTH - 0.12, 0.0)
                .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        ));
    }
    parts
}
