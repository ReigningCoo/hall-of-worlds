//! Vegetable plants: listing the level's.

use bevy::prelude::*;
use server::debug_api::{BrpResult, Value, json};
use shared::vegetable::Vegetables;

use crate::common::{CurrentDungeon, ServerClock};

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
