//! Casks (`shared::cask`): staves, hoops and heads coloured by the wood, built
//! once without Bevy (`mesh.rs`, which the ground's casks use too). The
//! picture stands it upright and tips it towards the camera, as the tools'.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use shared::cask::CaskLook;

use super::Part;

mod mesh;

pub use mesh::{CaskMesh, cask_mesh, height_of, radius_share as cask_radius_share};

/// How tall the picture's cask is, against its `VIEW_WIDTH` of 2.5.
const PICTURE_HEIGHT: f32 = 2.2;
/// How far it is tipped back about x, as the tools are.
const TIP: f32 = -0.7;

/// A Bevy mesh of it, with its colours.
fn to_mesh(cask: CaskMesh) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, cask.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, cask.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, cask.colours)
    .with_inserted_indices(Indices::U32(cask.indices))
}

/// A cask's parts (`Look::Cask`): one mesh, scaled to fill its picture.
pub(super) fn cask_parts(
    look: CaskLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let scale = PICTURE_HEIGHT / height_of(look.size);
    let material = materials.add(StandardMaterial {
        perceptual_roughness: 0.75,
        reflectance: 0.2,
        ..default()
    });
    let standing =
        Transform::from_xyz(0.0, -PICTURE_HEIGHT / 2.0, 0.0).with_scale(Vec3::splat(scale));
    vec![Part {
        mesh: meshes.add(to_mesh(cask_mesh(look))),
        material,
        transform: Transform::from_rotation(Quat::from_rotation_x(TIP)) * standing,
    }]
}

/// The cells its picture fills: its item's.
pub(super) fn cells(look: CaskLook) -> (u8, u8) {
    let shape = look.size.shape();
    (shape.width, shape.height)
}

#[cfg(test)]
mod tests;
