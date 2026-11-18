//! Fruit plants: listing the level's, and the showcase of every one.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, json, params};
use shared::fruit::{FruitStage, Fruits, PlantFamily};

use super::done;
use crate::common::{CurrentDungeon, FruitShowcase, ServerClock};

#[derive(Deserialize)]
pub(super) struct ShowcaseParams {
    /// 1 young, 2 leafy, 3 flowering, 4 unripe, 5 ripe; none (or 0) to draw
    /// the plants as they are.
    stage: Option<u8>,
    /// Only this family's plants (`"Palm"`), or all.
    family: Option<PlantFamily>,
}

/// `client.fruit_showcase { "stage": 5, "family": "Palm" }`: every fruit
/// plant at that stage in rows south of the player, eight a row, three cells
/// apart, in the table's order.
pub(super) fn fruit_showcase(
    In(request): In<Option<Value>>,
    mut showcase: ResMut<FruitShowcase>,
) -> BrpResult {
    let ShowcaseParams { stage, family } = params(request)?;
    let stage = match stage {
        Some(1) => FruitStage::Young,
        Some(2) => FruitStage::Leafy,
        Some(3) => FruitStage::Flowering,
        Some(4) => FruitStage::Fruiting { ripe: false },
        Some(5) => FruitStage::Fruiting { ripe: true },
        _ => {
            showcase.0 = None;
            return done();
        }
    };
    showcase.0 = Some((stage, family));
    done()
}

/// `client.fruit_plants`: the level's fruit plants the client knows of: `x`,
/// `y`, `fruit` (its key), `stage` (1 young to 4 fruiting), `ripe` (can be
/// picked) and `trunk`.
pub(super) fn fruit_plants(
    In(_): In<Option<Value>>,
    (dungeon, clock): (Res<CurrentDungeon>, Res<ServerClock>),
) -> BrpResult {
    let now = clock.minutes.max(0.0) as u64;
    let plants: Vec<Value> = (dungeon.level.fruit_plants())
        .filter_map(|((x, y), plant)| {
            let def = Fruits::built_in().get(&plant.fruit)?;
            Some(json!({
                "x": x,
                "y": y,
                "fruit": plant.fruit,
                "stage": plant.stage(def, now).number(),
                "ripe": plant.can_pick(def, now),
                "trunk": def.plant.has_trunk(),
            }))
        })
        .collect();
    Ok(json!({ "plants": plants }))
}
