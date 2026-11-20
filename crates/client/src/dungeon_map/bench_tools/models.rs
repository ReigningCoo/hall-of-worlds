//! The tools' models for benches: each tool's pieces (`item_art::tool_pieces`)
//! put together as one shell, stood on its foot, made the size a tool on a
//! bench is drawn at and simplified for the map; with its black outline and a
//! sepia copy for memory, as fungus patches have. Built once per kind.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::item::ItemKind;

use super::super::CELL_SIZE;
use crate::item_art::tool_pieces;
use crate::plants::Shell;

/// How far across the ground a tool may reach from its middle.
const REACH: f32 = CELL_SIZE * 0.17;
/// How tall a tool may stand.
const TALLEST: f32 = CELL_SIZE * 0.34;
/// How fine the map's copy is (`Shell::simplify`), in world units.
const GRID: f32 = 0.25;

/// A tool's meshes: its colours, its outline and its sepia copy.
#[derive(Clone, Debug)]
pub(in crate::dungeon_map) struct ToolMeshes {
    pub(in crate::dungeon_map) fill: Handle<Mesh>,
    pub(super) outline: Handle<Mesh>,
    pub(super) memory: Handle<Mesh>,
}

/// Tools' models built so far; `None` for a kind with nothing to draw.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct ToolModels(HashMap<ItemKind, Option<ToolMeshes>>);

impl ToolModels {
    /// The model of a tool of `kind`, built if it hasn't been.
    pub(in crate::dungeon_map) fn get(
        &mut self,
        kind: ItemKind,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Option<ToolMeshes> {
        let built = self.0.entry(kind).or_insert_with(|| {
            let shell = tool_shell(kind, meshes, materials)?;
            Some(ToolMeshes {
                fill: meshes.add(shell.fill()),
                outline: meshes.add(shell.outline(0.5)),
                memory: meshes.add(shell.memory()),
            })
        });
        built.clone()
    }
}

/// A tool as one shell, its foot's middle at the origin, fitted within
/// `REACH` and `TALLEST`; `None` if it has no pieces.
pub(super) fn tool_shell(
    kind: ItemKind,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Option<Shell> {
    let mut shell = Shell::default();
    for piece in tool_pieces(kind, meshes, materials) {
        shell.add(&piece.mesh, piece.place, piece.colour);
    }
    if shell.is_empty() {
        return None;
    }
    let (low, high) = (shell.positions.iter()).fold(
        (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN)),
        |(low, high), point| (low.min(*point), high.max(*point)),
    );
    let foot = Vec3::new((low.x + high.x) / 2.0, low.y, (low.z + high.z) / 2.0);
    let size = high - low;
    let across = (size.x.max(size.z) / 2.0).max(f32::EPSILON);
    let scale = (REACH / across).min(TALLEST / size.y.max(f32::EPSILON));
    for point in &mut shell.positions {
        *point = (*point - foot) * scale;
    }
    shell.simplify(GRID);
    Some(shell)
}

#[cfg(test)]
mod tests;
