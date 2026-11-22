//! Chandeliers: the one thing hanging from a ceiling (owner's rule).

use serde::{Deserialize, Serialize};

/// How a chandelier is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ChandelierStyle {
    /// A wrought iron hoop of candles on chains.
    IronHoop,
    /// Two crossed wooden beams, a candle at each end.
    WoodenCross,
    /// Branched brass arms in two tiers.
    Brass,
}

impl ChandelierStyle {
    pub const ALL: [Self; 3] = [Self::IronHoop, Self::WoodenCross, Self::Brass];

    /// How many candles it holds.
    pub fn candles(self) -> u8 {
        match self {
            Self::IronHoop => 8,
            Self::WoodenCross => 4,
            Self::Brass => 9,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::IronHoop => "Iron Chandelier",
            Self::WoodenCross => "Wooden Chandelier",
            Self::Brass => "Brass Chandelier",
        }
    }
}
