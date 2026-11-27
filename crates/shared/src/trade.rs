//! Trading (trading plan PR 3, `docs/context/trading.md`): the rules every
//! trade goes by, so the client can say beforehand what the server will do.
//!
//! - Nothing moves until a trade is completed: what is offered stays where it
//!   is, and closing, walking out of reach, leaving the level or going resets
//!   everything (owner).
//! - Shops buy at 50% of an item's value (`shared::value`) and sell at 150%,
//!   town standing and the Trading skill each moving both by up to 25%
//!   (`Rates`); what a shop doesn't normally sell it pays a further 50% less
//!   for (`price.rs`).
//! - Coins are paid and change given automatically on completing, highest
//!   coins first, over the carry limit if need be (`item::inventory::pay`).
//!   Paying more than is due is a tip, less is haggling (`settle.rs`).
//! - Trades between characters (`between.rs`): each side up to 8 × 8, settled
//!   once both are locked in, into the other's receive box (`received.rs`).

mod between;
mod complete;
mod price;
mod rates;
mod received;
mod settle;
mod shop_trade;

pub use between::{Between, Offer, SIDE_CELLS, Side, for_trade};
pub use complete::{Completed, complete};
pub use price::{NOT_SOLD_PERCENT, normally_sold, shop_asks, shop_pays};
pub use rates::{MOST_SKILL, MOST_STANDING, Rates, skill_percent, standing_percent};
pub use received::{Received, TradingSetting};
pub use settle::{HAGGLE_PERCENT, Payment, TIP_PERCENT, TRADE_PRACTICE, due, haggle_room, judge};
pub use shop_trade::{Buy, Refusal, ShopTrade, Totals, for_sale};

use crate::spot::{CELL_SPAN, Spot};

/// How far apart, in cells, two sides of a trade may be: a shop's door
/// counts as where it is.
pub const REACH_CELLS: u64 = 5;

/// The same in units (`spot`).
pub const REACH: u64 = REACH_CELLS * CELL_SPAN as u64;

/// Whether someone at `at` is near enough to the door in cell `door` to
/// trade with its shop. Both sides go by it.
pub fn shop_in_reach(at: Spot, door: (i64, i64)) -> bool {
    at.distance(Spot::middle_of(door)) <= REACH
}

/// Whether two characters at `a` and `b` are near enough to trade.
pub fn in_reach(a: Spot, b: Spot) -> bool {
    a.distance(b) <= REACH
}

#[cfg(test)]
mod between_tests;
#[cfg(test)]
mod tests;
