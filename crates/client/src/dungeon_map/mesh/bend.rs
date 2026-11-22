//! The Hall of Worlds' meshes drawn bent (`common::HallBend`): every vertex
//! moved by its column's offset once built, so each cell becomes a slight
//! parallelogram meeting its neighbours, with no steps.

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

/// Moves each vertex of a mesh by the bend at its column.
fn bend_mesh(mesh: &mut Mesh, bend: &HallBend) {
    if let Some(VertexAttributeValues::Float32x3(places)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for place in places {
            *place = bend.shift(Vec3::from(*place), CELL_SIZE).into();
        }
    }
}
