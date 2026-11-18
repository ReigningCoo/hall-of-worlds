//! What dev tools keep as they are while set: stairs' style, how far crops
//! are drawn on.

use bevy::prelude::*;
use shared::site::StairsStyle;
use std::env;

use crate::common::{CropGrowthShift, CurrentDungeon};
use crate::dungeon_map::CROP_STAGES;

/// What dev tools keep as they are while set, however the game changes them:
/// every stairs' style, and crops drawn this many stages on with pasture as
/// this variety (`HOW_DEBUG_STAIRS`, `HOW_DEBUG_FIELDS`). The debug API sets them too.
#[derive(Resource, Default)]
pub(super) struct Forced {
    pub(super) stairs: Option<StairsStyle>,
    pub(super) fields: Option<u32>,
}

pub(super) fn apply_forced(
    forced: Res<Forced>,
    mut dungeon: ResMut<CurrentDungeon>,
    mut crops: ResMut<CropGrowthShift>,
) {
    if let Some(style) = forced.stairs
        && dungeon.stairs.iter().any(|stairs| stairs.style != style)
    {
        for stairs in &mut dungeon.stairs {
            stairs.style = style;
        }
    }
    let Some(stages) = forced.fields else {
        return;
    };
    let shift = stages % CROP_STAGES as u32;
    if crops.0 != shift {
        crops.0 = shift;
    }
    if (dungeon.fields.iter()).any(|field| field.livestock && field.variety != stages) {
        for field in dungeon.fields.iter_mut().filter(|field| field.livestock) {
            field.variety = stages;
        }
    }
}

/// `HOW_DEBUG_STAIRS` and `HOW_DEBUG_FIELDS`.
pub(super) fn read(app: &mut App) {
    let mut forced = app.world_mut().resource_mut::<Forced>();
    forced.stairs = env::var("HOW_DEBUG_STAIRS")
        .ok()
        .and_then(|n| n.trim().parse::<usize>().ok())
        .and_then(|n| StairsStyle::ALL.get(n).copied());
    forced.fields = env::var("HOW_DEBUG_FIELDS")
        .ok()
        .and_then(|n| n.trim().parse::<u32>().ok());
}
