//! Casks on racks (tavern furniture plan PR 1): each cask the server last
//! told of on a cask rack (`common::Benches`) drawn in its own place
//! (`furniture::cask_spot`), by its size and wood, with black outlines.
//! Remembered like benches' tools: in sepia where the sight mask says out of
//! sight, in colour too where it says in sight.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::cask::CaskLook;
use shared::furniture::Held;

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::trees::TreeMaterials;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{Benches, CurrentDungeon, DrawnGround};

mod models;

pub(super) use models::CaskModels;

/// One cask as drawn, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Drawn {
    lit: bool,
    look: CaskLook,
    /// Where it lies, in tenths of a unit.
    at: IVec3,
}

/// The casks drawn now, by rack and place.
#[derive(Resource, Default)]
pub(super) struct DrawnCasks {
    casks: HashMap<((i64, i64), usize), (Entity, Drawn)>,
    /// What was seen last time.
    looks: Option<u64>,
}

/// Draws the casks again when what is seen, the racks, their casks or the
/// ground change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_rack_casks(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, benches): (Res<CurrentDungeon>, Res<Shown>, Res<Benches>),
    heights: Res<DrawnGround>,
    (lit, sepia): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<CaskModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnCasks>,
) {
    let unchanged = !dungeon.is_changed() && !heights.is_changed() && !benches.is_changed();
    if drawn.looks == Some(shown.looks) && unchanged {
        return;
    }
    drawn.looks = Some(shown.looks);
    let mut keep = HashMap::new();
    for (&at, held) in &benches.0 {
        let Held::Casks(casks) = held else {
            continue;
        };
        let Some(piece) = dungeon.level.furniture_at(at.0, at.1) else {
            continue;
        };
        if piece.at != at {
            continue;
        }
        let cells = || piece.cells();
        if !cells().any(|(x, y)| dungeon.remembered.is_seen(x, y)) {
            continue;
        }
        let in_sight =
            (shown.sight.as_ref()).is_some_and(|sight| cells().any(|(x, y)| sight.is_seen(x, y)));
        // Level at the highest ground under it, as the rack's top is.
        let ground = (cells())
            .map(|(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
            .fold(f32::MIN, f32::max);
        for (index, cask) in casks.places().iter().enumerate() {
            let Some(look) = cask.as_ref().and_then(|item| item.cask()).map(|c| c.look()) else {
                continue;
            };
            let Some((spot, turn)) = super::furniture::cask_spot(piece, index) else {
                continue;
            };
            let place = spot + Vec3::Y * ground;
            let now = Drawn {
                lit: in_sight,
                look,
                at: (place * 10.0).round().as_ivec3(),
            };
            let key = (at, index);
            if let Some((entity, was)) = drawn.casks.remove(&key) {
                if was == now {
                    keep.insert(key, (entity, was));
                    continue;
                }
                commands.entity(entity).despawn();
            }
            let model = models.get(look, &mut meshes);
            let mut spawned = commands.spawn((
                Transform::from_translation(place).with_rotation(turn),
                Visibility::default(),
            ));
            spawned.with_children(|cask| {
                let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>| {
                    cask.spawn((
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
    for (_, (entity, _)) in drawn.casks.drain() {
        commands.entity(entity).despawn();
    }
    drawn.casks = keep;
}
