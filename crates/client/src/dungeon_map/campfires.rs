//! Campfires (`shared::campfire`, cooking plan PR 7): each one burning on the
//! level (`Level::campfires`, sent with the cells) drawn as a ring of stones
//! round crossed sticks, with flames as tall as the kindling it has left
//! (`model.rs`), black outlines, and flickering (still with Reduce Motion). Each
//! carries a warm `LightSource`, so it lights the ground and tokens round it
//! as a carried light does (`update_meshes`, `lights.rs`). Embers (nothing
//! left, about to go out) glow faintly, without flames.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::campfire::LIGHT_CELLS;

use super::town::cell_centre;
use super::{CELL_SIZE, DungeonScene};
use crate::common::{
    CurrentDungeon, DrawnGround, LightSource, MapPosition, Preferences, SmoothPosition,
};

mod model;

pub(super) use model::{FireParts, spawn_fire};

/// A fire's light: firelight's warm orange.
pub(super) const FIRE_LIGHT: Color = Color::srgb(1.0, 0.62, 0.28);

/// How far embers' light reaches, in cells.
const EMBER_CELLS: u32 = 1;

/// What a drawn fire shows, to tell when it must be drawn again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Drawn {
    pieces: u8,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// The fires drawn now, by cell, and the meshes and materials they share.
#[derive(Resource, Default)]
pub(super) struct DrawnFires {
    fires: HashMap<(i64, i64), (Entity, Drawn)>,
    parts: Option<FireParts>,
}

/// A flame, flickering: how tall it stands, where its foot is, and where in
/// its flicker it starts.
#[derive(Component)]
pub(super) struct Flame {
    pub(super) height: f32,
    pub(super) base: f32,
    pub(super) phase: f32,
}

/// Draws the level's fires again when its cells or the ground change.
pub(super) fn update_campfires(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, heights): (Res<CurrentDungeon>, Res<DrawnGround>),
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut drawn: ResMut<DrawnFires>,
) {
    if !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    let parts = (drawn.parts)
        .get_or_insert_with(|| FireParts::new(&mut meshes, &mut materials))
        .clone();
    let mut keep = HashMap::with_capacity(drawn.fires.len());
    for ((x, y), pieces) in dungeon.level.campfires() {
        let middle = cell_centre(x, y);
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let now = Drawn {
            pieces,
            ground: (ground * 10.0).round() as i32,
        };
        if let Some((entity, was)) = drawn.fires.remove(&(x, y)) {
            if was == now {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let position = MapPosition { x, y };
        let light = LightSource {
            radius: if pieces == 0 {
                EMBER_CELLS
            } else {
                LIGHT_CELLS
            },
            colour: FIRE_LIGHT,
            darkvision: 0,
        };
        let fire = commands
            .spawn((
                Transform::from_translation(middle.with_y(ground)),
                Visibility::default(),
                position,
                SmoothPosition::from(position),
                light,
            ))
            .id();
        spawn_fire(&mut commands, fire, &parts, ((x, y), pieces), false);
        commands.entity(*scene).add_child(fire);
        keep.insert((x, y), (fire, now));
    }
    for (_, (entity, _)) in drawn.fires.drain() {
        commands.entity(entity).despawn();
    }
    drawn.fires = keep;
}

/// Each frame: the flames rise and fall a little, out of step with each
/// other; still with Reduce Motion.
pub(super) fn flicker_flames(
    time: Res<Time>,
    preferences: Res<Preferences>,
    mut flames: Query<(&Flame, &mut Transform)>,
) {
    let seconds = time.elapsed_secs();
    for (flame, mut transform) in &mut flames {
        let flicker = match preferences.graphics.reduce_motion {
            true => 1.0,
            false => 1.0 + 0.18 * (seconds * 9.0 + flame.phase).sin() * (seconds * 3.7).cos(),
        };
        let height = flame.height * flicker;
        transform.scale = Vec3::new(1.0, height, 1.0);
        // Cones stand on their middle: kept on the sticks as they grow.
        transform.translation.y = flame.base + height / 2.0;
    }
}
