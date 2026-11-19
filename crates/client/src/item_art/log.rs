//! Logs and kindling (`shared::woodcutting`, woodcutting plan PR 4): a log
//! lying along its picture with its bark's relief and colour on the side and
//! plain ringed wood on its ends, rolled up a little so the near end shows;
//! kindling as three split sticks of its wood. Both built once without Bevy
//! (`log/mesh.rs`, which the ground's log piles use too), by the wood
//! (`log/bark.rs`: the research's bark, drawn procedurally).

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use shared::material::Material;

use super::Part;

mod bark;
mod kindling;
mod mesh;
mod pattern;

pub use mesh::{GROUND_DETAIL, LOG_LENGTH, LOG_RADIUS, LogMesh, log_mesh};

/// How long the picture's log is, against its `VIEW_WIDTH` of 2.5 across
/// eight cells; and the kindling's sticks across two.
const LOG_DRAWN: f32 = 2.45;
const STICKS_DRAWN: f32 = 2.0;
/// How far a log is rolled up at its right end and turned, so that end shows.
const LIFT: f32 = 0.6;
const TURN: f32 = 0.06;

/// A Bevy mesh of it, with its colours.
pub fn to_mesh(log: LogMesh) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, log.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, log.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, log.colours)
    .with_inserted_indices(Indices::U32(log.indices))
}

fn bark_material(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        perceptual_roughness: 0.92,
        reflectance: 0.15,
        ..default()
    })
}

/// A log's parts (`Look::Log`): one mesh, filling its eight cells.
pub(super) fn log_parts(
    wood: Material,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let scale = LOG_DRAWN / LOG_LENGTH;
    let turned = Quat::from_rotation_y(TURN) * Quat::from_rotation_z(LIFT);
    vec![Part {
        mesh: meshes.add(to_mesh(log_mesh(wood, mesh::PICTURE_DETAIL, 1))),
        material: bark_material(materials),
        transform: Transform::from_rotation(turned).with_scale(Vec3::splat(scale)),
    }]
}

/// Kindling's parts (`Look::Kindling`): the sticks, filling its two cells.
pub(super) fn kindling_parts(
    wood: Material,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let scale = STICKS_DRAWN / kindling::STICK_LENGTH;
    vec![Part {
        mesh: meshes.add(to_mesh(kindling::kindling_mesh(wood))),
        material: bark_material(materials),
        transform: Transform::from_rotation(Quat::from_rotation_z(0.35))
            .with_scale(Vec3::splat(scale)),
    }]
}

#[cfg(test)]
mod tests;
