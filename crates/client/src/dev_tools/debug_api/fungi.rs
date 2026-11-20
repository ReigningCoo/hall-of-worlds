//! Fungus patches: listing the level's, and the showcase of every one.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, json, params};
use shared::fungus::{Form, Fungi};
use shared::herb::STAGES;

use super::done;
use crate::common::{CurrentDungeon, FungusShowcase, ServerClock};

#[derive(Deserialize)]
pub(super) struct ShowcaseParams {
    /// 1 young to 4 full grown; none (or 0) to draw the patches as they are.
    stage: Option<u8>,
    /// Only this form's fungi (`"Bracket"`), or all.
    form: Option<Form>,
}

/// `client.fungus_showcase { "stage": 4, "form": "Bracket" }`: a patch of
/// every fungus at that stage in rows south of the player, eight a row, two
/// cells apart, in the table's order.
pub(super) fn fungus_showcase(
    In(request): In<Option<Value>>,
    mut showcase: ResMut<FungusShowcase>,
) -> BrpResult {
    let ShowcaseParams { stage, form } = params(request)?;
    showcase.0 = stage
        .filter(|stage| (1..=STAGES).contains(stage))
        .map(|stage| (stage, form));
    done()
}

/// `client.fungus_patches`: the level's fungus patches the client knows of:
/// `x`, `y`, `fungus` (its key), `stage` (0 picked, 1 to 4 full grown) and
/// `ready` (can be picked).
pub(super) fn fungus_patches(
    In(_): In<Option<Value>>,
    (dungeon, clock): (Res<CurrentDungeon>, Res<ServerClock>),
) -> BrpResult {
    let now = clock.minutes.max(0.0) as u64;
    let patches: Vec<Value> = (dungeon.level.fungus_patches())
        .filter_map(|((x, y), patch)| {
            let def = Fungi::built_in().get(&patch.fungus)?;
            Some(json!({
                "x": x,
                "y": y,
                "fungus": patch.fungus,
                "stage": patch.stage(def, now),
                "ready": patch.can_pick(def, now),
            }))
        })
        .collect();
    Ok(json!({ "patches": patches }))
}
