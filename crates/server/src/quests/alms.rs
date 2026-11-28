//! Alms (trading plan PR 7): coins given to a town's beggars, by Give Item or
//! in a trade, raise the giver's standing there a little by the amount, up to
//! a few points a day. Unlike other helpful acts they count past Known, as
//! tips do. The figures are Claude's, to balance.

use shared::item::Item;
use shared::poi::ChunkPos;

use super::standing::add;
use crate::storage::{Alms, QuestBook};

/// Copper given for each point of standing.
pub(crate) const COPPER_PER_POINT: u64 = 10;

/// The most points a town's beggars give in a UTC day (a fifth of a quest).
pub(crate) const POINTS_A_DAY: u32 = 3;

/// What `item` is worth in coins: a stack of coins, else nothing.
pub(crate) fn copper_in(item: &Item) -> u64 {
    (item.kind.coin_value()).map_or(0, |each| each * u64::from(item.count()))
}

/// The character gave `copper` to beggars of `town` on `day`: the points it
/// adds to their standing there, none past the day's cap. Earlier days'
/// alms are forgotten.
pub(crate) fn give(book: &mut QuestBook, town: ChunkPos, day: i64, copper: u64) -> u32 {
    book.alms.retain(|_, alms| alms.day == day);
    let alms = book.alms.entry(town).or_insert(Alms { day, copper: 0 });
    let before = points(alms.copper);
    alms.copper = alms.copper.saturating_add(copper);
    let gained = points(alms.copper) - before;
    if gained > 0 {
        add(book, town, gained);
    }
    gained
}

/// The points a day's `copper` is worth, capped.
fn points(copper: u64) -> u32 {
    (copper / COPPER_PER_POINT).min(u64::from(POINTS_A_DAY)) as u32
}

#[cfg(test)]
mod tests;
