//! The Hall of Worlds' meshes drawn bent and twisted (`common::HallBend`):
//! every vertex turned round the middle line and moved by its column's
//! offset once built, its normal turned with it, so each cell becomes a
//! slight parallelogram meeting its neighbours, with no steps. The twist is
//! the whole Hall's, so this is never redone as the player walks: the view
//! rolls instead (`view3d`).

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::meshes::DungeonMeshes;
use crate::common::HallBend;

impl DungeonMeshes {
    /// Moves everything built to where the bend draws it; nothing when off.
    pub fn bend(&mut self, bend: &HallBend) {
        if *bend == HallBend::default() {
            return;
        }
        let outlined = &mut self.outlined;
        let built = (self.solid.iter_mut())
            .chain(&mut self.memory)
            .chain(&mut self.stairs_through)
            .chain([&mut self.see_through])
            .chain(
                [&mut outlined.black, &mut outlined.fill, &mut outlined.lines]
                    .into_iter()
                    .flatten(),
            );
        for mesh in built {
            bend_mesh(mesh, bend);
        }
    }
}

/// Moves each vertex of a mesh by the bend at its column, turning its normal
/// by the twist there.
fn bend_mesh(mesh: &mut Mesh, bend: &HallBend) {
    let mut columns = Vec::new();
    if let Some(VertexAttributeValues::Float32x3(places)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for place in places {
            columns.push(place[0] / CELL_SIZE);
            *place = bend.shift(Vec3::from(*place), CELL_SIZE).into();
        }
    }
    if let Some(VertexAttributeValues::Float32x3(normals)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
    {
        for (normal, &x) in normals.iter_mut().zip(&columns) {
            *normal = (bend.tilt(x) * Vec3::from(*normal)).into();
        }
    }
}
