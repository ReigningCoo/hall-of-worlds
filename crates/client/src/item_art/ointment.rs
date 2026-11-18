//! Ointment (brewing PR 6): a small round tin, its lid off and leaning behind,
//! full of salve coloured as a potion of the same doses would be
//! (`LiquidLook::of`), a swirl of its second colour on top. Built upright and
//! tipped towards the camera, like the tools. One cell.

use bevy::prelude::*;

use super::bottle::{main_colour, second_colour};
use super::{LiquidLook, Part};

/// How far it is tipped back about x, so the salve's top shows.
const TIP: f32 = -0.35;
/// The tin's radius and height.
const RADIUS: f32 = 0.62;
const HEIGHT: f32 = 0.42;

/// An ointment's parts (`Look::Ointment`), against the picture's `VIEW_WIDTH`.
pub(super) fn ointment_parts(
    salve: Option<&LiquidLook>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let tin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.66, 0.66, 0.64),
        metallic: 0.8,
        perceptual_roughness: 0.4,
        ..default()
    });
    let (main, second) = salve.map_or((Color::srgb(0.8, 0.76, 0.6), None), |look| {
        (main_colour(look), Some(second_colour(look)))
    });
    let matte = |colour: Color| StandardMaterial {
        base_color: colour,
        perceptual_roughness: 0.35,
        reflectance: 0.3,
        ..default()
    };
    let salve_top = materials.add(matte(main));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The tin, its rolled rim, and the salve filling it.
        part(
            meshes.add(Cylinder::new(RADIUS, HEIGHT)),
            &tin,
            Transform::from_xyz(0.0, -0.1, 0.0),
        ),
        part(
            meshes.add(Torus::new(RADIUS - 0.04, RADIUS + 0.04)),
            &tin,
            Transform::from_xyz(0.0, HEIGHT / 2.0 - 0.1, 0.0),
        ),
        part(
            meshes.add(Cylinder::new(RADIUS - 0.04, 0.02)),
            &salve_top,
            Transform::from_xyz(0.0, HEIGHT / 2.0 - 0.08, 0.0),
        ),
        // The lid, upright behind it.
        part(
            meshes.add(Cylinder::new(RADIUS + 0.02, 0.08)),
            &tin,
            Transform::from_xyz(0.0, 0.0, -0.8).with_rotation(Quat::from_rotation_x(1.2)),
        ),
    ];
    if let Some(second) = second {
        // A swirl of the second colour where it was scooped.
        let swirl = materials.add(matte(second));
        parts.push(part(
            meshes.add(Torus::new(0.16, 0.24)),
            &swirl,
            Transform::from_xyz(0.12, HEIGHT / 2.0 - 0.07, 0.08)
                .with_scale(Vec3::new(1.0, 0.4, 1.0)),
        ));
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
