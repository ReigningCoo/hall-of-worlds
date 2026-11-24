//! The player's own light (owner): a lantern burning, held or tied to the
//! belt, lights `shared::lantern::LANTERN_LIGHT` cells round them in its warm
//! colour, else the carried light (`Inventory::light_reach`); a spell's
//! Light reaches as far as it says, and Darkness swallows both
//! (`Sheet::light_reach`).

use bevy::prelude::*;

use crate::character::CharacterSheet;
use crate::common::{LANTERN_COLOUR, LightSource};
use crate::inventory::Carried;
use crate::player::Player;

/// Keeps the player's `LightSource` as what they carry says.
pub(super) fn follow_lantern(
    carried: Res<Carried>,
    sheet: Res<CharacterSheet>,
    mut light: Single<&mut LightSource, With<Player>>,
) {
    if !carried.is_changed() && !sheet.is_changed() {
        return;
    }
    let radius = sheet.0.light_reach(&carried.0);
    let colour = match carried.0.lantern_lit() {
        true => LANTERN_COLOUR,
        false => Color::WHITE,
    };
    if light.radius != radius || light.colour != colour {
        light.radius = radius;
        light.colour = colour;
    }
}
