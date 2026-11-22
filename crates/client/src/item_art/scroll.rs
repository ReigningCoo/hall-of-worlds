//! Recipe scrolls (`Look::Recipe`; recipes plan PR 3): a rolled-up sheet of
//! parchment lying across the picture, its rolled ends a shade darker, tied
//! round the middle with a ribbon, and its craft's emblem standing on it,
//! upright whatever the angle. The angle and emblem come from the craft
//! (`ScrollLook`), so Alchemy's scrolls can lie the other way with their own
//! picture: Cooking's lie from bottom left to top right with a chef's hat.
//! One cell, the same for every dish.

use bevy::prelude::*;
use shared::recipe::Craft;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use super::Part;

/// How far an emblem is tipped towards the camera, so it shows its front.
const TIP: f32 = 0.5;
/// The roll's radius and length, against the picture's `VIEW_WIDTH` of 2.5.
const RADIUS: f32 = 0.27;
const LENGTH: f32 = 1.85;

/// What stands on a scroll, by its craft.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Emblem {
    /// Cooking's: a white chef's hat, a band below a puffed top.
    ChefsHat,
}

/// How a scroll of a craft is drawn: one picture per craft.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScrollLook {
    pub craft: Craft,
}

impl ScrollLook {
    /// The angle the roll lies at in the picture (radians anticlockwise from
    /// across it, as seen) and the emblem on it.
    fn style(self) -> (f32, Emblem) {
        match self.craft {
            // Alchemy has no scrolls yet: its own angle and emblem later.
            Craft::Cooking | Craft::Alchemy => (FRAC_PI_4, Emblem::ChefsHat),
        }
    }
}

/// A scroll's parts (`Look::Recipe`).
pub(super) fn scroll_parts(
    look: ScrollLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let (angle, emblem) = look.style();
    let mut parts = roll_parts(angle, meshes, materials);
    parts.extend(emblem_parts(emblem, meshes, materials));
    parts
}

/// A material of this colour, matt as paper and cloth are.
fn matt(
    materials: &mut Assets<StandardMaterial>,
    (r, g, b): (f32, f32, f32),
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(r, g, b),
        perceptual_roughness: 0.85,
        reflectance: 0.2,
        ..default()
    })
}

/// The roll along the picture at `angle`, with the ribbon round it. Built
/// along x with +y towards the camera (which looks down, -z up the picture),
/// then turned about y.
fn roll_parts(
    angle: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let parchment = matt(materials, (0.93, 0.86, 0.68));
    let edge = matt(materials, (0.78, 0.68, 0.48));
    let ribbon = matt(materials, (0.68, 0.12, 0.1));
    let along = Quat::from_rotation_z(FRAC_PI_2);
    let half = LENGTH / 2.0;
    let mut parts = vec![Part {
        mesh: meshes.add(Cylinder::new(RADIUS, LENGTH)),
        material: parchment,
        transform: Transform::from_rotation(along),
    }];
    // The rolled ends, the sheet's edge spiralling in.
    for x in [-half, half] {
        for (radius, ring) in [(RADIUS * 0.92, 0.025), (RADIUS * 0.55, 0.02)] {
            parts.push(Part {
                mesh: meshes.add(Torus::new(radius - ring, radius + ring)),
                material: edge.clone(),
                transform: Transform::from_xyz(x, 0.0, 0.0).with_rotation(along),
            });
        }
    }
    // The ribbon tied round the middle, a little to one side of the emblem.
    for x in [-0.42, 0.42] {
        parts.push(Part {
            mesh: meshes.add(Cylinder::new(RADIUS * 1.06, 0.12)),
            material: ribbon.clone(),
            transform: Transform::from_xyz(x, 0.0, 0.0).with_rotation(along),
        });
    }
    let turn = Transform::from_rotation(Quat::from_rotation_y(angle));
    parts
        .into_iter()
        .map(|part| Part {
            transform: turn * part.transform,
            ..part
        })
        .collect()
}

/// The emblem, built standing (+y up), tipped towards the camera and set on
/// the middle of the roll, upright in the picture.
fn emblem_parts(
    emblem: Emblem,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let parts = match emblem {
        Emblem::ChefsHat => chefs_hat(meshes, materials),
    };
    let facing = Transform::from_rotation(Quat::from_rotation_x(TIP - FRAC_PI_2));
    // On the roll's top, its foot a little down the picture.
    let on_roll = Transform::from_xyz(0.0, RADIUS, 0.12).with_scale(Vec3::splat(0.82));
    parts
        .into_iter()
        .map(|part| Part {
            transform: on_roll * facing * part.transform,
            ..part
        })
        .collect()
}

/// A chef's hat: a white band, and a puffed top of three bulges over it.
fn chefs_hat(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Vec<Part> {
    let white = matt(materials, (0.97, 0.97, 0.95));
    let shade = matt(materials, (0.86, 0.86, 0.84));
    let mut parts = vec![
        Part {
            mesh: meshes.add(Cylinder::new(0.3, 0.34)),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, 0.17, 0.0),
        },
        // A pleated seam where the top puffs out of the band.
        Part {
            mesh: meshes.add(Torus::new(0.28, 0.33)),
            material: shade,
            transform: Transform::from_xyz(0.0, 0.34, 0.0),
        },
    ];
    for (x, y, size) in [(-0.2, 0.55, 0.25), (0.2, 0.55, 0.25), (0.0, 0.66, 0.29)] {
        parts.push(Part {
            mesh: meshes.add(Sphere::new(size)),
            material: white.clone(),
            transform: Transform::from_xyz(x, y, 0.0),
        });
    }
    parts
}
