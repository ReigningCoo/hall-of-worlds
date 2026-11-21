//! Fires in lit hearths (cooking plan PR 8): each hearth the server said is
//! lit (`common::Benches`) gets a small fire in its firebox (`campfires`'
//! model without its ring of stones), flickering, with a warm
//! `LightSource` as a campfire's. Drawn once the hearth has been seen.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::campfire::LIGHT_CELLS;

use super::campfires::{FIRE_LIGHT, FireParts, spawn_fire};
use super::furniture::firebox;
use super::{CELL_SIZE, DungeonScene};
use crate::common::{
    Benches, CurrentDungeon, DrawnGround, LightSource, MapPosition, SmoothPosition,
};

/// Pieces a hearth's fire is drawn with: the server tells only lit or not.
const PIECES: u8 = 3;

/// The hearth fires drawn now, by the hearth's cell, with the ground under
/// each (in tenths of a unit), and the parts they share.
#[derive(Resource, Default)]
pub(super) struct DrawnHearthFires {
    fires: HashMap<(i64, i64), (Entity, i32)>,
    parts: Option<FireParts>,
}

/// Draws the hearths' fires again when the benches, the level or the ground
/// change.
pub(super) fn update_hearth_fires(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, heights, benches): (Res<CurrentDungeon>, Res<DrawnGround>, Res<Benches>),
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
    mut drawn: ResMut<DrawnHearthFires>,
) {
    if !dungeon.is_changed() && !heights.is_changed() && !benches.is_changed() {
        return;
    }
    let parts = (drawn.parts)
        .get_or_insert_with(|| FireParts::new(&mut meshes, &mut materials))
        .clone();
    let mut keep = HashMap::with_capacity(drawn.fires.len());
    for &at in &benches.1 {
        let Some(piece) = dungeon.level.furniture_at(at.0, at.1) else {
            continue;
        };
        let Some(middle) = firebox(piece).filter(|_| piece.at == at) else {
            continue;
        };
        if !dungeon.remembered.is_seen(at.0, at.1) {
            continue;
        }
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let tenths = (ground * 10.0).round() as i32;
        if let Some((entity, was)) = drawn.fires.remove(&at) {
            if was == tenths {
                keep.insert(at, (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let position = MapPosition { x: at.0, y: at.1 };
        let light = LightSource {
            radius: LIGHT_CELLS,
            colour: FIRE_LIGHT,
            darkvision: 0,
        };
        let fire = commands
            .spawn((
                Transform::from_translation(middle + Vec3::Y * ground),
                Visibility::default(),
                position,
                SmoothPosition::from(position),
                light,
            ))
            .id();
        spawn_fire(&mut commands, fire, &parts, (at, PIECES), true);
        commands.entity(*scene).add_child(fire);
        keep.insert(at, (fire, tenths));
    }
    for (_, (entity, _)) in drawn.fires.drain() {
        commands.entity(entity).despawn();
    }
    drawn.fires = keep;
}
