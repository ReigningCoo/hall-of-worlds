//! Bottles on wine and spirit racks (tavern furniture plan PR 2): each bottle
//! the server last told of on one (`common::Benches`) drawn in its own place
//! (`furniture::bottle_spot`), with black outlines. Remembered like casks on
//! racks: in sepia where the sight mask says out of sight, in colour too where
//! it says in sight.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::furniture::Held;

use super::herbs::HerbMaterials;
use super::sight_lines::SurfaceMaterial;
use super::trees::TreeMaterials;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{Benches, CurrentDungeon, DrawnGround};

mod models;

pub(super) use models::BottleModels;
use models::RackBottleLook;

/// One bottle as drawn, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Drawn {
    lit: bool,
    look: RackBottleLook,
    /// Where it is, in tenths of a unit.
    at: IVec3,
}

/// The bottles drawn now, by rack and place.
#[derive(Resource, Default)]
pub(super) struct DrawnBottles {
    bottles: HashMap<((i64, i64), usize), (Entity, Drawn)>,
    /// What was seen last time.
    looks: Option<u64>,
}

/// Draws the bottles again when what is seen, the racks, their bottles or
/// the ground change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_rack_bottles(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, benches): (Res<CurrentDungeon>, Res<Shown>, Res<Benches>),
    heights: Res<DrawnGround>,
    (lit, sepia): (Res<HerbMaterials>, Res<TreeMaterials>),
    mut models: ResMut<BottleModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnBottles>,
) {
    let unchanged = !dungeon.is_changed() && !heights.is_changed() && !benches.is_changed();
    if drawn.looks == Some(shown.looks) && unchanged {
        return;
    }
    drawn.looks = Some(shown.looks);
    let mut keep = HashMap::new();
    let level = &dungeon.level;
    for (&at, held) in &benches.0 {
        let Held::Bottles(bottles) = held else {
            continue;
        };
        let top = level.furniture_on_top_at(at.0, at.1);
        let Some(piece) = top.or(level.furniture_at(at.0, at.1)) else {
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
        // Level at the highest ground under it, as furniture's tops are.
        let ground = (cells())
            .map(|(x, y)| heights.drawn_between(x as f32, y as f32) * CELL_SIZE)
            .fold(f32::MIN, f32::max);
        for (index, bottle) in bottles.places().iter().enumerate() {
            let Some(look) = bottle.as_ref().and_then(RackBottleLook::of) else {
                continue;
            };
            let half = look.size().0 / 2.0;
            let Some((spot, turn)) = super::furniture::bottle_spot(piece, index, half) else {
                continue;
            };
            let place = spot + Vec3::Y * ground;
            let now = Drawn {
                lit: in_sight,
                look,
                at: (place * 10.0).round().as_ivec3(),
            };
            let key = (at, index);
            if let Some((entity, was)) = drawn.bottles.remove(&key) {
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
            spawned.with_children(|bottle| {
                let mut copy = |mesh: &Handle<Mesh>, material: &Handle<SurfaceMaterial>| {
                    bottle.spawn((
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
    for (_, (entity, _)) in drawn.bottles.drain() {
        commands.entity(entity).despawn();
    }
    drawn.bottles = keep;
}
