//! Furniture with places in general (`shared::furniture::Held`, tavern
//! furniture plan PR 1): crafting benches' tools and cask racks' casks, kept
//! by the piece's cell in the level's save (`LevelSave::benches`, `racks`),
//! put on and taken off through the same messages (`connections/bench.rs`).

use shared::furniture::{Held, Holder, Piece};
use shared::spot::Spot;

use super::{BenchKey, settle_storage};
use crate::ServerWorld;
use crate::sites::Visit;

/// A piece with places on a visit's level: the piece, what it is and where
/// what lies on it is kept.
#[derive(Clone, Copy, Debug)]
pub(crate) struct HeldHere {
    pub(crate) piece: Piece,
    pub(crate) holder: Holder,
    pub(crate) key: BenchKey,
}

impl Visit {
    /// The piece with places covering `(x, y)` on the level, if any.
    pub(crate) fn held_at(&self, x: i64, y: i64) -> Option<HeldHere> {
        let piece = *self.level.level.furniture_at(x, y)?;
        let holder = Holder::of(piece.furniture)?;
        let key = self.plant_key(piece.at.0, piece.at.1);
        Some(HeldHere { piece, holder, key })
    }
}

impl HeldHere {
    /// Whether someone standing at `at` can reach it to put things on or
    /// take them off: any of its cells' middles within
    /// `potion::COLLECT_REACH`, as for using a bench.
    pub(crate) fn in_reach(&self, at: Spot) -> bool {
        let reach = u128::from(shared::potion::COLLECT_REACH).pow(2);
        (self.piece.cells()).any(|cell| at.distance_squared(Spot::middle_of(cell)) <= reach)
    }
}

/// Real time now, in Unix seconds, as things on the ground go by.
fn unix_now() -> u64 {
    crate::quests::unix_now().max(0) as u64
}

/// What lies on the piece kept at `key`, settled to now: a bench's food in
/// storage rested at `celsius` (`settle_storage`), a rack's casks aged by
/// world time (`RackCasks::settle`), kept so without telling anyone (what
/// changes is sent with it). Nothing if nothing was ever put there, or what
/// was kept is for another piece.
pub(crate) fn settle_held(
    world: &mut ServerWorld,
    key: BenchKey,
    holder: Holder,
    celsius: i32,
) -> Held {
    let rack = match holder {
        Holder::Bench(craft) => return Held::Bench(settle_storage(world, key, craft, celsius)),
        Holder::Casks(rack) => rack,
    };
    let (id, index, at) = key;
    let level = world.site(id).levels.get(&index);
    let kept = (level.and_then(|level| level.racks.get(&at))).filter(|casks| casks.rack == rack);
    let Some(mut casks) = kept.cloned() else {
        return Held::new(holder);
    };
    if casks.settle(unix_now()) {
        let saved = world.site_mut(id).levels.entry(index).or_default();
        saved.racks.insert(at, casks.clone());
    }
    Held::Casks(casks)
}

/// Keeps what lies on the piece at `key` and has it sent to everyone who has
/// the piece.
pub(crate) fn keep_held(world: &mut ServerWorld, key: BenchKey, held: Held) {
    let (id, index, at) = key;
    let saved = world.site_mut(id).levels.entry(index).or_default();
    match held {
        Held::Bench(tools) => drop(saved.benches.insert(at, tools)),
        Held::Casks(casks) => drop(saved.racks.insert(at, casks)),
    }
    world.bench_changes.push(key);
}
