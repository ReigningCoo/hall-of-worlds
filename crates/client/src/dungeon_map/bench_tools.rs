//! Crafting benches' tools (crafting benches plan PR 3): each tool the server
//! last told of on a bench (`common::Benches`) drawn as its model in its own
//! place on the bench (`furniture::tool_spot`): tall ones on the shelf, the
//! rest in front. Remembered like fungus patches: in sepia where the sight
//! mask says out of sight, in colour too where it says in sight.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::furniture::Furniture;
use shared::item::ItemKind;

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::trees::TreeMaterials;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{Benches, CurrentDungeon, DrawnGround};

mod models;

pub(super) use models::ToolModels;

/// One tool as drawn, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Drawn {
    lit: bool,
    /// Where it stands, in tenths of a unit.
    at: IVec3,
}

/// The tools drawn now, by bench and kind.
#[derive(Resource, Default)]
pub(super) struct DrawnTools {
    tools: HashMap<((i64, i64), ItemKind), (Entity, Drawn)>,
    /// What was seen last time.
    looks: Option<u64>,
}

/// Draws the tools again when what is seen, the benches, their tools or the
/// ground change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_bench_tools(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, benches): (Res<CurrentDungeon>, Res<Shown>, Res<Benches>),
    heights: Res<DrawnGround>,
    (lit, sepia): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<ToolModels>,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
    mut drawn: ResMut<DrawnTools>,
) {
    let unchanged = !dungeon.is_changed() && !heights.is_changed() && !benches.is_changed();
    if drawn.looks == Some(shown.looks) && unchanged {
        return;
    }
    drawn.looks = Some(shown.looks);
    let mut keep = HashMap::new();
    for (&at, held) in &benches.0 {
        let Some(tools) = held.bench() else {
            continue;
        };
        let Some(piece) = dungeon.level.furniture_at(at.0, at.1) else {
            continue;
        };
        if !matches!(piece.furniture, Furniture::CraftingBench { .. }) || piece.at != at {
            continue;
        }
        let cells = || piece.cells();
        if !cells().any(|(x, y)| dungeon.remembered.is_seen(x, y)) {
            continue;
        }
        let in_sight =
            (shown.sight.as_ref()).is_some_and(|sight| cells().any(|(x, y)| sight.is_seen(x, y)));
        // Level at the highest ground under it, as the bench's top is.
        let ground = (cells())
            .map(|(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
            .fold(f32::MIN, f32::max);
        for tool in tools.tools() {
            let Some((spot, _)) = super::furniture::tool_spot(piece, tool.kind) else {
                continue;
            };
            let place = spot + Vec3::Y * ground;
            let now = Drawn {
                lit: in_sight,
                at: (place * 10.0).round().as_ivec3(),
            };
            let key = (at, tool.kind);
            if let Some((entity, was)) = drawn.tools.remove(&key) {
                if was == now {
                    keep.insert(key, (entity, was));
                    continue;
                }
                commands.entity(entity).despawn();
            }
            let Some(model) = models.get(tool.kind, &mut meshes, &mut materials) else {
                continue;
            };
            let facing = piece.facing.beside((0, 0));
            let turn = (facing.0 as f32).atan2(facing.1 as f32);
            let mut spawned = commands.spawn((
                Transform::from_translation(place).with_rotation(Quat::from_rotation_y(turn)),
                Visibility::default(),
            ));
            spawned.with_children(|tool| {
                let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>| {
                    tool.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        NotShadowCaster,
                    ));
                };
                copy(&model.memory, &sepia.memory);
                copy(&model.outline, &sepia.outline);
                if now.lit {
                    copy(&model.fill, &lit.fill);
                    copy(&model.outline, &lit.outline);
                }
            });
            let entity = spawned.id();
            commands.entity(*scene).add_child(entity);
            keep.insert(key, (entity, now));
        }
    }
    for (_, (entity, _)) in drawn.tools.drain() {
        commands.entity(entity).despawn();
    }
    drawn.tools = keep;
}
