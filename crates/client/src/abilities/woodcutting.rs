//! Fell Tree and Cut Kindling chosen by hand (woodcutting plan PR 4): what
//! each is aimed at, as the server will check it (`connections/fell_tree.rs`,
//! `cut_kindling.rs`).

use shared::dungeon::Level;
use shared::facing::{Heading, TURN, faced_cell};
use shared::id::Id;
use shared::item::{Inventory, Item};
use shared::potion::COLLECT_REACH;
use shared::spot::Spot;
use shared::woodcutting::{LogAt, holds_tree, log_to_cut, tree_faced};

/// Whether Fell Tree can be asked for by someone at `at` facing `heading`:
/// `Some(None)` with a tree faced, `Some(Some((cell, heading)))` with one in
/// reach beside or behind them, which they first turn to (told to the server
/// before the ask, as it checks the tree faced), `None` with none in reach.
pub(super) fn tree_to_fell(
    level: &Level,
    at: Spot,
    heading: Heading,
) -> Option<Option<((i64, i64), Heading)>> {
    if tree_faced(level, at, heading).is_some() {
        return Some(None);
    }
    let all_round = (TURN / 2) as u16;
    let cell = faced_cell(level, at, heading, COLLECT_REACH, all_round, holds_tree)?;
    Heading::towards(at, Spot::middle_of(cell)).map(|towards| Some((cell, towards)))
}

/// What Cut Kindling is aimed at: `Some(None)` for a log carried (the server
/// takes the first), `Some(Some(id))` for the first on the ground while the
/// ground's storage is on screen (`inventory::GroundShown`), `None` for no log.
pub(super) fn log_aim(carried: &Inventory, ground: &[Item]) -> Option<Option<Id>> {
    match log_to_cut(carried, Some(ground))? {
        LogAt::Carried(_) => Some(None),
        LogAt::Ground(id) => Some(Some(id)),
    }
}

#[cfg(test)]
mod tests;
