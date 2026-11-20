//! Fungus patches, for now as placeholder markers until their models come
//! (fungi plan PR 4): a small flattened ball in each patch's main colour on
//! its cell, as big as it has grown by the world's clock
//! (`FungusPatch::stage`), standing on the ground. Only patches in sight are
//! drawn, never from memory. Picking them goes as herbs' do (`herbs::pick`).

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::fungus::Fungi;
use shared::herb::STAGES;

use super::town::cell_centre;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{CurrentDungeon, DrawnGround, ServerClock};

/// A full-grown marker's radius, as a share of a cell.
const RADIUS: f32 = 0.18;

/// What a drawn patch shows: its fungus, stage and the ground under it (in
/// tenths of a unit), to tell when it must be drawn again.
type Drawn = (String, u8, i32);

/// The markers drawn now, the shared ball and the materials by colour.
#[derive(Resource, Default)]
pub(super) struct DrawnFungi {
    cells: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen and the hour last time.
    built: Option<(u64, u64)>,
    ball: Option<Handle<Mesh>>,
    colours: HashMap<(u8, u8, u8), Handle<StandardMaterial>>,
}

/// Draws the markers in sight again when what is seen, the hour, the patches
/// or the ground change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_fungi(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, clock): (Res<CurrentDungeon>, Res<Shown>, Res<ServerClock>),
    heights: Res<DrawnGround>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut drawn: ResMut<DrawnFungi>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60);
    if drawn.built == Some(key) && !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    drawn.built = Some(key);
    let fungi = Fungi::built_in();
    let wanted: Vec<((i64, i64), Drawn)> = shown.sight.as_ref().map_or_else(Vec::new, |sight| {
        (dungeon.level.fungus_patches())
            .filter(|&((x, y), _)| sight.is_seen(x, y))
            .filter_map(|((x, y), patch)| {
                let stage = patch.stage(fungi.get(&patch.fungus)?, now);
                let ground = heights.drawn_between(x as f32, y as f32) * CELL_SIZE;
                let ground = (ground * 10.0).round() as i32;
                (stage > 0).then(|| ((x, y), (patch.fungus.clone(), stage, ground)))
            })
            .collect()
    });
    let ball = (drawn
        .ball
        .get_or_insert_with(|| meshes.add(Sphere::new(1.0))))
    .clone();
    let mut keep = HashMap::with_capacity(wanted.len());
    for (at, now_drawn) in wanted {
        if let Some((entity, was)) = drawn.cells.remove(&at) {
            if was == now_drawn {
                keep.insert(at, (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let Some(def) = fungi.get(&now_drawn.0) else {
            continue;
        };
        let colour = def
            .look
            .colours
            .first()
            .map_or((200, 200, 200), |c| (c.0, c.1, c.2));
        let material = (drawn.colours.entry(colour))
            .or_insert_with(|| materials.add(Color::srgb_u8(colour.0, colour.1, colour.2)))
            .clone();
        let size = RADIUS * CELL_SIZE * f32::from(now_drawn.1) / f32::from(STAGES);
        let middle = cell_centre(at.0, at.1).with_y(now_drawn.2 as f32 / 10.0);
        let transform =
            Transform::from_translation(middle).with_scale(Vec3::new(size, size * 0.6, size));
        let marker = commands
            .spawn((
                transform,
                Mesh3d(ball.clone()),
                MeshMaterial3d(material),
                NotShadowCaster,
            ))
            .id();
        commands.entity(*scene).add_child(marker);
        keep.insert(at, (marker, now_drawn));
    }
    for (_, (entity, _)) in drawn.cells.drain() {
        commands.entity(entity).despawn();
    }
    drawn.cells = keep;
}
