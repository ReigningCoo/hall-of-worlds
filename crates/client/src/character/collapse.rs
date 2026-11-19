//! Collapsing drunk (`shared::alcohol::Stage::Collapsed`): the server spends
//! the energy the collapse lasts, so the player can't act until it is over;
//! this spends the same here, so nothing is tried meanwhile.

use bevy::prelude::*;
use shared::alcohol::Stage;
use shared::bonus::Source;

use super::CharacterSheet;
use crate::common::{PlayerSpeed, SpentEnergy};

/// When the sheet first says the player has collapsed, spends its speed for
/// every tick the collapse lasts.
pub(super) fn spend_collapse(
    sheet: Res<CharacterSheet>,
    speed: Res<PlayerSpeed>,
    mut was: Local<bool>,
    mut spent: MessageWriter<SpentEnergy>,
) {
    if !sheet.is_changed() {
        return;
    }
    let collapse = sheet
        .0
        .effects
        .0
        .iter()
        .find(|effect| effect.source == Source::Alcohol(Stage::Collapsed));
    let collapsed = collapse.is_some();
    if collapsed && !*was {
        let ticks = collapse
            .and_then(|effect| effect.end.map(|end| end.saturating_sub(effect.start)))
            .unwrap_or(0);
        let cost = u64::from(speed.0) * ticks;
        spent.write(SpentEnergy(u32::try_from(cost).unwrap_or(u32::MAX)));
    }
    *was = collapsed;
}
