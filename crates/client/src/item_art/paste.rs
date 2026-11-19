//! Paste (Alchemy plan PR 3): a heap of ground herbs in a shallow clay dish,
//! coloured as a potion of the same doses would be (`LiquidLook::of`), flecked
//! with its second colour. Built upright and tipped towards the camera, like
//! an ointment. One cell.

use bevy::prelude::*;

use super::bottle::{main_colour, second_colour};
use super::{LiquidLook, Part};

/// How far it is tipped back about x, so the heap shows.
const TIP: f32 = -0.35;

/// A paste's parts (`Look::Paste`), against the picture's `VIEW_WIDTH`.
pub(super) fn paste_parts(
    paste: Option<&LiquidLook>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let matte = |colour: Color, roughness: f32| StandardMaterial {
        base_color: colour,
        perceptual_roughness: roughness,
        reflectance: 0.2,
        ..default()
    };
    let clay = materials.add(matte(Color::srgb(0.66, 0.42, 0.28), 0.9));
    let (main, second) = paste.map_or((Color::srgb(0.45, 0.52, 0.3), None), |look| {
        (main_colour(look), Some(second_colour(look)))
    });
    let heap = materials.add(matte(main, 0.8));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The dish, wider at the top, and its rim.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.7,
                radius_bottom: 0.45,
                height: 0.22,
            }),
            &clay,
            Transform::from_xyz(0.0, -0.2, 0.0),
        ),
        part(
            meshes.add(Torus::new(0.64, 0.74)),
            &clay,
            Transform::from_xyz(0.0, -0.09, 0.0),
        ),
        // The heap of paste in it.
        part(
            meshes.add(Sphere::new(0.52)),
            &heap,
            Transform::from_xyz(0.0, -0.08, 0.0).with_scale(Vec3::new(1.0, 0.45, 1.0)),
        ),
    ];
    if let Some(second) = second {
        // Flecks of the second colour on top.
        let fleck = materials.add(matte(second, 0.8));
        for (x, z) in [(0.18, 0.1), (-0.2, 0.05), (0.02, -0.18)] {
            parts.push(part(
                meshes.add(Sphere::new(0.07)),
                &fleck,
                Transform::from_xyz(x, 0.12, z),
            ));
        }
    }
    let tipped = Transform::from_rotation(Quat::from_rotation_x(TIP));
    parts
        .into_iter()
        .map(|part| Part {
            transform: tipped * part.transform,
            ..part
        })
        .collect()
}
