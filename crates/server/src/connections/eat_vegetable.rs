//! Eating a vegetable raw (through `consume.rs`): its healing and raw effects
//! (`vegetable::eating`). A whole big one or a piece is cut first
//! (`vegetable::cut`): one serving is eaten and the rest go back, as a cut
//! fruit's do (`eat_fruit::put_back`).

use bevy::prelude::*;
use shared::item::Item;
use shared::vegetable::{Vegetables, cut, eating};

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;

/// Eats `eaten`, one vegetable already taken from where it was (`carried` or on
/// the ground), at action tick `now`. A vegetable the table lacks does nothing.
pub(super) fn eat_vegetable(
    client: &mut Client,
    world: &mut ServerWorld,
    (eaten, carried): (&Item, bool),
    now: u64,
) {
    let Some(form) = eaten.vegetable() else {
        return;
    };
    let Some(def) = Vegetables::built_in().get(&form.vegetable) else {
        info!(
            "Ate a vegetable this server doesn't have: {}",
            form.vegetable
        );
        return;
    };
    if let Some((_, Some(rest))) = cut(eaten, &mut new_id) {
        super::eat_fruit::put_back(client, world, rest, carried);
    }
    let (sheet, situation) = client.sheet_and_situation();
    sheet.change_effects(situation, now, |on| {
        for effect in eating(def, now) {
            on.add(effect);
        }
    });
    client.unsaved = true;
    client.refresh_attributes(now);
}
