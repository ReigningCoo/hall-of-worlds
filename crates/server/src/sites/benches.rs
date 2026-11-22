//! Crafting benches' tools (`shared::crafting_bench`, crafting benches plan
//! PR 2): kept in the save of the level a bench stands on, by the bench's
//! cell (`LevelSave::benches`), a first level's in its own site's save even
//! where it is played set into the wilderness (as plants are, `plant_key`).
//! Generated benches are given their starting tools once (`stock.rs`); a
//! bench's tools go to a player with its cells and again whenever they change
//! (`share.rs`); benches made with the developer tool are kept (`dev.rs`).
//! Cask racks are kept the same way, all as furniture with places
//! (`held.rs`, `shared::furniture::Held`).

use shared::crafting_bench::{BenchTools, Craft};
use shared::furniture::{Furniture, Piece};

use super::Visit;
use crate::ServerWorld;
use crate::storage::SiteId;

// Split into files (`cargo xtask lint` keeps files small).
mod dev;
mod held;
mod share;
mod stock;

pub(crate) use dev::create_bench;
pub(crate) use held::{HeldHere, fireside, keep_held, settle_held};
pub(crate) use share::share_benches;
pub(super) use stock::{put_made_benches, stock_benches};

/// Where a bench's tools are kept: the site, its level and the bench's cell
/// (`Piece::at`) there.
pub(crate) type BenchKey = (SiteId, u32, (i64, i64));

/// A crafting bench on a visit's level: its piece, craft and where its tools
/// are kept.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BenchHere {
    pub(crate) piece: Piece,
    pub(crate) craft: Craft,
    pub(crate) key: BenchKey,
}

impl Visit {
    /// The crafting bench covering `(x, y)` on the level, if any.
    pub(crate) fn bench_at(&self, x: i64, y: i64) -> Option<BenchHere> {
        let piece = *self.level.level.furniture_at(x, y)?;
        let Furniture::CraftingBench { craft } = piece.furniture else {
            return None;
        };
        let key = self.plant_key(piece.at.0, piece.at.1);
        Some(BenchHere { piece, craft, key })
    }

    /// The crafting bench the player is at to use it: within reach and
    /// facing it (`crafting_bench::bench_faced`).
    pub(crate) fn bench_faced(&self, heading: shared::facing::Heading) -> Option<BenchHere> {
        let piece = shared::crafting_bench::bench_faced(&self.level.level, self.spot(), heading)?;
        self.bench_at(piece.at.0, piece.at.1)
    }
}

/// The tools on the bench kept at `key`, empty if none were ever put there.
pub(crate) fn bench_tools(world: &mut ServerWorld, key: BenchKey, craft: Craft) -> BenchTools {
    let (id, index, at) = key;
    let level = world.site(id).levels.get(&index);
    (level.and_then(|level| level.benches.get(&at)))
        .filter(|tools| tools.craft == craft)
        .cloned()
        .unwrap_or_else(|| BenchTools::new(craft))
}

/// The bench kept at `key` (`bench_tools`) with the food in its storage
/// rested to now at `celsius` (as food on the ground, `shared::food`), kept
/// so without telling anyone (what changes is sent with it).
pub(crate) fn settle_storage(
    world: &mut ServerWorld,
    key: BenchKey,
    craft: Craft,
    celsius: i32,
) -> BenchTools {
    let mut tools = bench_tools(world, key, craft);
    if tools.storage().is_empty() {
        return tools;
    }
    let now = crate::quests::unix_now().max(0) as u64;
    let mut changed = false;
    for item in tools.storage_mut() {
        changed |= item.settle_lying_food(now, celsius).seen();
    }
    if changed {
        let (id, index, at) = key;
        let saved = world.site_mut(id).levels.entry(index).or_default();
        saved.benches.insert(at, tools.clone());
    }
    tools
}

/// Keeps the tools on the bench at `key` and has them sent to everyone who
/// has the bench.
pub(crate) fn keep_bench(world: &mut ServerWorld, key: BenchKey, tools: BenchTools) {
    keep_held(world, key, shared::furniture::Held::Bench(tools));
}

#[cfg(test)]
mod bar_tests;
#[cfg(test)]
mod bottle_rack_tests;
#[cfg(test)]
mod rack_tests;
#[cfg(test)]
mod tests;
