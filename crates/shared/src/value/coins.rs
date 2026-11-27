//! A sum of copper split into the coins it is shown as.

use crate::item::COINS_PER_NEXT;

/// A sum as gold, silver and copper, each under 100 but gold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coins {
    pub gold: u64,
    pub silver: u64,
    pub copper: u64,
}

impl Coins {
    /// `copper` as the fewest coins.
    pub const fn from_copper(copper: u64) -> Self {
        let next = COINS_PER_NEXT as u64;
        Self {
            gold: copper / (next * next),
            silver: copper / next % next,
            copper: copper % next,
        }
    }
}
