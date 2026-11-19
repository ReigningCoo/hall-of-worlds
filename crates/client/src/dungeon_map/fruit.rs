//! Fruit plants: each a model of its family (`plants::FruitModels`) at the
//! stage it has grown to by the world's clock (`FruitPlant::stage`), in the
//! wilderness and farms' orchards, standing on the ground, in a variant and
//! turned by its cell (vines' trellises keep along their rows). Picking them
//! by click or tap is `herbs::pick`'s, as for herbs; `fruit_picking` finds
//! the plant a click lands on.
//!
//! Only plants in sight are drawn, never from memory: they will have grown
//! since. Each plant is an entity under the scene, with its outline a child.

use std::collections::HashMap;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use shared::dungeon::Level;
use shared::fruit::{FruitStage, Fruits, PlantFamily};
use shared::rng::Seed;

use super::herbs::HerbMaterials;
use super::town::cell_centre;
use super::{CELL_SIZE, DungeonScene, Shown};
use crate::common::{CurrentDungeon, DrawnGround, FruitShowcase, MapPosition, ServerClock};
use crate::plants::FruitModels;
use crate::player::Player;

/// What a drawn plant shows, to tell when it must be drawn again.
#[derive(Clone, Debug, PartialEq)]
struct Drawn {
    fruit: String,
    stage: FruitStage,
    /// The ground under its middle, in tenths of a unit.
    ground: i32,
}

/// What the showcase shows: a stage, and one family or all.
type Showcase = Option<(FruitStage, Option<PlantFamily>)>;

/// The plants drawn now, by cell, and what they were drawn for.
#[derive(Resource, Default)]
pub(super) struct DrawnFruit {
    plants: HashMap<(i64, i64), (Entity, Drawn)>,
    /// What was seen, the hour and the showcase last time.
    built: Option<(u64, u64, Showcase)>,
}

/// Draws the plants in sight again when what is seen, the hour, the plants,
/// the ground or the showcase change.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_fruit(
    mut commands: Commands,
    scene: Single<Entity, With<DungeonScene>>,
    (dungeon, shown, clock): (Res<CurrentDungeon>, Res<Shown>, Res<ServerClock>),
    (heights, showcase): (Res<DrawnGround>, Res<FruitShowcase>),
    player: Single<&MapPosition, With<Player>>,
    materials: Res<HerbMaterials>,
    mut models: ResMut<FruitModels>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawn: ResMut<DrawnFruit>,
) {
    let now = clock.minutes.max(0.0) as u64;
    let key = (shown.looks, now / 60, showcase.0);
    if drawn.built == Some(key) && !dungeon.is_changed() && !heights.is_changed() {
        return;
    }
    drawn.built = Some(key);
    let fruits = Fruits::built_in();
    let wanted: Vec<((i64, i64), &str, FruitStage)> = match showcase.0 {
        Some(asked) => super::fruit_showcase::cells((player.x, player.y), asked),
        None => shown.sight.as_ref().map_or_else(Vec::new, |sight| {
            (dungeon.level.fruit_plants())
                .filter(|&((x, y), _)| sight.is_seen(x, y))
                // Felled and growing back: `trees`' models.
                .filter(|&((x, y), _)| dungeon.level.felled_at(x, y).is_none())
                .filter_map(|(at, plant)| {
                    let def = fruits.get(&plant.fruit)?;
                    Some((at, def.key.as_str(), plant.stage(def, now)))
                })
                .collect()
        }),
    };
    let mut keep = HashMap::with_capacity(wanted.len());
    for ((x, y), fruit, stage) in wanted {
        let middle = cell_centre(x, y);
        let ground = heights.drawn_between(middle.x / CELL_SIZE, middle.z / CELL_SIZE) * CELL_SIZE;
        let now_drawn = Drawn {
            fruit: fruit.to_owned(),
            stage,
            ground: (ground * 10.0).round() as i32,
        };
        if let Some((entity, was)) = drawn.plants.remove(&(x, y)) {
            if was == now_drawn {
                keep.insert((x, y), (entity, was));
                continue;
            }
            commands.entity(entity).despawn();
        }
        let Some(def) = fruits.get(fruit) else {
            continue;
        };
        let (variant, turn) = looks(x, y);
        let turn = if def.plant == PlantFamily::Vine {
            0.0
        } else {
            turn
        };
        let model = models.get(def, stage, variant, &mut meshes);
        let plant = commands
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
        commands.entity(*scene).add_child(plant);
        keep.insert((x, y), (plant, now_drawn));
    }
    for (_, (entity, _)) in drawn.plants.drain() {
        commands.entity(entity).despawn();
    }
    drawn.plants = keep;
}

/// The variant a cell's plant is drawn in, and how far it is turned.
pub(super) fn looks(x: i64, y: i64) -> (u8, f32) {
    let hash = Seed::new(0x6672_7569_7463).at(&[x, y]);
    let turn = (hash >> 8) as u16 as f32 / f32::from(u16::MAX) * std::f32::consts::TAU;
    (hash as u8, turn)
}

/// What a fruit plant is called on hover: its fruit's name and how it is
/// growing ("Apple, in flower"; "Apple, ripe: ready to pick"), or growing
/// back if felled ("Apple, stump").
pub(super) fn fruit_name(level: &Level, now: u64, (x, y): (i64, i64)) -> Option<String> {
    let plant = level.fruit_at(x, y)?;
    let def = Fruits::built_in().get(&plant.fruit)?;
    if let Some(stage) = level.felled_at(x, y) {
        let how = super::tree_picking::stage_name(stage);
        return Some(format!("{}, {how}", def.name));
    }
    let how = match plant.stage(def, now) {
        FruitStage::Young => "young",
        FruitStage::Leafy => "in leaf",
        FruitStage::Flowering => "in flower",
        FruitStage::Fruiting { ripe: false } => "unripe",
        FruitStage::Fruiting { ripe: true } => "ripe: ready to pick",
    };
    Some(format!("{}, {how}", def.name))
}
