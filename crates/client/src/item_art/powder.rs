//! Powder (Alchemy plan PR 6): a little cone of calcined powder on a square of
//! folded paper, coloured as a potion of the same doses would be
//! (`LiquidLook::of`) but paler, dusted with its second colour. Built upright
//! and tipped towards the camera, like a paste. One cell.

use bevy::prelude::*;

use super::bottle::{main_colour, second_colour};
use super::{LiquidLook, Part};

/// How far it is tipped back about x, so the heap shows.
const TIP: f32 = -0.45;

/// A powder's parts (`Look::Powder`), against the picture's `VIEW_WIDTH`.
pub(super) fn powder_parts(
    powder: Option<&LiquidLook>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let matte = |colour: Color, roughness: f32| StandardMaterial {
        base_color: colour,
        perceptual_roughness: roughness,
        reflectance: 0.1,
        ..default()
    };
    // Calcined: paler than the potion, a third of the way to white.
    let pale = |colour: Color| colour.mix(&Color::WHITE, 0.35);
    let paper = materials.add(matte(Color::srgb(0.9, 0.86, 0.74), 0.95));
    let (main, second) = powder.map_or((Color::srgb(0.78, 0.76, 0.72), None), |look| {
        (pale(main_colour(look)), Some(pale(second_colour(look))))
    });
    let heap = materials.add(matte(main, 1.0));
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The paper, turned a little so it reads as a square.
        part(
            meshes.add(Cuboid::new(1.3, 0.03, 1.3)),
            &paper,
            Transform::from_xyz(0.0, -0.3, 0.0).with_rotation(Quat::from_rotation_y(0.5)),
        ),
        // The heap, a low cone.
        part(
            meshes.add(Cone {
                radius: 0.55,
                height: 0.5,
            }),
            &heap,
            Transform::from_xyz(0.0, -0.04, 0.0),
        ),
    ];
    if let Some(second) = second {
        // A dusting of the second colour round its foot.
        let dust = materials.add(matte(second, 1.0));
        for (x, z) in [(0.42, 0.2), (-0.45, 0.12), (0.1, -0.5), (-0.15, 0.48)] {
            parts.push(part(
                meshes.add(Sphere::new(0.06)),
                &dust,
                Transform::from_xyz(x, -0.27, z).with_scale(Vec3::new(1.0, 0.4, 1.0)),
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
