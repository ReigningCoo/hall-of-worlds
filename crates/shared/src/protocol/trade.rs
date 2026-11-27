//! Trading (`shared::trade`): what the client asks once a trade is open
//! (`ClientMessage::OpenTrade`), and what the server tells it. Nothing moves
//! until a shop trade's `Complete`, or both sides of a trade between
//! characters are locked in; anything else only changes the trade.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::site::BuildingKind;
use crate::trade::{
    Between, Buy, Offer, Payment, Rates, Received, Refusal, ShopTrade, TradingSetting,
};

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
    /// Something being sold, or offered to a character, taken back.
    Unsell { item: Id },
    /// Something carried put on one's side of a trade with a character, or
    /// moved there: it stays carried until settled. Unlocks the side.
    Offer(Offer),
    /// Locking in (the Trade toggle) or out of a trade with a character:
    /// once both sides are locked in it settles.
    Lock(bool),
    /// The answer to being asked to trade (`TradeNews::Asked`) by `by`.
    Answer { by: Id, allow: bool },
    /// One thing taken out of the receive box to a place (works with no trade
    /// open): answered with `Received` and the inventory.
    Take(Buy),
    /// Everything that fits taken out of the receive box.
    TakeAll,
    /// What to pay in copper instead of what is due (negative: to be paid),
    /// a tip or a haggle; none: what is due.
    Pay(Option<i64>),
    /// Everything at once, or nothing: answered with `Ended`, the inventory
    /// and the stock, or `Refused`.
    Complete,
    /// Closing it, or giving up asking: nothing moves.
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
    /// A trade with a character opened, or changed by either side (the
    /// other's things come in `TradeStock` first).
    Between(Between),
    /// Waiting for `with` to Allow or Deny the trade asked for.
    Asking { with: Id },
    /// `by` (a player, `name`) asks to trade: answered with `TradeAsk::Answer`.
    Asked { by: Id, name: String },
    /// What is in the receive box: on joining and whenever it changes.
    Received(Received),
    /// The character's Trading setting: on joining and after `SetTrading`.
    Setting(TradingSetting),
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
    /// Walked out of reach, or left the level (or the other did, or went):
    /// nothing moved.
    OutOfReach,
    /// Both sides locked in: what each offered went into the other's
    /// receive box (an NPC's to the NPC).
    Exchanged,
    /// The one asked said no.
    Denied,
}
