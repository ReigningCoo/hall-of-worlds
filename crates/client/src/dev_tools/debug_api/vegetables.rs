//! Vegetable plants: listing the level's, and the showcase of every one.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, json, params};
use shared::herb::STAGES;
use shared::vegetable::{PlantShape, Vegetables};

use super::done;
use crate::common::{CurrentDungeon, ServerClock, VegetableShowcase};

#[derive(Deserialize)]
pub(super) struct ShowcaseParams {
    /// 1 sprouting to 4 full grown; none (or 0) to draw the plants as they
    /// are.
    stage: Option<u8>,
    /// Only this plant shape's vegetables (`"GrainGrass"`), or all.
    shape: Option<PlantShape>,
}

/// `client.vegetable_showcase { "stage": 4, "shape": "Bush" }`: a cell of
/// every vegetable at that stage in rows south of the player, eight a row,
/// two cells apart, in the table's order.
pub(super) fn vegetable_showcase(
    In(request): In<Option<Value>>,
    mut showcase: ResMut<VegetableShowcase>,
) -> BrpResult {
    let ShowcaseParams { stage, shape } = params(request)?;
    showcase.0 = stage
        .filter(|stage| (1..=STAGES).contains(stage))
        .map(|stage| (stage, shape));
    done()
}

/// `client.vegetable_plants`: the level's vegetable plants the client knows
/// of: `x`, `y`, `vegetable` (its key), `stage` (0 dug up, 1 to 4 full grown)
/// and `ready` (can be picked).
pub(super) fn vegetable_plants(
    In(_): In<Option<Value>>,
    (dungeon, clock): (Res<CurrentDungeon>, Res<ServerClock>),
) -> BrpResult {
    let now = clock.minutes.max(0.0) as u64;
    let plants: Vec<Value> = (dungeon.level.vegetable_plants())
        .filter_map(|((x, y), plant)| {
            let def = Vegetables::built_in().get(&plant.vegetable)?;
            Some(json!({
                "x": x,
                "y": y,
                "vegetable": plant.vegetable,
                "stage": plant.stage(def, now),
                "ready": plant.can_pick(def, now),
            }))
        })
        .collect();
    Ok(json!({ "plants": plants }))
}
