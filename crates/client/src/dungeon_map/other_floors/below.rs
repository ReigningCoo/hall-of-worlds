//! The floors below's entities: a lowered parent per storey down, holding a
//! mesh per `Surface` in sight and one remembered, each with its own material.

use bevy::camera::visibility::NoFrustumCulling;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use super::super::mesh::Surface;
use super::super::sight_lines::SurfaceMaterial;

/// One storey down's meshes and materials, in `Surface::ALL` order.
pub(super) struct BelowFloor {
    pub(super) root: Entity,
    pub(super) solid: Vec<(Entity, Handle<Mesh>, Handle<SurfaceMaterial>)>,
    pub(super) memory: Vec<(Entity, Handle<Mesh>, Handle<SurfaceMaterial>)>,
}

/// The floors below made so far, by storeys down less one; kept (hidden) when
/// not needed, as a building's floors are few.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct Below {
    pub(super) floors: Vec<BelowFloor>,
    /// What the last rebuild cost, in milliseconds, for `ViewStats`.
    pub(super) cost_ms: f32,
}

impl Below {
    /// Makes floors under the scene's root until there are `count`.
    pub(super) fn make(
        &mut self,
        count: usize,
        commands: &mut Commands,
        scene: Entity,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<SurfaceMaterial>,
        like: &[Handle<SurfaceMaterial>],
    ) {
        while self.floors.len() < count {
            let root = commands
                .spawn((Transform::default(), Visibility::Hidden, ChildOf(scene)))
                .id();
            let mut part = |material: &Handle<SurfaceMaterial>| {
                // A copy, given its own colour and look by `follow_materials`.
                let material = materials
                    .get(material)
                    .cloned()
                    .map(|copy| materials.add(copy))
                    .unwrap_or_default();
                let mesh = meshes.add(Mesh::from(Cuboid::from_length(0.0)));
                let entity = commands
                    .spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Visibility::Hidden,
                        // Replaced as things change, so its bounds go stale.
                        NoFrustumCulling,
                        NotShadowCaster,
                        ChildOf(root),
                    ))
                    .id();
                (entity, mesh, material)
            };
            let solid = Surface::ALL
                .iter()
                .map(|&s| part(&like[s as usize]))
                .collect();
            let memory = Surface::ALL
                .iter()
                .map(|&s| part(&like[s as usize]))
                .collect();
            self.floors.push(BelowFloor {
                root,
                solid,
                memory,
            });
        }
    }
}
