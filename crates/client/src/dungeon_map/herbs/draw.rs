//! Placing herb patches' plants: when what is seen, the hour or the patches
//! change, works out which patches show at which stage, and spawns or
//! despawns only those that differ.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::rng::Seed;

use super::super::town::cell_centre;
use super::super::{CELL_SIZE, DungeonScene, Shown};
use super::{HerbMaterials, shown_stage};
use crate::common::{
    CurrentDungeon, DrawnGround, HerbGrowthShift, HerbShowcase, HerbTable, MapPosition,
    ServerClock, WorldReturn,
};
use crate::plants::{PlantModels, patch_layout};
use crate::player::Player;

/// For each patch's own seed: where its plants stand and which they are.
const PATCHES: Seed = Seed::new(0x68_6572_6273);

/// What a drawn patch shows, to tell when it must be drawn again.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Drawn {
    herb: String,
    stage: u8,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// The patches drawn now, by cell, and what they were drawn for.
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct DrawnPatches {
    patches: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen, the hour, the shift and the showcase last time.
    built: Option<(u64, u64, u32, Option<u8>)>,
}

/// Draws the patches in sight again when what is seen, the hour, the patches,
/// the herbs, the ground or the debug settings change: a patch moves on a
/// stage at most every few game hours.
#[allow(clippy::too_many_arguments)]
pub(in crate::dungeon_map) fn update_herbs(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, table): (Res<CurrentDungeon>, Res<Shown>, Res<HerbTable>),
    (clock, shift, showcase): (Res<ServerClock>, Res<HerbGrowthShift>, Res<HerbShowcase>),
    (place, heights): (Res<WorldReturn>, Res<DrawnGround>),
    player: Single<&MapPosition, With<Player>>,
    materials: Res<HerbMaterials>,
    mut models: ResMut<PlantModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnPatches>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60, shift.0, showcase.0);
    let changed = dungeon.is_changed() || table.is_changed() || heights.is_changed();
    if drawn.built == Some(key) && !changed {
        return;
    }
    drawn.built = Some(key);
    let ground_at =
        |at: Vec3| heights.drawn_between(at.x / CELL_SIZE, at.z / CELL_SIZE) * CELL_SIZE;
    let wanted: Vec<((i64, i64), String, u8)> = match showcase.0 {
        Some(stage) => super::showcase::cells(&table, (player.x, player.y), stage),
        None => shown.sight.as_ref().map_or_else(Vec::new, |sight| {
            dungeon
                .level
                .herbs()
                .filter(|&((x, y), _)| sight.is_seen(x, y))
                .filter_map(|(at, patch)| {
                    let def = table.get(&patch.herb)?;
                    let stage = patch.stage(def, now);
                    (stage > 0).then(|| (at, patch.herb.clone(), shown_stage(stage, shift.0)))
                })
                .collect()
        }),
    };
    let mut keep = HashMap::with_capacity(wanted.len());
    for ((x, y), herb, stage) in wanted {
        let middle = cell_centre(x, y);
        let ground = (ground_at(middle) * 10.0).round() as i32;
        let now_drawn = Drawn {
            herb,
            stage,
            ground,
        };
        if let Some((entity, was)) = drawn.patches.remove(&(x, y)) {
            if was == now_drawn {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let Some(def) = table.get(&now_drawn.herb) else {
            continue;
        };
        let seed = PATCHES.at(&[place.0.x, place.0.y, x, y]);
        let patch = commands
            .spawn((Transform::IDENTITY, Visibility::Inherited))
            .id();
        commands.entity(*scene).add_child(patch);
        for spot in patch_layout(&def.plant, seed) {
            let foot = middle + Vec3::new(spot.offset.x, 0.0, spot.offset.y);
            let foot = foot.with_y(ground_at(foot));
            let model = models.get(def, stage, spot.variant, &mut meshes);
            let transform = Transform::from_translation(foot)
                .with_rotation(Quat::from_rotation_y(spot.turn))
                .with_scale(Vec3::splat(spot.scale));
            let plant = commands
                .spawn((
                    transform,
                    Mesh3d(model.fill),
                    MeshMaterial3d(materials.fill.clone()),
                    NotShadowCaster,
                ))
                .with_child((
                    Mesh3d(model.outline),
                    MeshMaterial3d(materials.outline.clone()),
                    NotShadowCaster,
                ))
                .id();
            commands.entity(patch).add_child(plant);
        }
        keep.insert((x, y), (patch, now_drawn));
    }
    for (_, (entity, _)) in drawn.patches.drain() {
        commands.entity(entity).despawn();
    }
    drawn.patches = keep;
}
