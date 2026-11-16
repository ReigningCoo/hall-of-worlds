//! Building the other floors again, only when what is known or seen of them,
//! the light, the looks or the settings change, or the camera has moved a
//! cell's width (walls' faces are chosen by where it is).

use bevy::platform::time::Instant;
use bevy::prelude::*;
use shared::dungeon::{Level, Sight};

use super::super::daylight::OutsideLight;
use super::super::theme::Theme;
use super::super::{CELL_SIZE, DungeonScene, Shown, SurfaceMaterials};
use super::floors::{Floor, Floors, make};
use super::one::{Building, Placed, build_floor};
use super::which::{floors_above, floors_below, seen_now};
use crate::common::{
    CurrentDungeon, Lightning, MapPosition, OtherFloorLooks, OtherLevels, Preferences, ViewStats,
};
use crate::player::Player;
use crate::view3d::ViewCamera;

/// What the other floors were last built for.
#[derive(Default)]
pub(in crate::dungeon_map) struct Built {
    looks: u64,
    camera: Option<Vec3>,
}

/// Builds the floors below and above the player's in a building, from what
/// the client has of them, and hides those not needed.
#[allow(clippy::too_many_arguments)]
pub(in crate::dungeon_map) fn draw_others(
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
    (mut floors, surfaces): (ResMut<Floors>, Res<SurfaceMaterials>),
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
            add_cost(&mut stats, floors.cost_ms);
        }
        return;
    }
    built.looks = shown.looks;
    built.camera = Some(camera);
    let start = Instant::now();
    let (below, above) = match &dungeon.info {
        Some(info) if shown.sight.is_some() => (floors_below(info), floors_above(info)),
        _ => (Vec::new(), Vec::new()),
    };
    let floors = &mut *floors;
    let like = &surfaces.solid;
    for (made, count) in [
        (&mut floors.below, below.len()),
        (&mut floors.above, above.len()),
    ] {
        make(
            made,
            count,
            &mut commands,
            *scene,
            &mut meshes,
            &mut materials,
            like,
        );
        for (at, floor) in made.iter().enumerate() {
            let used = at < count;
            let shown = if used {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            commands.entity(floor.root).insert(shown);
        }
    }
    if let (Some(info), Some(sight)) = (&dungeon.info, &shown.sight)
        && !(below.is_empty() && above.is_empty())
    {
        let storeys = (info.levels - u32::from(info.basement)) as usize;
        let known: Vec<Option<&Level>> = (0..storeys as u32)
            .map(|index| levels.level(index).map(|level| &level.level))
            .collect();
        let cells: Vec<Option<&Sight>> = (0..storeys as u32)
            .map(|index| match levels.level(index) {
                _ if index == info.index => Some(&dungeon.remembered),
                level => level.map(|level| &level.remembered),
            })
            .collect();
        let at = (player.x, player.y);
        let now = seen_now(&known, &dungeon.level, info.index as usize, at, sight);
        let mut light = outside.in_flash(lightning.0);
        light = light.max(Vec3::splat(preferences.graphics.least_light()));
        let building = Building {
            info,
            known: &known,
            cells: &cells,
            own: &dungeon.level,
            now: &now,
            at,
            camera,
            light,
            shape: Theme::of_level(Some(info.kind)).shape,
            looks: &looks,
        };
        let placed = below
            .iter()
            .map(|&(index, down)| (index, Placed::Below(down)))
            .zip(&floors.below)
            .chain(
                above
                    .iter()
                    .map(|&(index, up)| (index, Placed::Above(up)))
                    .zip(&floors.above),
            );
        for ((index, place), floor) in placed {
            match levels.level(index) {
                Some(level) => build_floor(
                    &mut commands,
                    &mut meshes,
                    floor,
                    &building,
                    level,
                    (index, place),
                ),
                None => hide(&mut commands, floor),
            }
        }
    }
    floors.cost_ms = start.elapsed().as_secs_f32() * 1000.0;
    add_cost(&mut stats, floors.cost_ms);
}

/// Hides all of a floor's meshes.
fn hide(commands: &mut Commands, floor: &Floor) {
    for (entity, _, _) in floor.solid.iter().chain(&floor.memory) {
        commands.entity(*entity).insert(Visibility::Hidden);
    }
}

/// Says what the other floors cost in the view's stats, once.
fn add_cost(stats: &mut ResMut<ViewStats>, ms: f32) {
    if ms > 0.0 && !stats.summary.contains("other floors") {
        let summary = format!("{}, other floors {ms:.2} ms", stats.summary);
        stats.bypass_change_detection().summary = summary;
    }
}
