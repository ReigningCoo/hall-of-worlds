//! Eating a fruit (through `consume.rs`): its healing and effects
//! (`fruit::eating`). A whole big fruit is cut first (`fruit::cut`): one
//! serving is eaten and the rest go back, given back to what is carried
//! (`Inventory::give`) or put on the ground when it lay there or there is no room.

use bevy::prelude::*;
use shared::fruit::{Fruits, cut, eating};
use shared::id::Id;
use shared::item::Item;
use shared::vegetable;

use super::Client;
use crate::ServerWorld;
use crate::ids::new_id;

/// Eats `eaten`, one fruit already taken from where it was (`carried` or on the
/// ground), at action tick `now`. A fruit the table lacks does nothing.
pub(super) fn eat_fruit(
    client: &mut Client,
    world: &mut ServerWorld,
    (eaten, carried): (&Item, bool),
    now: u64,
) {
    let Some(form) = eaten.fruit() else {
        return;
    };
    let Some(def) = Fruits::built_in().get(&form.fruit) else {
        info!("Ate a fruit this server doesn't have: {}", form.fruit);
        return;
    };
    if let Some((_, Some(rest))) = cut(eaten, &mut new_id) {
        put_back(client, world, rest, carried);
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

/// The servings left of a cut fruit or vegetable, back where it came from.
pub(super) fn put_back(client: &mut Client, world: &mut ServerWorld, rest: Item, carried: bool) {
    let rest = match carried {
        true => match client.inventory.give(rest, world.stacks, &mut new_id) {
            Ok(()) => return,
            Err(rest) => rest,
        },
        false => rest,
    };
    crate::ground::put_down(client, world, rest);
}

/// Whether eating the top one of carried `item` leaves its servings somewhere:
/// anything but a whole fruit or vegetable does, and one does if they fit in
/// what is carried or there is ground to put them on (in a site).
pub(super) fn servings_fit(client: &Client, world: &ServerWorld, item: Id, food: &Item) -> bool {
    let cut = cut(food, &mut || Id::NOBODY).or_else(|| vegetable::cut(food, &mut || Id::NOBODY));
    let Some((_, Some(rest))) = cut else {
        return true;
    };
    if client.visit.is_some() {
        return true;
    }
    let mut after = client.inventory.clone();
    after.take_one(item, &mut new_id);
    after.give(rest, world.stacks, &mut || Id::NOBODY).is_ok()
}
