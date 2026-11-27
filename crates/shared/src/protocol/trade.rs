//! Trading with a shop (`shared::trade`): what the client asks once a trade
//! is open (`ClientMessage::OpenTrade`), and what the server tells it.
//! Nothing moves until `Complete`; anything else only changes the trade.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::site::BuildingKind;
use crate::trade::{Buy, Payment, Rates, Refusal, ShopTrade};

/// A change to the open trade, or completing or closing it. Each is answered
/// with `TradeNews::State`, or `Refused` and the state as it was.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeAsk {
    /// Something from the shop's stock to buy, and where it is to go.
    Buy(Buy),
    /// Something being bought put back.
    Unbuy { item: Id },
    /// Something carried to sell: it stays carried until completed.
    Sell { item: Id },
    /// Something being sold taken back.
    Unsell { item: Id },
    /// What to pay in copper instead of what is due (negative: to be paid),
    /// a tip or a haggle; none: what is due.
    Pay(Option<i64>),
    /// Everything at once, or nothing: answered with `Ended`, the inventory
    /// and the stock, or `Refused`.
    Complete,
    /// Closing it: nothing moves.
    Close,
}

/// What the server tells of a trade.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeNews {
    /// A trade opened with a shop's keeper (after its `TradeStock`): its rates
    /// for this character and its kind, to work out the totals the same way.
    Opened {
        with: Id,
        rates: Rates,
        shop: BuildingKind,
    },
    /// The trade as the server holds it, after any change.
    State(ShopTrade),
    /// A change or completing refused, and why.
    Refused(Refusal),
    /// The trade is over.
    Ended { with: Id, why: TradeEnd },
}

/// Why a trade ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeEnd {
    /// Completed: `paid` copper (negative: the player was paid), and how.
    Completed { paid: i64, payment: Payment },
    /// Closed, or another opened: nothing moved.
    Closed,
    /// Walked out of reach, or left the level: nothing moved.
    OutOfReach,
}
