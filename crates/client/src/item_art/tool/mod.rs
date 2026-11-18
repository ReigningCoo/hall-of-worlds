//! Brewing's tools (`shared::ability::Ability::tool`): the cauldron, the mortar
//! and pestle and the oil press; and the tap casks are drawn from (`tap.rs`). Each is built standing upright (y up) and then
//! tipped towards the camera, so its picture shows it from the front and a
//! little above rather than from the top.

use bevy::prelude::*;
use shared::item::ItemKind;

use super::Part;

mod cauldron;
mod mortar;
mod oil_press;
mod tap;

/// How far a tool is tipped back about x, so its front faces the camera more.
const TIP: f32 = -0.7;

/// The parts of one of the tools (`Look::Tool`), against the picture's
/// `VIEW_WIDTH` of 2.5.
pub(super) fn tool_parts(
    kind: ItemKind,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let parts = match kind {
        ItemKind::Cauldron => cauldron::cauldron_parts(meshes, materials),
        ItemKind::MortarAndPestle => mortar::mortar_parts(meshes, materials),
        ItemKind::Tap => tap::tap_parts(meshes, materials),
        _ => oil_press::oil_press_parts(meshes, materials),
    };
    let tipped = Transform::from_rotation(Quat::from_rotation_x(TIP));
    parts
        .into_iter()
        .map(|part| Part {
            transform: tipped * part.transform,
            ..part
        })
        .collect()
}

/// The cells a tool's picture fills, wide and long: its item's.
pub(super) fn cells(kind: ItemKind) -> (u8, u8) {
    let shape = kind.def().shape;
    (shape.width, shape.height)
}

/// A material with this colour and roughness, not metal.
fn matte(red: f32, green: f32, blue: f32, roughness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(red, green, blue),
        perceptual_roughness: roughness,
        reflectance: 0.15,
        ..default()
    }
}

/// Dark iron, a little shiny.
fn iron(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.32),
        metallic: 0.7,
        perceptual_roughness: 0.55,
        ..default()
    })
}
