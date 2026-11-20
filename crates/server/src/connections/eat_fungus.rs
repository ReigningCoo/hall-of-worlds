//! Eating a fungus raw (through `consume.rs`): its healing, raw effects and
//! poison (`fungus::eating`). A big piece is cut first (`fungus::cut`): one
//! serving is eaten and the rest go back, as a cut fruit's do
//! (`eat_fruit::put_back`).

use bevy::prelude::*;
use shared::fungus::{Fungi, cut, eating};
use shared::item::Item;

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;

/// Eats `eaten`, one fungus already taken from where it was (`carried` or on
/// the ground), at action tick `now`. A fungus the table lacks does nothing.
pub(super) fn eat_fungus(
    client: &mut Client,
    world: &mut ServerWorld,
    (eaten, carried): (&Item, bool),
    now: u64,
) {
    let Some(form) = eaten.fungus() else {
        return;
    };
    let Some(def) = Fungi::built_in().get(&form.fungus) else {
        info!("Ate a fungus this server doesn't have: {}", form.fungus);
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
