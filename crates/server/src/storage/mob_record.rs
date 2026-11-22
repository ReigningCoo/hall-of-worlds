//! What a mob has come to know and own, and what it was doing, kept in its
//! level's save (`LevelSave::records`, `unsettled`).

use serde::{Deserialize, Serialize};
use shared::item::{Inventory, Item};
use shared::mob::Memory;

/// What a mob has come to know and own.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobRecord {
    pub memory: Memory,
    pub carried: Vec<Item>,
    /// What it wears and carries: rolled when it is first needed, then its own.
    #[serde(default)]
    pub gear: Option<Inventory>,
}

/// What a mob was doing, for carrying on after the server has been off.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedMob {
    /// The cell, for older saves: `spot` is where it is.
    pub position: (i64, i64),
    #[serde(default)]
    pub spot: Option<shared::spot::Spot>,
    pub doing: crate::entities::Doing,
}
