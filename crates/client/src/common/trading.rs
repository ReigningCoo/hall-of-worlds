//! The trade open, as the trade window (`trade`) last heard it from the
//! server, for the inventory (`inventory`) to draw and check by: what is
//! being sold (still carried, marked with a bag) and bought (still the shop's,
//! drawn unpaid where it is to go), and whether more can be afforded; or,
//! with a player or an NPC, both sides as the server last told them.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Item;
use shared::site::BuildingKind;
use shared::trade::{Between, Buy, Rates, Totals, TradingSetting, due, shop_asks};
use shared::value::total;

/// Stands for the trade window's own side, as somewhere to let go of what is
/// carried (`TakesGifts`): a stand-in, never a real id, numbered far past
/// any the inventory makes.
pub fn own_side() -> Id {
    Id::stand_in(u64::MAX - 1)
}

/// Stands for the trade window's receive box, as somewhere things are taken
/// from (`StockCell::with`): a stand-in like `own_side`.
pub fn receive_box() -> Id {
    Id::stand_in(u64::MAX - 2)
}

/// The character's Trading setting (Settings › Gameplay), as the server last
/// told it: `trade` keeps it, `system_menu` shows and changes it.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradeSetting(pub TradingSetting);

/// The trade open, if any: with a shop, a player or an NPC.
#[derive(Resource, Default, Debug)]
pub struct Trading {
    /// Its keeper, or the character traded with (or asked).
    pub with: Option<Id>,
    /// With a character: both sides as last told (`None` for a shop, or
    /// while asking).
    pub between: Option<Between>,
    /// Waiting for the one asked to Allow or Deny.
    pub asking: bool,
    /// The shop's rates for this character, and its kind, once told.
    pub terms: Option<(Rates, BuildingKind)>,
    /// What is being sold, still carried.
    pub selling: Vec<Id>,
    /// What is being bought, still the shop's, and where it is to go.
    pub buying: Vec<(Buy, Item)>,
    /// Its sums, as worked out from the above.
    pub totals: Totals,
}

impl Trading {
    /// Something being bought, by its id.
    pub fn unpaid(&self, item: Id) -> Option<&(Buy, Item)> {
        self.buying.iter().find(|(buy, _)| buy.item == item)
    }

    pub fn is_selling(&self, item: Id) -> bool {
        self.selling.contains(&item)
    }

    /// Whether anything is in it, either side: the window's dropdown stays
    /// as it is until both are clear.
    pub fn is_busy(&self) -> bool {
        let offered = (self.between.as_ref()).is_some_and(|between| {
            !between.mine.offers.is_empty() || !between.theirs.offers.is_empty()
        });
        !self.selling.is_empty() || !self.buying.is_empty() || offered
    }

    /// Whether buying `item` too would still be paid for by `coins` carried
    /// (coins + pending sales − pending buys, as `Totals::affordable`).
    pub fn affords(&self, item: &Item, coins: u64) -> bool {
        let Some((rates, _)) = self.terms else {
            return false;
        };
        let asks = self.totals.asks + shop_asks(total(item), rates);
        due(asks, self.totals.pays) <= coins as i64
    }
}
