//! Putting tools on a crafting bench and taking them off
//! (`ClientMessage::Bench`, crafting benches plan PR 2), in turn with steps:
//! the bench must be in reach (`sites::BenchHere::in_reach`). Anyone may put
//! or take (owner's), food in a Range's storage too (cooking plan PR 8),
//! where its clocks run on real time at the land's warmth as on the ground. A tool of a kind already there swaps with it; anything
//! the bench won't take is refused with `BenchRefused`, never saying why.

use bevy::prelude::*;
use shared::protocol::{BenchAsk, BenchNews, ServerMessage};

use super::Client;
use super::give::give;
use crate::ServerWorld;
use crate::sites::{BenchHere, bench_tools, keep_bench, settle_storage};

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
    let (BenchAsk::Put { bench, .. } | BenchAsk::Take { bench, .. }) = ask else {
        return (true, 0);
    };
    let Some(visit) = &client.visit else {
        return (client.send_inventory(), 0);
    };
    let Some(here) = visit.bench_at(bench.0, bench.1) else {
        debug!("No crafting bench at {bench:?}");
        return (client.send_inventory(), 0);
    };
    if !here.in_reach(visit.spot()) {
        debug!("The crafting bench at {bench:?} is out of reach");
        return (client.send_inventory(), 0);
    }
    let refused = match ask {
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

/// Puts carried `item` on the bench, giving back what it swaps with. Whether
/// the bench took it; something that can't be let go of is not refused by
/// the bench, only left where it is.
fn put(
    client: &mut Client,
    world: &mut ServerWorld,
    here: BenchHere,
    item: shared::id::Id,
) -> bool {
    client.inventory.lift_part(item);
    let Some(found) = client.inventory.find(item) else {
        return true;
    };
    let mut tools = bench_tools(world, here.key, here.craft);
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
    // Food left in storage rests as on the ground (tools have no clocks).
    item.lay_food_down(unix_now());
    let Ok(old) = tools.put(item) else {
        unreachable!("checked on a copy");
    };
    if let Some(old) = old {
        give(client, world, old);
    }
    keep_bench(world, here.key, tools);
    client.unsaved = true;
    client.update_speed(tick);
    true
}

/// Takes tool `item` off the bench into what is carried (or onto the ground).
fn take(client: &mut Client, world: &mut ServerWorld, here: BenchHere, item: shared::id::Id) {
    let celsius = world.habitat_at(client.world_position()).celsius;
    let mut tools = settle_storage(world, here.key, here.craft, celsius);
    let Some(mut tool) = tools.take(item) else {
        return;
    };
    // Stored food's time lying there counted, its clock carried from now.
    tool.rest_food(0, unix_now(), celsius);
    keep_bench(world, here.key, tools);
    give(client, world, tool);
    client.unsaved = true;
    client.update_speed(world.action_clock.ticks());
}
