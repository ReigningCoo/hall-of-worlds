//! Vessels (`shared::vessel`): tankards, cups, goblets, drinking horns, wine
//! bottles, jugs, flasks and waterskins, each built upright from simple
//! shapes (y up) and tipped towards the camera, as the tools are. A wine
//! bottle is tall, dark and corked, unlike a potion's small vial.

use bevy::prelude::*;
use shared::vessel::VesselKind;

use super::Part;

mod big;
mod cups;
mod horn;
mod narrow;

/// How far a vessel is tipped back about x, so its front faces the camera.
const TIP: f32 = -0.6;

/// A vessel's parts (`Look::Vessel`), against the picture's `VIEW_WIDTH` of 2.5
/// (a long one's built twice as big and halved).
pub(super) fn vessel_parts(
    vessel: VesselKind,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mut kit = Kit {
        meshes,
        materials,
        parts: Vec::new(),
    };
    match vessel {
        VesselKind::Tankard => cups::tankard(&mut kit),
        VesselKind::Cup => cups::cup(&mut kit),
        VesselKind::Goblet => cups::goblet(&mut kit),
        VesselKind::DrinkingHorn => horn::horn(&mut kit),
        VesselKind::WineBottle => narrow::wine_bottle(&mut kit),
        VesselKind::Flask => narrow::flask(&mut kit),
        VesselKind::Jug => big::jug(&mut kit),
        VesselKind::Waterskin => big::waterskin(&mut kit),
    }
    // Built for a square picture: the longer side of a long one spans the
    // same `VIEW_WIDTH`, so it is drawn smaller by its shorter side's share.
    let (wide, long) = cells(vessel);
    let share = f32::from(wide.min(long)) / f32::from(wide.max(long));
    let tipped =
        Transform::from_rotation(Quat::from_rotation_x(TIP)).with_scale(Vec3::splat(share));
    kit.parts
        .into_iter()
        .map(|part| Part {
            transform: tipped * part.transform,
            ..part
        })
        .collect()
}

/// The cells a vessel's picture fills: its item's.
pub(super) fn cells(vessel: VesselKind) -> (u8, u8) {
    let shape = vessel.shape();
    (shape.width, shape.height)
}

/// What a vessel is built with: the assets and the parts so far.
struct Kit<'a> {
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<StandardMaterial>,
    parts: Vec<Part>,
}

impl Kit<'_> {
    /// Adds a part of this shape, look and place.
    fn add(&mut self, mesh: impl Into<Mesh>, material: &Handle<StandardMaterial>, at: Transform) {
        self.parts.push(Part {
            mesh: self.meshes.add(mesh.into()),
            material: material.clone(),
            transform: at,
        });
    }

    /// A look that isn't metal: this colour and roughness.
    fn matte(&mut self, colour: [f32; 3], roughness: f32) -> Handle<StandardMaterial> {
        let [red, green, blue] = colour;
        self.materials.add(StandardMaterial {
            base_color: Color::srgb(red, green, blue),
            perceptual_roughness: roughness,
            reflectance: 0.15,
            ..default()
        })
    }

    /// A metal of this colour, a little shiny.
    fn metal(&mut self, colour: [f32; 3]) -> Handle<StandardMaterial> {
        let [red, green, blue] = colour;
        self.materials.add(StandardMaterial {
            base_color: Color::srgb(red, green, blue),
            metallic: 0.8,
            perceptual_roughness: 0.4,
            ..default()
        })
    }
}

/// Pewter, earthenware, leather, cork and the dark inside of a mouth.
const PEWTER: [f32; 3] = [0.62, 0.63, 0.64];
const CLAY: [f32; 3] = [0.7, 0.38, 0.24];
const LEATHER: [f32; 3] = [0.42, 0.26, 0.14];
const CORK: [f32; 3] = [0.66, 0.5, 0.32];
const HOLLOW: [f32; 3] = [0.08, 0.06, 0.05];

/// A handle on the right, an upright ring in the picture's plane.
fn handle(kit: &mut Kit, material: &Handle<StandardMaterial>, x: f32, y: f32, size: f32) {
    let ring = Torus::new(size * 0.75, size);
    let upright = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    kit.add(
        ring,
        material,
        Transform::from_xyz(x, y, 0.0).with_rotation(upright),
    );
}

/// A dark disc filling a mouth this wide at this height.
fn mouth(kit: &mut Kit, radius: f32, y: f32) {
    let dark = kit.matte(HOLLOW, 1.0);
    kit.add(
        Cylinder::new(radius, 0.02),
        &dark,
        Transform::from_xyz(0.0, y, 0.0),
    );
}

#[cfg(test)]
mod tests;
