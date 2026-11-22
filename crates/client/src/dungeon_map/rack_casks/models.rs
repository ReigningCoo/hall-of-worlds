//! Casks' models for racks: `item_art`'s cask (`cask_mesh`) as one shell
//! standing upright about its middle, at the size its piece draws its size
//! at (`furniture::cask_scale`), simplified for the map; with its black
//! outline grown out from its axis (so the staves' seams leave no cracks, as
//! the ground's casks) and a sepia copy for memory. Built once per look.

use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use shared::cask::CaskLook;

use super::super::furniture::cask_scale;
use crate::item_art::{CaskMesh, cask_mesh};
use crate::plants::Shell;

/// How fine the map's copy is (`Shell::simplify`), in world units.
const GRID: f32 = 0.25;

/// A cask's meshes: its colours, its outline and its sepia copy.
#[derive(Clone, Debug)]
pub(super) struct CaskMeshes {
    pub(super) fill: Handle<Mesh>,
    pub(super) outline: Handle<Mesh>,
    pub(super) memory: Handle<Mesh>,
}

/// Casks' models built so far, by look.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct CaskModels(HashMap<CaskLook, CaskMeshes>);

impl CaskModels {
    /// The model of a cask of `look`, built if it hasn't been.
    pub(super) fn get(&mut self, look: CaskLook, meshes: &mut Assets<Mesh>) -> CaskMeshes {
        let built = self.0.entry(look).or_insert_with(|| {
            let shell = cask_shell(look);
            let mut grown = shell.clone();
            grown.normals = outward(&shell.positions);
            CaskMeshes {
                fill: meshes.add(shell.fill()),
                outline: meshes.add(grown.outline(1.0)),
                memory: meshes.add(shell.memory()),
            }
        });
        built.clone()
    }
}

/// A cask of `look` as one shell, upright, its middle at the origin.
fn cask_shell(look: CaskLook) -> Shell {
    let CaskMesh {
        positions,
        normals,
        colours,
        indices,
    } = cask_mesh(look);
    let height = crate::item_art::cask_height(look.size);
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices));
    let scale = cask_scale(look.size);
    let place = Transform::from_scale(Vec3::splat(scale)).with_translation(Vec3::new(
        0.0,
        -height / 2.0 * scale,
        0.0,
    ));
    let mut shell = Shell::default();
    shell.add(&mesh, place, Vec3::ONE);
    shell.simplify(GRID);
    shell
}

/// Normals pointing out from the cask's middle, squashed to its shape, for
/// its outline.
fn outward(positions: &[Vec3]) -> Vec<Vec3> {
    let (radius, half) = (positions.iter()).fold((0.0f32, 0.0f32), |(r, h), p| {
        (r.max(p.x.hypot(p.z)), h.max(p.y.abs()))
    });
    let (radius, half) = (radius.max(1e-6), half.max(1e-6));
    (positions.iter())
        .map(|p| Vec3::new(p.x / radius, p.y / half, p.z / radius).normalize_or(Vec3::Y))
        .collect()
}
