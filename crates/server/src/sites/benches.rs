//! Crafting benches' tools (`shared::crafting_bench`, crafting benches plan
//! PR 2): kept in the save of the level a bench stands on, by the bench's
//! cell (`LevelSave::benches`), a first level's in its own site's save even
//! where it is played set into the wilderness (as plants are, `plant_key`).
//! Generated benches are given their starting tools once (`stock.rs`); a
//! bench's tools go to a player with its cells and again whenever they change
//! (`share.rs`); benches made with the developer tool are kept (`dev.rs`).

use shared::crafting_bench::{BenchTools, Craft};
use shared::furniture::{Furniture, Piece};
use shared::spot::Spot;

use super::Visit;
use crate::ServerWorld;
use crate::storage::SiteId;

// Split into files (`cargo xtask lint` keeps files small).
mod dev;
mod share;
mod stock;

pub(crate) use dev::create_bench;
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

impl BenchHere {
    /// Whether someone standing at `at` can reach it to put tools on or take
    /// them off: any of its cells' middles within `potion::COLLECT_REACH`, as
    /// for using it.
    pub(crate) fn in_reach(&self, at: Spot) -> bool {
        let reach = u128::from(shared::potion::COLLECT_REACH).pow(2);
        (self.piece.cells()).any(|cell| at.distance_squared(Spot::middle_of(cell)) <= reach)
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

/// Keeps the tools on the bench at `key` and has them sent to everyone who
/// has the bench.
pub(crate) fn keep_bench(world: &mut ServerWorld, key: BenchKey, tools: BenchTools) {
    let (id, index, at) = key;
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.benches.insert(at, tools);
    world.bench_changes.push(key);
}

#[cfg(test)]
mod tests;
