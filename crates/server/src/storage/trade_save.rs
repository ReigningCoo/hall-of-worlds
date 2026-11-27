//! What is saved of a character's trading (trading plan PR 5): their Trading
//! setting and their receive box. A trade itself is never saved: going
//! resets it, nothing moved.

use serde::{Deserialize, Serialize};
use shared::trade::{Received, TradingSetting};

/// A character's Trading setting and what waits in their receive box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TradeSave {
    #[serde(default)]
    pub setting: TradingSetting,
    #[serde(default)]
    pub received: Received,
}

impl TradeSave {
    /// As a new character's: nothing worth writing.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}
