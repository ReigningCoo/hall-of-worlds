//! Feeding yeast and starting starters (`shared::yeast`, drinks plan PR 8).

use serde::{Deserialize, Serialize};

use crate::id::Id;

/// A client asking to feed yeast or begin a starter, each answered with
/// `Inventory`, or nothing if refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum YeastAsk {
    /// Feed the carried yeast stack `yeast` from the carried flour stack
    /// `flour`, with fresh water and empty bottles carried
    /// (`yeast::plan_feeding`).
    Feed { yeast: Id, flour: Id },
    /// Begin a starter from the carried flour stack `flour` and a bottle of
    /// fresh water off the carried stack `water` (`yeast::can_start`).
    Start { water: Id, flour: Id },
}
