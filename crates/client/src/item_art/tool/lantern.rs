//! The Lantern (lantern task PR 6): a hurricane lantern. A brass oil fount at
//! the bottom, a glass chimney with a small flame inside, four iron wires
//! round it, an iron hood and vent on top and a ring to carry it by. One cell
//! wide, two long, as the item.

use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

use super::{Part, iron};

/// The glass chimney's radius, and how far the wires stand out from its middle.
const GLASS: f32 = 0.26;
const WIRES: f32 = 0.31;

pub(super) fn lantern_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let brass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.78, 0.6, 0.28),
        metallic: 0.85,
        perceptual_roughness: 0.35,
        ..default()
    });
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.88, 0.8, 0.45),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.08,
        reflectance: 0.6,
        ..default()
    });
    let flame = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.78, 0.25),
        emissive: LinearRgba::rgb(2.0, 1.2, 0.3),
        ..default()
    });
    let iron = iron(materials);
    let part = |mesh, material: &Handle<StandardMaterial>, transform| Part {
        mesh,
        material: material.clone(),
        transform,
    };
    let mut parts = vec![
        // The fount, holding the oil, and its rim.
        part(
            meshes.add(Cylinder::new(0.33, 0.28)),
            &brass,
            Transform::from_xyz(0.0, -0.85, 0.0),
        ),
        part(
            meshes.add(Torus::new(0.26, 0.33)),
            &brass,
            Transform::from_xyz(0.0, -0.7, 0.0),
        ),
        // The glass, and the flame inside it.
        part(
            meshes.add(Cylinder::new(GLASS, 0.75)),
            &glass,
            Transform::from_xyz(0.0, -0.32, 0.0),
        ),
        part(
            meshes.add(Cone {
                radius: 0.07,
                height: 0.24,
            }),
            &flame,
            Transform::from_xyz(0.0, -0.42, 0.0),
        ),
        // The hood over the glass, its vent and the ring to carry it by.
        part(
            meshes.add(ConicalFrustum {
                radius_top: 0.14,
                radius_bottom: 0.34,
                height: 0.24,
            }),
            &iron,
            Transform::from_xyz(0.0, 0.17, 0.0),
        ),
        part(
            meshes.add(Cylinder::new(0.11, 0.14)),
            &iron,
            Transform::from_xyz(0.0, 0.35, 0.0),
        ),
        part(
            meshes.add(Torus::new(0.2, 0.25)),
            &iron,
            Transform::from_xyz(0.0, 0.64, 0.0).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        ),
    ];
    // Four wires guarding the glass, from the fount's rim to the hood.
    for n in 0..4 {
        let angle = TAU * (n as f32 + 0.5) / 4.0;
        let (x, z) = (angle.cos() * WIRES, angle.sin() * WIRES);
        parts.push(part(
            meshes.add(Cylinder::new(0.025, 0.8)),
            &iron,
            Transform::from_xyz(x, -0.3, z),
        ));
    }
    parts
}
