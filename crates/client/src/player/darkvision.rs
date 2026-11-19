//! Darkvision (Alchemy plan PR 8): while a Darkvision potion lasts, the
//! player's `LightSource` also sees in the dark as far as its effect says
//! (`shared::bonus::Totals::darkvision`), which `dungeon_map` draws.

use bevy::prelude::*;

use super::Player;
use crate::character::CharacterSheet;
use crate::common::LightSource;

/// Keeps the player's Darkvision as their effects say: the server drops
/// effects that have ended and sends the sheet again, so all kept count.
pub(super) fn follow_darkvision(
    sheet: Res<CharacterSheet>,
    mut light: Single<&mut LightSource, With<Player>>,
) {
    if !sheet.is_changed() {
        return;
    }
    let reach = sheet.0.totals(&[], 0).darkvision();
    if light.darkvision != reach {
        light.darkvision = reach;
    }
}
