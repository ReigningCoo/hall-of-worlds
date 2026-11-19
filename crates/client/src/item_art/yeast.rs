//! Yeast (drinks PR 4): a small stoneware crock of starter, glazed brown at
//! the top over pale clay below, its rim rolled, the cream-coloured starter
//! frothing up inside with a few bubbles on it. Tipped towards the camera, as
//! the sack of flour is, so the froth shows. The same for every yeast: its
//! quality is the popup's. One cell.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;

/// How far the crock is tipped towards the camera, so the starter shows.
const TIP: f32 = 0.6;

/// The crock's parts (`Look::Yeast`), against the picture's `VIEW_WIDTH` of
/// 2.5.
pub(super) fn yeast_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut colour = |red, green, blue, roughness| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(red, green, blue),
            perceptual_roughness: roughness,
            reflectance: 0.2,
            ..default()
        })
    };
    let clay = colour(0.82, 0.74, 0.6, 0.9);
    let glaze = colour(0.42, 0.24, 0.12, 0.35);
    let starter = colour(0.93, 0.87, 0.72, 0.8);
    let bubble = colour(0.98, 0.95, 0.86, 0.5);
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The pale belly, swelling a little, and the glazed shoulder above.
        part(
            meshes.add(Sphere::new(1.0)),
            &clay,
            Transform::from_xyz(0.0, -0.1, 0.0).with_scale(Vec3::new(0.6, 0.5, 0.6)),
        ),
        part(
            meshes.add(Cylinder::new(0.5, 0.5)),
            &glaze,
            Transform::from_xyz(0.0, 0.25, 0.0),
        ),
        // The rolled rim.
        part(
            meshes.add(Torus::new(0.4, 0.52)),
            &glaze,
            Transform::from_xyz(0.0, 0.5, 0.0),
        ),
        // The starter frothing up inside the rim.
        part(
            meshes.add(Sphere::new(1.0)),
            &starter,
            Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::new(0.42, 0.12, 0.42)),
        ),
    ];
    // A few bubbles on the froth.
    for (x, z, size) in [(-0.15, 0.1, 0.07), (0.12, -0.08, 0.05), (0.18, 0.16, 0.04)] {
        parts.push(part(
            meshes.add(Sphere::new(size)),
            &bubble,
            Transform::from_xyz(x, 0.6, z),
        ));
    }
    // Built standing up, tipped, then turned to face the pictures' camera,
    // which looks down from above with -z up the picture.
    let facing = Transform::from_rotation(Quat::from_rotation_x(TIP - FRAC_PI_2));
    parts
        .into_iter()
        .map(|part| Part {
            transform: facing * part.transform,
            ..part
        })
        .collect()
}
