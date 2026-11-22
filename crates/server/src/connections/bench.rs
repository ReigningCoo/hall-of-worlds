//! Putting things on furniture with places and taking them off
//! (`ClientMessage::Bench`, crafting benches plan PR 2; any piece with places
//! since the tavern furniture plan PR 1), in turn with steps: the piece must
//! be in reach (`sites::HeldHere::in_reach`). Anyone may put or take
//! (owner's): tools on a bench, food in a Range's storage (cooking plan PR 8),
//! where its clocks run on real time at the land's warmth as on the ground,
//! casks on a rack of their size, bottles on a wine or spirit rack (one off
//! a stack), whose clocks go by world time as on the ground. A tool of a kind already there swaps with it; anything the piece
//! won't take is refused with `BenchRefused`, never saying why.

use bevy::prelude::*;
use shared::protocol::{BenchAsk, BenchNews, ServerMessage};

use super::Client;
use super::give::give;
use crate::ServerWorld;
use crate::sites::{HeldHere, keep_held, settle_held};

/// Real time now, in Unix seconds, as food on the ground rests by.
fn unix_now() -> u64 {
    crate::quests::unix_now().max(0) as u64
}

/// Does what was asked of the bench. Whether the client is still there, and
/// what it cost (nothing).
pub(super) fn bench_act(
    client: &mut Client,
    world: &mut ServerWorld,
    ask: BenchAsk,
) -> (bool, u32) {
    let (BenchAsk::Put { bench, .. } | BenchAsk::Take { bench, .. } | BenchAsk::Pour { bench, .. }) =
        ask
    else {
        return (true, 0);
    };
    let Some(visit) = &client.visit else {
        return (client.send_inventory(), 0);
    };
    let Some(here) = visit.held_at(bench.0, bench.1) else {
        debug!("No furniture with places at {bench:?}");
        return (client.send_inventory(), 0);
    };
    if !here.in_reach(visit.spot()) {
        debug!("The furniture with places at {bench:?} is out of reach");
        return (client.send_inventory(), 0);
    }
    let refused = match ask {
        BenchAsk::Pour { from, into, .. } => {
            let cost = super::bar_pour::pour_at_bar(client, world, here, (from, into));
            return (client.send_inventory(), cost);
        }
        BenchAsk::Put { item, .. } => !put(client, world, here, item),
        BenchAsk::Dev(_) => false,
        BenchAsk::Take { item, .. } => {
            take(client, world, here, item);
            false
        }
    };
    let stayed = !refused || client.link.send(&ServerMessage::Bench(BenchNews::Refused));
    (stayed && client.send_inventory(), 0)
}

/// Puts carried `item` on the piece, giving back what it swaps with. Whether
/// the piece took it; something that can't be let go of is not refused by
/// the piece, only left where it is.
fn put(client: &mut Client, world: &mut ServerWorld, here: HeldHere, item: shared::id::Id) -> bool {
    client.inventory.lift_part(item);
    // Of a stack (bottles), only one goes, one to a place.
    let stacked = client
        .inventory
        .find(item)
        .is_some_and(|found| found.count() > 1);
    let item = match stacked {
        true => match client
            .inventory
            .lift_count(item, 1, &mut crate::ids::new_id)
        {
            Some(one) => one,
            None => return true,
        },
        false => item,
    };
    let Some(found) = client.inventory.find(item) else {
        return true;
    };
    let celsius = world.habitat_at(client.world_position()).celsius;
    let mut tools = settle_held(world, here.key, here.holder, celsius);
    if tools.clone().put(found.clone()).is_err() {
        return false;
    }
    let tick = world.action_clock.ticks();
    let letting_go = |after: &mut shared::item::Inventory| drop(after.hand_over(item));
    if !client.may_take_off(item) || !client.change_allowed(letting_go, tick) {
        return true;
    }
    let Some(mut item) = client.inventory.hand_over(item) else {
        return true;
    };
    // Food left in storage rests as on the ground, casks and sealed bottles
    // on a rack go by world time (tools have no clocks).
    let now = unix_now();
    item.lay_food_down(now);
    item.lay_casks_down(now);
    let Ok(old) = tools.put(item) else {
        unreachable!("checked on a copy");
    };
    if let Some(old) = old {
        give(client, world, old);
    }
    keep_held(world, here.key, tools);
    client.unsaved = true;
    client.update_speed(tick);
    true
}

/// Takes `item` off the piece into what is carried (or onto the ground).
fn take(client: &mut Client, world: &mut ServerWorld, here: HeldHere, item: shared::id::Id) {
    let celsius = world.habitat_at(client.world_position()).celsius;
    let mut tools = settle_held(world, here.key, here.holder, celsius);
    let Some(mut tool) = tools.take(item) else {
        return;
    };
    // Stored food's and racked casks' time lying there counted (a cask's last
    // seconds as on the ground), their clocks carried from now.
    let now = unix_now();
    tool.rest_food(0, now, celsius);
    let _ = tool.age_casks(0, now);
    keep_held(world, here.key, tools);
    give(client, world, tool);
    client.unsaved = true;
    client.update_speed(world.action_clock.ticks());
}
