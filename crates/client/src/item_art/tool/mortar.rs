//! The Mortar and Pestle: a grey stone bowl on a low foot, its pestle resting
//! in it and leaning out to the right. One cell.

use bevy::prelude::*;

use super::{Part, matte};

/// Where the bowl's rim is.
const RIM: f32 = 0.2;
/// How far the pestle leans from upright, to the right.
const LEAN: f32 = 0.6;

pub(super) fn mortar_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let stone = materials.add(matte(0.62, 0.6, 0.56, 0.85));
    let hollow = materials.add(matte(0.42, 0.4, 0.37, 0.95));
    let pestle = materials.add(matte(0.74, 0.72, 0.68, 0.7));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    // The pestle's lower end in the bowl, and the way it leans.
    let end = Vec3::new(0.05, RIM - 0.12, 0.0);
    let along = Vec3::new(LEAN.sin(), LEAN.cos(), 0.0);
    let leaning = Quat::from_rotation_z(-LEAN);
    vec![
        // The bowl, wider at the top.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.62,
                radius_bottom: 0.42,
                height: 0.55,
            }),
            &stone,
            Transform::from_xyz(0.0, RIM - 0.275, 0.0),
        ),
        // Its low foot.
        part(
            meshes.add(Cylinder::new(0.46, 0.1)),
            &stone,
            Transform::from_xyz(0.0, RIM - 0.6, 0.0),
        ),
        // The hollow, darker, and the rounded rim round it.
        part(
            meshes.add(Cylinder::new(0.52, 0.02)),
            &hollow,
            Transform::from_xyz(0.0, RIM + 0.005, 0.0),
        ),
        part(
            meshes.add(Torus::new(0.52, 0.64)),
            &stone,
            Transform::from_xyz(0.0, RIM, 0.0),
        ),
        // The pestle: a rod with a rounded, heavier end in the bowl.
        part(
            meshes.add(Capsule3d::new(0.08, 0.8)),
            &pestle,
            Transform::from_translation(end + along * 0.5).with_rotation(leaning),
        ),
        part(
            meshes.add(Sphere::new(0.14)),
            &pestle,
            Transform::from_translation(end + along * 0.1),
        ),
    ]
}
