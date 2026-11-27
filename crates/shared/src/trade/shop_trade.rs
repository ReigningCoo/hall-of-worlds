//! A trade with a shop as both sides hold it: what the player is buying
//! (still the shop's until completed, with where it is to go) and selling
//! (still carried), and what they offer to pay. Its totals are worked out
//! the same way on both sides.

use serde::{Deserialize, Serialize};

use super::{Rates, due, normally_sold, shop_asks, shop_pays};
use crate::id::Id;
use crate::item::{Holder, Inventory, Item, Rotation};
use crate::site::BuildingKind;
use crate::value::total;

/// A trade with the shop whose keeper is `with`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopTrade {
    pub with: Id,
    /// What is being bought from its stock, in the order asked.
    pub buying: Vec<Buy>,
    /// What is being sold, still carried, in the order asked.
    pub selling: Vec<Id>,
    /// What the player offers to pay in copper (negative: to be paid), if
    /// not what is due: a tip or a haggle.
    pub paying: Option<i64>,
}

/// Something from a shop's stock, and the place in the player's inventory
/// it goes to (anywhere it fits, if that is taken by then).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buy {
    pub item: Id,
    pub to: Holder,
    pub x: u8,
    pub y: u8,
    pub rotation: Rotation,
}

/// Why something in a trade was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refusal {
    /// The shop no longer has it.
    NotStocked,
    /// The player no longer carries it.
    NotCarried,
    /// It can't be sold: coins, a quest item, something holding either, or
    /// something that won't come off.
    NotForSale,
    /// It is in the trade already.
    AlreadyIn,
    /// Offering less than the haggle allows.
    TooShort,
    /// Not carrying the coins.
    CantAfford,
    /// No room for what is bought, or for the coins.
    NoRoom,
    /// Too far from the shop, or no trade is open with it.
    NotHere,
    /// Nothing to complete: nothing bought or sold, and nothing given.
    Nothing,
}

/// A trade's sums: what the shop asks and pays in fine units, and what the
/// player pays in copper (`due`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Totals {
    pub asks: u64,
    pub pays: u64,
    pub due: i64,
}

impl Totals {
    /// Whether this many copper carried pays for it (coins + pending sales −
    /// pending buys).
    pub fn affordable(&self, coins: u64, paying: Option<i64>) -> bool {
        paying.unwrap_or(self.due) <= coins as i64
    }
}

impl ShopTrade {
    /// A new trade, with nothing in it.
    pub fn new(with: Id) -> Self {
        Self {
            with,
            buying: Vec::new(),
            selling: Vec::new(),
            paying: None,
        }
    }

    /// Whether `item` is in it, either side.
    pub fn has(&self, item: Id) -> bool {
        self.selling.contains(&item) || self.buying.iter().any(|buy| buy.item == item)
    }

    /// Its sums, with the shop's `stock` and what the player carries: refused
    /// if anything in it has gone or can't be sold. Something sold inside
    /// something else sold is refused (it went with it).
    pub fn totals(
        &self,
        stock: &[Item],
        carried: &Inventory,
        rates: Rates,
        shop: BuildingKind,
    ) -> Result<Totals, Refusal> {
        let mut asks = 0;
        for buy in &self.buying {
            let item = stock.iter().find(|item| item.id == buy.item);
            asks += shop_asks(total(item.ok_or(Refusal::NotStocked)?), rates);
        }
        let mut trial = carried.clone();
        let mut pays = 0;
        for &id in &self.selling {
            let item = trial.find(id).ok_or(Refusal::NotCarried)?;
            if !for_sale(item) {
                return Err(Refusal::NotForSale);
            }
            let sold = normally_sold(shop, item.kind);
            pays += shop_pays(total(item), rates, sold);
            trial.hand_over(id).ok_or(Refusal::NotForSale)?;
        }
        Ok(Totals {
            asks,
            pays,
            due: due(asks, pays),
        })
    }
}

/// Whether a shop takes it: never coins (they pay), quest items (the
/// character's own) or anything holding either.
pub fn for_sale(item: &Item) -> bool {
    fn holds_coins(item: &Item) -> bool {
        item.kind.coin_value().is_some()
            || (item.contents.iter()).any(|placed| holds_coins(&placed.item))
            || item.above.iter().any(holds_coins)
    }
    !holds_coins(item) && !item.holds_quest_item()
}
