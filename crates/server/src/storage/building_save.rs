//! What players have done with a town's building, saved with its level.

use serde::{Deserialize, Serialize};
use shared::id::Id;
use shared::item::Item;
use std::collections::BTreeMap;

/// What players have done with a town's building.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildingSave {
    /// Trades made by players, on top of the number it was generated with.
    pub trades: u32,
    /// The server's clock (game minutes) when last interacted with, for its stock: in the
    /// future if the clock was moved back (dev tools), counting as no time passed.
    pub last_interaction: Option<f64>,
    /// Who keeps it, if a shop: a mob never on the map, in the level's `records`, whose
    /// `carried` is what the shop has.
    #[serde(default)]
    pub keeper: Option<Id>,
    /// What it had for sale before it had a keeper: moved to them when they come.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stock: Vec<Item>,
    /// The game minute it last finished making something.
    #[serde(default)]
    pub last_made: Option<u64>,
    /// What players sold it, by the game minute each was sold (`shop::sold_gone`).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sold: BTreeMap<Id, u64>,
}
