//! Drinking from a vessel (through `consume.rs`, drinks plan PR 2): a serving
//! of water (`vessel::SERVING_ML`) from a tankard, cup or any vessel, carried
//! or lying in reach, which does nothing yet; the vessel stays where it is.

use bevy::prelude::*;
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::Item;
use shared::item_form::ItemForm;

use super::Client;
use super::pour::{put_back, side};
use crate::ServerWorld;

/// Drinks from `item` if it is a vessel, carried or in reach: whether the
/// client is still there and what it cost. `None` if it isn't a vessel.
pub(super) fn drink_vessel(
    client: &mut Client,
    world: &mut ServerWorld,
    item: Id,
) -> Option<(bool, u32)> {
    let side = side(client, world, item).ok()?;
    let vessel = side.stack.top().vessel()?.clone();
    let (after, ml) = vessel.drunk();
    if ml == 0 {
        warn!(
            "{} refused drinking from {item:?}: not water",
            client.in_log()
        );
    } else {
        let changed = Item {
            form: Some(ItemForm::Vessel(after)),
            ..side.stack.top().clone()
        };
        put_back(client, world, &side, changed);
        client.unsaved = true;
    }
    client.ground_told.again = true;
    Some((client.send_inventory(), handling_cost(ml)))
}
