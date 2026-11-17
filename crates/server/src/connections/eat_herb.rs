//! Eating a herb raw (through `consume.rs`): teaches its first effect and gives a
//! weak, short dose of it (`herb::raw_dose`), stronger as the herb's quality and
//! the eater's Herbalism allow, and a herb bearing fruit that fruit's healing
//! and effects too (`herb::eating_herb`). A herb the server doesn't have does nothing.

use bevy::prelude::*;
use shared::herb::{HERBALISM, eating_herb, skill_value};
use shared::item::{Item, Quality};

use super::Client;
use crate::ServerWorld;

/// Eats `eaten`, one herb already taken from where it was, at action tick `now`.
pub(super) fn eat_herb(client: &mut Client, world: &ServerWorld, eaten: &Item, now: u64) {
    let Some(form) = eaten.herb() else {
        return;
    };
    let Some(def) = world.herbs.get(&form.herb) else {
        info!("Ate a herb this server doesn't have: {}", form.herb);
        return;
    };
    let quality = eaten.quality.unwrap_or(Quality::Common);
    let herbalism = skill_value(&client.sheet.skills, HERBALISM);
    client.sheet.herbs.learn_effect(&def.key, 0);
    let effects = eating_herb(def, quality, herbalism, now);
    let (sheet, situation) = client.sheet_and_situation();
    sheet.change_effects(situation, now, |on| {
        for effect in effects {
            on.add(effect);
        }
    });
    client.unsaved = true;
    client.refresh_attributes(now);
}
