//! Harvesting herb patches: what the server answers a
//! `ClientMessage::PickHerb { x, y }` with (the patch beside or under the
//! player, `LevelChunk::herbs`, in turn with their steps).

use serde::{Deserialize, Serialize};

use crate::item::Quality;

/// The answer to `ClientMessage::PickHerb`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HerbNews {
    /// The patch at `(x, y)` was harvested: `count` of the herb with key
    /// `herb`, at `quality`, now carried (or on the ground under the player, if
    /// it didn't fit). `Inventory`, `Sheet` and the patch's chunk follow.
    Picked {
        x: i64,
        y: i64,
        herb: String,
        quality: Quality,
        count: u32,
    },
    /// Nothing could be harvested at `(x, y)`: no patch there, out of reach,
    /// not grown enough, or nothing to take this early.
    Refused { x: i64, y: i64 },
}
