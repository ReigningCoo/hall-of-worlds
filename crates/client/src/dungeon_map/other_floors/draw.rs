//! Building the floors below again, only when what is known or seen of them,
//! the light, the looks or the settings change, or the camera has moved a
//! cell's width (walls' faces are chosen by where it is).

use bevy::platform::time::Instant;
use bevy::prelude::*;
use shared::dungeon::{Level, Sight};
use std::collections::{BTreeMap, BTreeSet};

use super::super::daylight::OutsideLight;
use super::super::mesh::{DungeonMeshes, LightGrid, View, build_posed};
use super::super::theme::Theme;
use super::super::{CELL_SIZE, DungeonScene, Shown, SurfaceMaterials, WALL_HEIGHT};
use super::below::{Below, BelowFloor};
use super::which::{drawn, floors_below, seen_now};
use crate::common::{
    CurrentDungeon, Lightning, MapPosition, OtherFloorLooks, OtherLevels, Preferences, ViewStats,
};
use crate::player::Player;
use crate::view3d::ViewCamera;

/// How far below the floor above a storey's walls stop, so their tops don't
/// fight it for depth.
const TOP_GAP: f32 = WALL_HEIGHT * 0.01;

/// What the floors below were last built for.
#[derive(Default)]
pub(in crate::dungeon_map) struct Built {
    looks: u64,
    camera: Option<Vec3>,
}

/// Builds the floors below the player's in a building, from what the client
/// has of them, and hides those not needed.
#[allow(clippy::too_many_arguments)]
pub(in crate::dungeon_map) fn draw_below(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, levels, shown): (Res<CurrentDungeon>, Res<OtherLevels>, Res<Shown>),
    player: Single<&MapPosition, With<Player>>,
    camera: Single<&Transform, With<ViewCamera>>,
    (outside, lightning, looks, preferences): (
        Res<OutsideLight>,
        Res<Lightning>,
        Res<OtherFloorLooks>,
        Res<Preferences>,
    ),
    (mut below, surfaces): (ResMut<Below>, Res<SurfaceMaterials>),
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<super::SurfaceMaterial>>),
    mut built: Local<Built>,
    mut stats: ResMut<ViewStats>,
) {
    let camera = camera.translation;
    let moved = built
        .camera
        .is_none_or(|last| last.distance(camera) > CELL_SIZE);
    let changed = moved
        || built.looks != shown.looks
        || dungeon.is_changed()
        || levels.is_changed()
        || outside.is_changed()
        || lightning.is_changed()
        || looks.is_changed()
        || preferences.is_changed();
    if !changed {
        if stats.is_changed() {
            add_cost(&mut stats, below.cost_ms);
        }
        return;
    }
    built.looks = shown.looks;
    built.camera = Some(camera);
    let start = Instant::now();
    let floors = match &dungeon.info {
        Some(info) if shown.sight.is_some() => floors_below(info),
        _ => Vec::new(),
    };
    let like = &surfaces.solid;
    below.make(
        floors.len(),
        &mut commands,
        *scene,
        &mut meshes,
        &mut materials,
        like,
    );
    for (depth, floor) in below.floors.iter().enumerate() {
        let used = depth < floors.len();
        commands.entity(floor.root).insert(if used {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
    if let (Some(info), Some(sight)) = (&dungeon.info, &shown.sight)
        && !floors.is_empty()
    {
        let storeys = (info.levels - u32::from(info.basement)) as usize;
        let known: Vec<Option<&Level>> = (0..storeys as u32)
            .map(|index| levels.level(index).map(|level| &level.level))
            .collect();
        let at = (player.x, player.y);
        let viewer = info.index as usize;
        let now = seen_now(&known, &dungeon.level, viewer, at, sight);
        let shape = Theme::of_level(Some(info.kind)).shape;
        let mut light_now = outside.in_flash(lightning.0);
        light_now = light_now.max(Vec3::splat(preferences.graphics.least_light()));
        for (depth, (index, down)) in floors.iter().enumerate() {
            let floor = &below.floors[depth];
            let Some(level) = levels.level(*index) else {
                hide(&mut commands, floor);
                continue;
            };
            // The floors between it and the player's, theirs included.
            let above: Vec<&Level> = (*index + 1..info.index)
                .filter_map(|i| known[i as usize])
                .chain([&dungeon.level])
                .collect();
            let range = (*index == 0).then_some(looks.ground_range);
            let now = &now[*index as usize];
            let draw = drawn(
                &level.level,
                &level.remembered,
                now,
                &above,
                (at, range),
                looks.remembered,
            );
            let lowered = *down as f32 * looks.storey * WALL_HEIGHT;
            let scale = (looks.storey * WALL_HEIGHT - TOP_GAP) / WALL_HEIGHT;
            let local = Vec3::new(camera.x, (camera.y + lowered) / scale, camera.z);
            let view = View {
                cell: at,
                sight: now.clone(),
            };
            let light = LightGrid::filled(&level.level, light_now);
            let empty = Sight::new(&level.level);
            let made = build_posed(
                &level.level,
                (&[view], now, &draw, &empty),
                &light,
                &level.stairs,
                (&[], &BTreeSet::new(), &[], &level.walls, info.ruined),
                (local, shape, false),
                None,
                &BTreeMap::new(),
            );
            commands.entity(floor.root).insert(
                Transform::from_translation(Vec3::Y * -lowered)
                    .with_scale(Vec3::new(1.0, scale, 1.0)),
            );
            put(&mut commands, &mut meshes, floor, made);
        }
    }
    below.cost_ms = start.elapsed().as_secs_f32() * 1000.0;
    add_cost(&mut stats, below.cost_ms);
}

/// Puts a floor's built meshes in place, showing those with faces.
fn put(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    floor: &BelowFloor,
    made: DungeonMeshes,
) {
    let solid = made
        .solid
        .into_iter()
        .zip(&made.has_solid)
        .zip(&floor.solid);
    let memory = made
        .memory
        .into_iter()
        .zip(&made.has_memory)
        .zip(&floor.memory);
    for ((mesh, &has), (entity, handle, _)) in solid.chain(memory) {
        commands.entity(*entity).insert(if has {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
        // An empty mesh is left as it was, hidden: Bevy warns about meshes
        // with no vertices.
        if has && let Some(mut stored) = meshes.get_mut(handle) {
            *stored = mesh;
        }
    }
}

/// Hides all of a floor's meshes.
fn hide(commands: &mut Commands, floor: &BelowFloor) {
    for (entity, _, _) in floor.solid.iter().chain(&floor.memory) {
        commands.entity(*entity).insert(Visibility::Hidden);
    }
}

/// Says what the floors below cost in the view's stats, once.
fn add_cost(stats: &mut ResMut<ViewStats>, ms: f32) {
    if ms > 0.0 && !stats.summary.contains("floors below") {
        let summary = format!("{}, floors below {ms:.2} ms", stats.summary);
        stats.bypass_change_detection().summary = summary;
    }
}
