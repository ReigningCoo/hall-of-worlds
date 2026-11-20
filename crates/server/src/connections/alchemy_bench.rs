//! Alchemy at a crafting bench (`AlchemyOrder::bench`, crafting benches plan
//! PR 2): the player must be at the bench asked for, facing it, before each
//! run; its tools are used before carried ones (`Inventory::check_alchemy_at`),
//! and with one on it Alchemy can begin with no apparatus carried.

use shared::ability::Ability;
use shared::alchemy::AlchemyOrder;
use shared::crafting_bench::{BenchTools, Craft};

use super::Client;
use crate::ServerWorld;
use crate::sites::bench_tools;

/// The tools of the crafting bench `order` asks to work at, if the player is
/// at it (`Visit::bench_faced`); `Err` if they aren't, `None` if it asks for
/// none. Checked before each run, so walking off or turning away stops it.
pub(super) fn order_bench(
    client: &Client,
    world: &mut ServerWorld,
    order: &AlchemyOrder,
) -> Result<Option<BenchTools>, ()> {
    let Some(asked) = order.bench else {
        return Ok(None);
    };
    let visit = client.visit.as_ref().ok_or(())?;
    let here = visit.bench_faced(client.heading).ok_or(())?;
    if here.craft != Craft::Alchemy || !here.piece.cells().any(|cell| cell == asked) {
        return Err(());
    }
    Ok(Some(bench_tools(world, here.key, here.craft)))
}

/// Whether the player can use Alchemy without carrying an apparatus: they
/// know it, and the order kept asks for a bench they are at that holds one.
pub(super) fn can_use_at_bench(client: &Client, world: &mut ServerWorld) -> bool {
    let known = Ability::Alchemy.is_known(client.inventory.creature, &client.sheet);
    let Some(order) = client.alchemy.as_ref().filter(|_| known) else {
        return false;
    };
    let bench = order_bench(client, world, order);
    bench.is_ok_and(|tools| tools.is_some_and(|tools| !tools.tools().is_empty()))
}
