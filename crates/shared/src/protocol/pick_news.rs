//! Harvesting herb patches, fruit plants, vegetables and fungi: what the server answers a
//! `ClientMessage::Pick { x, y }` with (the patch or plant beside or under the
//! player, `LevelChunk::herbs`, `fruit`, `vegetables` or `fungi`, in turn with their steps).

use serde::{Deserialize, Serialize};

use crate::item::Quality;

/// The answer to `ClientMessage::Pick`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickNews {
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
    /// The fruit plant at `(x, y)` was picked: `count` of the fruit with key
    /// `fruit` now carried (or on the ground under the player). `Inventory`,
    /// `Sheet` and the plant's chunk follow.
    PickedFruit {
        x: i64,
        y: i64,
        fruit: String,
        count: u32,
    },
    /// The vegetable plant at `(x, y)` was picked: `count` of the vegetable with
    /// key `vegetable` now carried (or on the ground under the player).
    /// `Inventory`, `Sheet` and the plant's chunk follow.
    PickedVegetable {
        x: i64,
        y: i64,
        vegetable: String,
        count: u32,
    },
    /// The fungus patch at `(x, y)` was picked: `count` of the fungus with key
    /// `fungus` now carried (or on the ground under the player). `Inventory`,
    /// `Sheet` and the patch's chunk follow.
    PickedFungus {
        x: i64,
        y: i64,
        fungus: String,
        count: u32,
    },
    /// Nothing could be harvested at `(x, y)`: no patch there, out of reach,
    /// not grown enough, or nothing to take this early.
    Refused { x: i64, y: i64 },
}
