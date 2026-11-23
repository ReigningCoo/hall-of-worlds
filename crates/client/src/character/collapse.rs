//! Being held still: collapsing drunk (`shared::alcohol::Stage::Collapsed`)
//! or paralysed by a spell's Earth (`shared::spell::PARALYSED`). The server
//! spends the energy it lasts, so the player can't act until it is over; this
//! spends the same here, so nothing is tried meanwhile.

use bevy::prelude::*;
use shared::alcohol::Stage;
use shared::bonus::{Effect, Source};
use shared::spell::PARALYSED;

use super::CharacterSheet;
use crate::common::{PlayerSpeed, SpentEnergy};

/// Whether an effect holds its bearer still.
fn holds(effect: &Effect) -> bool {
    matches!(
        effect.source,
        Source::Alcohol(Stage::Collapsed) | Source::Spell(PARALYSED)
    )
}

/// When the sheet first shows an effect holding the player still (by its
/// source and start), spends their speed for every tick it lasts.
pub(super) fn spend_collapse(
    sheet: Res<CharacterSheet>,
    speed: Res<PlayerSpeed>,
    mut seen: Local<Vec<(Source, u64)>>,
    mut spent: MessageWriter<SpentEnergy>,
) {
    if !sheet.is_changed() {
        return;
    }
    let holding: Vec<&Effect> = sheet.0.effects.0.iter().filter(|e| holds(e)).collect();
    for effect in &holding {
        if seen.contains(&(effect.source, effect.start)) {
            continue;
        }
        let ticks = effect.end.map_or(0, |end| end.saturating_sub(effect.start));
        let cost = u64::from(speed.0) * ticks;
        spent.write(SpentEnergy(u32::try_from(cost).unwrap_or(u32::MAX)));
    }
    *seen = holding.iter().map(|e| (e.source, e.start)).collect();
}
