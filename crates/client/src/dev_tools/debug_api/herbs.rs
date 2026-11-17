//! Herb patches: listing the level's, and the showcase of every herb.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, json, params};

use super::done;
use crate::common::{CurrentDungeon, HerbShowcase, HerbTable, ServerClock};

#[derive(Deserialize)]
pub(super) struct ShowcaseParams {
    /// 1 to 4, or none (or 0) to draw the patches as they are.
    stage: Option<u8>,
}

/// `client.herb_showcase { "stage": 4 }`: every herb at that stage in rows
/// round the player, ten a row, two cells apart, in the table's order.
pub(super) fn herb_showcase(
    In(request): In<Option<Value>>,
    mut showcase: ResMut<HerbShowcase>,
) -> BrpResult {
    let ShowcaseParams { stage } = params(request)?;
    showcase.0 = stage.filter(|&stage| stage > 0);
    done()
}

/// `client.herbs`: the level's herb patches the client knows of: `x`, `y`,
/// `herb` (its key), `stage` (0 dug up, 1 to 4) and `ready` (can be picked).
pub(super) fn herbs(
    In(_): In<Option<Value>>,
    (dungeon, table, clock): (Res<CurrentDungeon>, Res<HerbTable>, Res<ServerClock>),
) -> BrpResult {
    let now = clock.minutes.max(0.0) as u64;
    let patches: Vec<Value> = dungeon
        .level
        .herbs()
        .filter_map(|((x, y), patch)| {
            let def = table.get(&patch.herb)?;
            Some(json!({
                "x": x,
                "y": y,
                "herb": patch.herb,
                "stage": patch.stage(def, now),
                "ready": patch.can_pick(def, now),
            }))
        })
        .collect();
    Ok(json!({ "patches": patches }))
}
