//! `CaskTime`: a cask's clocks (drinks plan PR 6, owner's decision 2): what
//! is still fermenting, how long it has aged sealed, how long it has been
//! opened, and when it was put on the ground. `shared::ferment` lets time
//! pass on them; the server counts world time on the ground and playtime in
//! the pack.

use serde::{Deserialize, Serialize};

use crate::item::Quality;

/// A cask's clocks, in real seconds. All zero for a cask holding nothing
/// that changes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaskTime {
    /// Seconds still to ferment: while above 0 the cask is fermenting,
    /// sealed, and nothing is poured from or into it.
    #[serde(default)]
    pub fermenting: u64,
    /// The quality its drink fermented to, before ageing: set by Ferment,
    /// none for a drink poured in.
    #[serde(default)]
    pub made: Option<Quality>,
    /// Seconds aged sealed since it finished fermenting.
    #[serde(default)]
    pub aged: u64,
    /// Seconds since it was opened holding what it holds: towards vinegar,
    /// or juice fermenting wild.
    #[serde(default)]
    pub opened_for: u64,
    /// Whether its owner has been told it turns to vinegar soon.
    #[serde(default)]
    pub warned: bool,
    /// When it was put on the ground (seconds since 1970), while it lies
    /// there: its clocks then go by world time.
    #[serde(default)]
    pub lying_since: Option<u64>,
}

impl CaskTime {
    pub fn is_fermenting(&self) -> bool {
        self.fermenting > 0
    }
}
