//! What one run of a cooking process takes (`cook.rs`), once checked: each
//! source whole (carried, or from the hearth's storage), each food stack's
//! count, whole bottles given back empty, vessels and casks drawn from.

use shared::cooking::{CookOrder, OrderInput, drawn};
use shared::item::ItemKind;
use shared::pour::BOTTLE_ML;

use super::Client;
use super::cook_kitchen::KitchenHere;
use super::give::give;
use crate::ServerWorld;
use crate::ids::new_id;
use crate::sites::{bench_tools, keep_bench};

/// Takes what one run of `order` uses at `here`.
pub(super) fn use_up(
    client: &mut Client,
    world: &mut ServerWorld,
    order: &CookOrder,
    here: &KitchenHere,
) {
    for &source in order.sources.iter().flatten() {
        if client.inventory.take_one(source, &mut new_id).is_some() {
            continue;
        }
        if let Some((key, craft)) = here.bench {
            let mut tools = bench_tools(world, key, craft);
            if tools.take(source).is_some() {
                keep_bench(world, key, tools);
            }
        }
    }
    for input in order.steps.iter().flat_map(|step| &step.inputs) {
        match *input {
            OrderInput::Food { stack, count } => {
                for _ in 0..count {
                    client.inventory.take_one(stack, &mut new_id);
                }
            }
            OrderInput::Liquid { stack, ml } => draw(client, world, stack, ml),
            OrderInput::WaterFaced { .. } => {}
        }
    }
}

/// Draws `ml` from the carried container stack `stack`: whole bottles taken
/// and given back empty, a vessel or cask left holding less.
fn draw(client: &mut Client, world: &mut ServerWorld, stack: shared::id::Id, ml: u32) {
    let Some(item) = client.inventory.find(stack) else {
        return;
    };
    if item.top().kind == ItemKind::Bottle {
        for _ in 0..ml / BOTTLE_ML {
            if let Some(bottle) = client.inventory.take_one(stack, &mut new_id) {
                give(client, world, drawn(&bottle, BOTTLE_ML));
            }
        }
        return;
    }
    let less = drawn(item, ml);
    client.inventory.replace_carried(less);
}
