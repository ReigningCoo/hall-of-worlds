//! Candlelight as fixed lights (tavern furniture plan PR 5): sconces,
//! chandeliers and candles on tables (`shared::decor::fixed_lights`), always
//! lit, warm, each from its cell as far as the rules say.

use bevy::prelude::*;
use shared::dungeon::Level;

use super::FixedLight;

/// Candlelight's colour and strength at its own cell: warm, a little less
/// than a fire's.
const CANDLELIGHT: Vec3 = Vec3::new(0.95, 0.7, 0.4);

/// The lights the level's candles give.
pub(super) fn candle_lights(level: &Level) -> Vec<FixedLight> {
    (shared::decor::fixed_lights(level).into_iter())
        .map(|light| FixedLight {
            cell: light.cell,
            radius: light.cells,
            colour: CANDLELIGHT,
        })
        .collect()
}
