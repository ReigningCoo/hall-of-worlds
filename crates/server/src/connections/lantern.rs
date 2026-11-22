//! The lantern on the server (lantern task PR 5; rules in `shared::lantern`):
//! filling one from a bottle of oil (`ClientMessage::FillLantern`), and its oil
//! burning by playtime while held or tied to the belt (`Client::burn_lanterns`,
//! from `playtime.rs`, so never while offline or idle).

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Inventory;
use shared::lantern::{FillRefused, fill};
use shared::pour::pour_cost;

use super::Client;
use super::pour::{put_back, side};
use crate::ServerWorld;

/// How often, in millilitres burnt, the player's client is sent the fount again
/// to keep its fuel bar right (it counts down by itself between): 10 ml, six
/// minutes of playtime. It is always told when a lantern goes out.
const TELL_EVERY_ML: u32 = 10;

/// Fills `lantern` from `bottle`: whether the client is still there, and what
/// it cost (as pouring the oil; nothing if refused). Answered with `Inventory`
/// either way.
pub(super) fn fill_lantern(
    client: &mut Client,
    world: &mut ServerWorld,
    lantern: Id,
    bottle: Id,
) -> (bool, u32) {
    let cost = match filled(client, world, lantern, bottle) {
        Ok(ml) => pour_cost(ml),
        Err(why) => {
            warn!(
                "{} refused filling {lantern:?} from {bottle:?}: {why}",
                client.in_log()
            );
            0
        }
    };
    client.unsaved = true;
    client.ground_told.again = true;
    (client.send_inventory(), cost)
}

/// Fills by the rule: how many millilitres went in. The lantern is carried
/// anywhere (held, tied or in a grid); the bottle is carried or lying in reach,
/// as for pouring, and the top one of its stack comes back empty.
fn filled(
    client: &mut Client,
    world: &mut ServerWorld,
    lantern: Id,
    bottle: Id,
) -> Result<u32, &'static str> {
    let binned = (client.inventory.bin.as_ref()).is_some_and(|item| item.id == lantern);
    let found = (client.inventory.find(lantern))
        .filter(|_| !binned)
        .ok_or("no such lantern is carried")?
        .clone();
    let source = side(client, world, bottle)?;
    let done = fill(&found, source.stack.top()).map_err(words)?;
    if !client.inventory.replace_carried(done.lantern) {
        return Err("the lantern can't be filled there");
    }
    put_back(client, world, &source, done.bottle);
    Ok(done.ml)
}

/// Why filling was refused, for the log.
fn words(refused: FillRefused) -> &'static str {
    match refused {
        FillRefused::NotALantern => "it isn't a lantern",
        FillRefused::NotOil => "the bottle doesn't hold oil",
        FillRefused::Full => "the lantern is full",
    }
}

/// The oil left in the lanterns that burn (held or tied), in millilitres.
fn oil_burning(inventory: &Inventory) -> u32 {
    (inventory.burning_places())
        .filter_map(|item| item.lantern())
        .map(|fount| fount.ml)
        .sum()
}

impl Client {
    /// Burns `secs` of playtime from the lanterns that burn: whether the
    /// client should be sent its inventory (one went out, or another
    /// `TELL_EVERY_ML` went). Viewers learn of the light going out from
    /// `EntityLook::light` (`entities::sync`).
    pub(super) fn burn_lanterns(&mut self, secs: u32) -> bool {
        let before = oil_burning(&self.inventory);
        if before == 0 {
            return false;
        }
        let burnt = self.inventory.burn_lanterns(secs);
        let after = oil_burning(&self.inventory);
        burnt.went_out || before / TELL_EVERY_ML != after / TELL_EVERY_ML
    }
}

#[cfg(test)]
mod tests;
