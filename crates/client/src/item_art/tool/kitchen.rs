//! Kitchen tools (cooking plan PR 8): the Pan, a shallow iron frying pan with
//! a long handle; the Bowl, a wide glazed earthenware mixing bowl; the Sieve,
//! a wooden hoop with a mesh across it.

use bevy::prelude::*;

use super::{Part, iron, matte};

fn part(mesh: Handle<Mesh>, material: &Handle<StandardMaterial>, transform: Transform) -> Part {
    Part {
        mesh,
        material: material.clone(),
        transform,
    }
}

/// The Pan: 1 × 2 cells, its handle running up the picture.
pub(super) fn pan_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let metal = iron(materials);
    let wood = materials.add(matte(0.42, 0.27, 0.14, 0.8));
    let dark = materials.add(matte(0.12, 0.12, 0.13, 0.6));
    vec![
        // The pan, its rim, and its dark cooking face.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.5,
                radius_bottom: 0.4,
                height: 0.16,
            }),
            &metal,
            Transform::from_xyz(0.0, 0.08, -0.35),
        ),
        part(
            meshes.add(Torus::new(0.45, 0.52)),
            &metal,
            Transform::from_xyz(0.0, 0.16, -0.35),
        ),
        part(
            meshes.add(Cylinder::new(0.44, 0.01)),
            &dark,
            Transform::from_xyz(0.0, 0.165, -0.35),
        ),
        // The handle: an iron shank and a wooden grip.
        part(
            meshes.add(Cuboid::new(0.09, 0.05, 0.4)),
            &metal,
            Transform::from_xyz(0.0, 0.15, 0.45),
        ),
        part(
            meshes.add(Capsule3d::new(0.07, 0.5)),
            &wood,
            Transform::from_xyz(0.0, 0.17, 0.92).with_rotation(Quat::from_rotation_x(1.5)),
        ),
    ]
}

/// The Bowl: one cell, wide and low.
pub(super) fn bowl_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let clay = materials.add(matte(0.72, 0.5, 0.33, 0.7));
    let glaze = materials.add(matte(0.86, 0.82, 0.7, 0.35));
    vec![
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.62,
                radius_bottom: 0.32,
                height: 0.5,
            }),
            &clay,
            Transform::from_xyz(0.0, 0.25, 0.0),
        ),
        part(
            meshes.add(Torus::new(0.56, 0.64)),
            &clay,
            Transform::from_xyz(0.0, 0.5, 0.0),
        ),
        // Its glazed inside.
        part(
            meshes.add(Cylinder::new(0.56, 0.02)),
            &glaze,
            Transform::from_xyz(0.0, 0.46, 0.0),
        ),
        part(
            meshes.add(Cylinder::new(0.34, 0.06)),
            &clay,
            Transform::from_xyz(0.0, 0.03, 0.0),
        ),
    ]
}

/// The Sieve: one cell, a wooden hoop and its mesh.
pub(super) fn sieve_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let wood = materials.add(matte(0.68, 0.52, 0.32, 0.8));
    let mesh_colour = materials.add(matte(0.25, 0.22, 0.2, 0.9));
    let mut parts = vec![
        part(
            meshes.add(Torus::new(0.56, 0.66)),
            &wood,
            Transform::from_xyz(0.0, 0.3, 0.0).with_scale(Vec3::new(1.0, 3.0, 1.0)),
        ),
        part(
            meshes.add(Cylinder::new(0.58, 0.015)),
            &mesh_colour,
            Transform::from_xyz(0.0, 0.08, 0.0),
        ),
    ];
    // Threads across the mesh, so it reads as one.
    let thread = meshes.add(Cuboid::new(1.12, 0.03, 0.025));
    for i in -3..=3 {
        let along = i as f32 * 0.15;
        let width = (1.0 - (along / 0.58).powi(2)).max(0.1).sqrt();
        parts.push(part(
            thread.clone(),
            &wood,
            Transform::from_xyz(0.0, 0.1, along).with_scale(Vec3::new(width, 1.0, 1.0)),
        ));
    }
    parts
}
