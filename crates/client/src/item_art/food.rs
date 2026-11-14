//! Placeholder food (owner, 2026-11-14): a loaf of bread, Dwarven Bread like a
//! lump of stone, and a slab of raw meat with its fat and a bone. Each lies
//! along the picture's width (x), two cells wide and one long.

use bevy::prelude::*;
use shared::item::ItemKind;

use super::Part;

/// The parts of one of the foods with a picture (`Look::Food`).
pub(super) fn food_parts(
    kind: ItemKind,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut colour = |red, green, blue, roughness| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(red, green, blue),
            perceptual_roughness: roughness,
            reflectance: 0.15,
            ..default()
        })
    };
    let blob = |meshes: &mut Assets<Mesh>, material, at: Vec3, scale: Vec3| Part {
        mesh: meshes.add(Sphere::new(1.0)),
        material,
        transform: Transform::from_translation(at).with_scale(scale),
    };
    match kind {
        ItemKind::Bread => {
            let crust = colour(0.72, 0.47, 0.22, 0.85);
            let score = colour(0.93, 0.82, 0.6, 0.9);
            let mut parts = vec![blob(meshes, crust, Vec3::ZERO, Vec3::new(1.1, 0.45, 0.5))];
            // Three slashes across the top, showing the paler crumb.
            for x in [-0.5, 0.0, 0.5] {
                parts.push(Part {
                    mesh: meshes.add(Cuboid::new(0.08, 0.06, 0.6)),
                    material: score.clone(),
                    transform: Transform::from_xyz(x, 0.42, 0.0)
                        .with_rotation(Quat::from_rotation_y(0.5)),
                });
            }
            parts
        }
        ItemKind::DwarvenBread => {
            let stone = colour(0.5, 0.46, 0.4, 1.0);
            let darker = colour(0.4, 0.36, 0.31, 1.0);
            vec![
                Part {
                    mesh: meshes.add(Cuboid::new(1.8, 0.55, 0.85)),
                    material: stone,
                    transform: Transform::from_rotation(Quat::from_rotation_y(0.08)),
                },
                // A lumpier top, as if hewn rather than baked.
                Part {
                    mesh: meshes.add(Cuboid::new(1.1, 0.3, 0.6)),
                    material: darker,
                    transform: Transform::from_xyz(-0.2, 0.32, 0.05)
                        .with_rotation(Quat::from_rotation_y(-0.15)),
                },
            ]
        }
        _ => {
            let meat = colour(0.62, 0.1, 0.12, 0.6);
            let fat = colour(0.95, 0.9, 0.82, 0.7);
            vec![
                blob(
                    meshes,
                    fat,
                    Vec3::new(-0.1, -0.03, 0.0),
                    Vec3::new(0.95, 0.24, 0.5),
                ),
                blob(
                    meshes,
                    meat,
                    Vec3::new(-0.15, 0.0, 0.0),
                    Vec3::new(0.85, 0.27, 0.44),
                ),
                // The bone, sticking out at one end, with its knob.
                Part {
                    mesh: meshes.add(Cylinder::new(0.09, 0.7)),
                    material: colour(0.96, 0.94, 0.86, 0.8),
                    transform: Transform::from_xyz(0.8, 0.05, 0.0)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                },
                blob(
                    meshes,
                    colour(0.96, 0.94, 0.86, 0.8),
                    Vec3::new(1.15, 0.05, 0.0),
                    Vec3::splat(0.15),
                ),
            ]
        }
    }
}
