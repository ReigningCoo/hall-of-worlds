//! Pouring at a bar (`BenchAsk::Pour`, tavern furniture plan PR 3): a drink
//! stored on furniture in the bar's room (`shared::bar::room_storage`: cask
//! racks, cradles, Tun stands, wine and spirit racks, bar counters) into a
//! carried vessel, by the pour rule (`shared::pour`, a tap carried to draw
//! from a cask). Never from anything lying on the ground (owner's), and only
//! at a bar counter in reach (checked by `bench.rs`). What was poured from
//! keeps its place, changed.

use bevy::prelude::*;
use shared::furniture::Holder;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::pour::{PourRefused, pour_cost};

use super::Client;
use super::pour::{put_back, side};
use crate::ServerWorld;
use crate::sites::{HeldHere, keep_held, settle_held};

/// Pours `from` into `into` at the bar `here`: what it cost (nothing if
/// refused, which is only logged).
pub(super) fn pour_at_bar(
    client: &mut Client,
    world: &mut ServerWorld,
    here: HeldHere,
    (from, into): (Id, Id),
) -> u32 {
    match poured(client, world, here, (from, into)) {
        Ok(ml) => {
            client.unsaved = true;
            pour_cost(ml)
        }
        Err(why) => {
            warn!(
                "{} refused pouring {from:?} into {into:?} at a bar: {why}",
                client.in_log()
            );
            0
        }
    }
}

/// Pours by the rule: how many millilitres went.
fn poured(
    client: &mut Client,
    world: &mut ServerWorld,
    here: HeldHere,
    (from, into): (Id, Id),
) -> Result<u32, &'static str> {
    if !matches!(here.holder, Holder::Bar(_)) {
        return Err("that isn't a bar");
    }
    let carried = client.inventory.find(into);
    if !carried.is_some_and(|found| found.kind == ItemKind::Vessel) {
        return Err("it isn't a carried vessel");
    }
    let target = side(client, world, into)?;
    let (stored, (piece, mut held)) = stored_drink(client, world, here, from)?;
    let tap = client.inventory.count_of(ItemKind::Tap) > 0;
    let done = shared::pour::pour(&stored, target.stack.top(), tap).map_err(PourRefused::words)?;
    if !held.replace(done.from) {
        return Err("it isn't stored there");
    }
    keep_held(world, piece.key, held);
    put_back(client, world, &target, done.into);
    Ok(done.ml)
}

/// The drink `id` stored in the bar's room, and the piece it lies on with
/// everything on that, settled to now.
#[allow(clippy::type_complexity)]
fn stored_drink(
    client: &Client,
    world: &mut ServerWorld,
    bar: HeldHere,
    id: Id,
) -> Result<(Item, (HeldHere, shared::furniture::Held)), &'static str> {
    let Some(visit) = &client.visit else {
        return Err("not on a level");
    };
    let celsius = world.habitat_at(client.world_position()).celsius;
    for here in visit.bar_storage(&bar.piece) {
        let key = here.key;
        let holder = here.holder;
        let held = settle_held(world, key, holder, celsius);
        let found = shared::bar::drinks_in(&held)
            .find(|item| item.id == id)
            .cloned();
        if let Some(item) = found {
            return Ok((item, (here, held)));
        }
    }
    Err("no such drink is stored in the bar's room")
}
