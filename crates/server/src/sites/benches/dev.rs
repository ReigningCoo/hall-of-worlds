//! Developer tools: Create Crafting Bench puts a bench with chosen tools on
//! the player's site level and keeps it, unlike Place Furniture.

use bevy::prelude::*;
use shared::crafting_bench::BenchTools;
use shared::furniture::{Furniture, free_for_bench};
use shared::item::Item;
use shared::protocol::DevBench;

use super::keep_bench;
use crate::ServerWorld;
use crate::connections::Client;
use crate::ids::new_id;
use crate::sites::FurnitureChange;

/// Checks the bench (a crafting bench furnishing could make, every cell free
/// floor: no wall, stairs or other furniture, not the wilderness), makes its
/// tools new and keeps both in the level's save, shared at the end of the
/// tick. Refusals are only logged. Whether the client is still welcome.
pub(crate) fn create_bench(client: &mut Client, world: &mut ServerWorld, order: DevBench) -> bool {
    let Some(visit) = &client.visit else {
        return true;
    };
    let piece = order.piece;
    let site = &visit.level;
    let free = |(x, y): (i64, i64)| {
        free_for_bench(&site.level, (x, y)) && !site.stairs.iter().any(|s| (s.x, s.y) == (x, y))
    };
    let Furniture::CraftingBench { craft } = piece.furniture else {
        info!("Refused creating a crafting bench: not one");
        return true;
    };
    if visit.is_wilderness() || !piece.is_well_made() || !piece.cells().all(free) {
        info!("Refused creating a crafting bench at {:?}", piece.at);
        return true;
    }
    let mut tools = BenchTools::new(craft);
    for &(kind, quality) in &order.tools {
        let mut item = Item::new(new_id(), kind);
        item.quality = Some(quality);
        if tools.put(item).is_err() {
            info!("Refused creating a crafting bench: it doesn't take {kind:?}");
            return true;
        }
    }
    let (id, index) = (visit.id, visit.index);
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.made_benches.push(piece);
    keep_bench(world, (id, index, piece.at), tools);
    let place = (id, index);
    world
        .furniture_changes
        .push(FurnitureChange::piece(place, piece));
    true
}
