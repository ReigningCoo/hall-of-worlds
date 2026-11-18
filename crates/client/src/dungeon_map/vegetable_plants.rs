//! Vegetable plants: each cell's a model of its plants (`plants::
//! VegetableModels`) at the stage they have grown to by the world's clock
//! (`VegetablePlant::stage`), moved on by the debug menu's shift, in farms'
//! fields and the wilderness, standing on the ground in a variant by the
//! cell. A field's cells turn only by quarter turns, so its rows line up;
//! wild ones any way. Nothing stands on a cell dug up.
//!
//! Only plants in sight are drawn, never from memory: they will have grown
//! since. Each cell is an entity under the scene, with its outline a child;
//! cells showing the same model share its meshes, so a field draws cheaply.

use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::rng::Seed;
use shared::vegetable::Vegetables;

use super::field::CROP_STAGES;
use super::herbs::HerbMaterials;
use super::town::cell_centre;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{
    CropGrowthShift, CurrentDungeon, DrawnGround, MapPosition, ServerClock, VegetableShowcase,
};
use crate::plants::VegetableModels;
use crate::player::Player;

/// What a drawn cell shows, to tell when it must be drawn again.
#[derive(Clone, Debug, PartialEq)]
struct Drawn {
    vegetable: String,
    stage: u8,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// The cells drawn now, and what they were drawn for.
#[derive(Resource, Default)]
pub(super) struct DrawnVegetables {
    cells: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen, the hour, the shift and the showcase last time.
    built: Option<(u64, u64, u32, VegetableShowcase)>,
}

/// Draws the cells in sight again when what is seen, the hour, the plants,
/// the ground, the shift or the showcase change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_vegetables(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, clock): (Res<CurrentDungeon>, Res<Shown>, Res<ServerClock>),
    (heights, shift, showcase): (
        Res<DrawnGround>,
        Res<CropGrowthShift>,
        Res<VegetableShowcase>,
    ),
    player: Single<&MapPosition, With<Player>>,
    materials: Res<HerbMaterials>,
    mut models: ResMut<VegetableModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnVegetables>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60, shift.0, *showcase);
    if drawn.built == Some(key) && !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    drawn.built = Some(key);
    let vegetables = Vegetables::built_in();
    let wanted: Vec<((i64, i64), &str, u8)> = match showcase.0 {
        Some(asked) => super::vegetable_showcase::cells((player.x, player.y), asked),
        None => shown.sight.as_ref().map_or_else(Vec::new, |sight| {
            (dungeon.level.vegetable_plants())
                .filter(|&((x, y), _)| sight.is_seen(x, y))
                .filter_map(|(at, plant)| {
                    let def = vegetables.get(&plant.vegetable)?;
                    let stage = (u32::from(plant.stage(def, now)) + shift.0) % CROP_STAGES as u32;
                    Some((at, def.key.as_str(), stage as u8))
                })
                .collect()
        }),
    };
    let mut keep = HashMap::with_capacity(wanted.len());
    for ((x, y), vegetable, stage) in wanted {
        let middle = cell_centre(x, y);
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let now_drawn = Drawn {
            vegetable: vegetable.to_owned(),
            stage,
            ground: (ground * 10.0).round() as i32,
        };
        if let Some((entity, was)) = drawn.cells.remove(&(x, y)) {
            if was == now_drawn {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let Some(def) = vegetables.get(vegetable) else {
            continue;
        };
        let (variant, turn) = looks(x, y);
        let in_field = dungeon.fields.iter().any(|field| field.rect.contains(x, y));
        let turn = if in_field {
            (turn / FRAC_PI_2).floor() * FRAC_PI_2
        } else {
            turn
        };
        let Some(model) = models.get(def, stage, variant, &mut meshes) else {
            continue;
        };
        let cell = commands
            .spawn((
                Transform::from_translation(middle.with_y(ground))
                    .with_rotation(Quat::from_rotation_y(turn)),
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
        commands.entity(*scene).add_child(cell);
        keep.insert((x, y), (cell, now_drawn));
    }
    for (_, (entity, _)) in drawn.cells.drain() {
        commands.entity(entity).despawn();
    }
    drawn.cells = keep;
}

/// The variant a cell is drawn in, and how far it is turned.
fn looks(x: i64, y: i64) -> (u8, f32) {
    let hash = Seed::new(0x7665_6765_7463).at(&[x, y]);
    let turn = (hash >> 8) as u16 as f32 / f32::from(u16::MAX) * TAU;
    (hash as u8, turn)
}
