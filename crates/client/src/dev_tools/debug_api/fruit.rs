//! Fruit plants: listing the level's.

use bevy::prelude::*;
use server::debug_api::{BrpResult, Value, json};
use shared::fruit::Fruits;

use crate::common::{CurrentDungeon, ServerClock};

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
