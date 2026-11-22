//! Candles on a table (tavern furniture plan PR 5): furniture standing on a
//! table or bar counter, as a spirit rack does, giving light
//! (`decor::CANDLE_LIGHT_CELLS`). Always lit, a first guess: they are only put
//! in lived-in places.

use serde::{Deserialize, Serialize};

/// How the candles stand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Candles {
    /// One candle in a turned wooden candlestick.
    Candlestick,
    /// Two candlesticks side by side.
    Pair,
    /// A branched candelabrum of three candles.
    Candelabrum,
}

impl Candles {
    pub const ALL: [Self; 3] = [Self::Candlestick, Self::Pair, Self::Candelabrum];

    /// How many candles burn.
    pub fn candles(self) -> u8 {
        match self {
            Self::Candlestick => 1,
            Self::Pair => 2,
            Self::Candelabrum => 3,
        }
    }
}
