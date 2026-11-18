//! Pouring (`ClientMessage::Pour`, drinks plan PR 2): one cask, vessel or
//! bottle into another, each carried or lying in reach, by `shared::pour`'s
//! rule (a tap carried to draw from a cask). Of a stack, its top one is poured
//! and given back on its own; one alone changes where it is.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::pour::{PourRefused, pour_cost};

use super::Client;
use super::grind::give;
use crate::ServerWorld;
use crate::ids::new_id;

/// A container to pour from or into: its whole stack, and whether it is carried
/// (else lying in reach).
pub(super) struct Side {
    pub(super) stack: Item,
    carried: bool,
}

/// The container `id`, carried (not worn) or lying in the player's reach.
pub(super) fn side(client: &Client, world: &mut ServerWorld, id: Id) -> Result<Side, &'static str> {
    if let Some(found) = client.inventory.find(id) {
        if client.inventory.is_worn(id) || found.holds_quest_item() {
            return Err("it can't be used from there");
        }
        return Ok(Side {
            stack: found.clone(),
            carried: true,
        });
    }
    let stack = crate::ground::lying_stack(client, world, id)?;
    Ok(Side {
        stack,
        carried: false,
    })
}

/// Puts `changed`, the side's top one with a new form, back: in place if it
/// was alone, else taken off its stack and given to the player.
pub(super) fn put_back(client: &mut Client, world: &mut ServerWorld, side: &Side, changed: Item) {
    if side.stack.count() == 1 {
        match side.carried {
            true => {
                client.inventory.replace_carried(changed);
            }
            false => crate::ground::replace_lying(client, world, changed),
        }
        return;
    }
    let taken = match side.carried {
        true => client.inventory.take_one(side.stack.id, &mut new_id),
        false => crate::ground::take_top_lying(client, world, side.stack.id).ok(),
    };
    if let Some(mut one) = taken {
        one.form = changed.form;
        give(client, world, one);
    }
}

/// Pours `from` into `into`: whether the client is still there, and what it
/// cost (nothing if refused).
pub(super) fn pour(
    client: &mut Client,
    world: &mut ServerWorld,
    from: Id,
    into: Id,
) -> (bool, u32) {
    let cost = match poured(client, world, from, into) {
        Ok(ml) => pour_cost(ml),
        Err(why) => {
            warn!(
                "{} refused pouring {from:?} into {into:?}: {why}",
                client.in_log()
            );
            0
        }
    };
    client.unsaved = true;
    client.ground_told.again = true;
    (client.send_inventory(), cost)
}

/// Pours by the rule: how many millilitres went.
fn poured(
    client: &mut Client,
    world: &mut ServerWorld,
    from: Id,
    into: Id,
) -> Result<u32, &'static str> {
    let source = side(client, world, from)?;
    let target = side(client, world, into)?;
    let tap = client.inventory.count_of(ItemKind::Tap) > 0;
    let done = shared::pour::pour(source.stack.top(), target.stack.top(), tap)
        .map_err(PourRefused::words)?;
    put_back(client, world, &source, done.from);
    put_back(client, world, &target, done.into);
    Ok(done.ml)
}
