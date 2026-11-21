//! Eating a serving of a Dish (through `consume.rs`; cooking plan PR 6): Well
//! Fed (`food::eating`), replacing any before it, and the hidden poisons of
//! any fungi in it (`food::food_poisons`, `fungus::poisoning`), each hidden
//! unless recognised, as eating the fungus itself would be.

use shared::cooking::Ingredients;
use shared::food::{eating, food_poisons};
use shared::fungus::{Fungi, poisoning};
use shared::item::Item;

use super::Client;
use super::fungus_harm::{eater, recognises};

/// Eats `eaten`, one serving already taken from where it was, at action tick
/// `now`.
pub(super) fn eat_dish(client: &mut Client, eaten: &Item, now: u64) {
    let Some(form) = eaten.food().filter(|form| form.rest.is_none()) else {
        return;
    };
    let quality = eaten.quality.unwrap_or_default();
    let well_fed = eating(form, quality, now, Ingredients::built_in());
    let fungi = Fungi::built_in();
    let poisons: Vec<_> = (food_poisons(form, fungi).into_iter())
        .filter_map(|def| Some((def, def.poison.as_ref()?)))
        .map(|(def, poison)| (def, poisoning(def, poison, now, eater(client, now))))
        .collect();
    for (def, effects) in poisons {
        let known = recognises(client, def);
        let (sheet, situation) = client.sheet_and_situation();
        sheet.lay_on(effects, known, situation, now);
    }
    let (sheet, situation) = client.sheet_and_situation();
    sheet.change_effects(situation, now, |on| on.add(well_fed));
    client.unsaved = true;
    client.refresh_attributes(now);
}
