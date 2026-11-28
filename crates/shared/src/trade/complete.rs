//! Completing a shop trade: everything at once or nothing (owner). Worked
//! out on copies, which the server keeps only if all of it could be done.

use super::{Payment, Rates, Refusal, ShopTrade, Totals, judge};
use crate::id::Id;
use crate::item::{Inventory, Item, MadeAt, PayRefused, StackLimits};
use crate::site::BuildingKind;

/// A trade done: what the player now carries and the shop has, what bought
/// didn't fit and goes into the receive box, what was paid (copper, negative
/// when the shop paid) and how.
#[derive(Clone, Debug)]
pub struct Completed {
    pub inventory: Inventory,
    pub received: Vec<Item>,
    pub stock: Vec<Item>,
    pub totals: Totals,
    pub paid: i64,
    pub payment: Payment,
}

/// What a shop and its customer would have after `trade`, with this much
/// Trading (a whole percentage): sold things join the stock, coins are paid
/// highest first with change given, the shop's coins over the carry limit if
/// need be, and what is bought goes wherever it fits, in the order bought,
/// the rest into the receive box (`Completed::received`).
#[allow(clippy::too_many_arguments)]
pub fn complete(
    trade: &ShopTrade,
    stock: &[Item],
    carried: &Inventory,
    (rates, shop): (Rates, BuildingKind),
    trading: u32,
    limits: StackLimits,
    made_at: Option<MadeAt>,
    new_id: &mut impl FnMut() -> Id,
) -> Result<Completed, Refusal> {
    let totals = trade.totals(stock, carried, rates, shop)?;
    let (paid, payment) = judge(totals.due, trade.paying, trading)?;
    if trade.buying.is_empty() && trade.selling.is_empty() && paid <= 0 {
        return Err(Refusal::Nothing);
    }
    let mut inventory = carried.clone();
    let mut stock = stock.to_vec();
    for &id in &trade.selling {
        stock.push(inventory.hand_over(id).ok_or(Refusal::NotCarried)?);
    }
    if paid > 0 {
        let paying = inventory.pay_coins(paid.unsigned_abs(), limits, made_at, new_id);
        paying.map_err(|refused| match refused {
            PayRefused::NotEnough => Refusal::CantAfford,
            PayRefused::NoRoom => Refusal::NoRoom,
        })?;
    }
    if paid < 0 {
        let receiving = inventory.receive_coins(paid.unsigned_abs(), limits, made_at, new_id);
        receiving.map_err(|_| Refusal::NoRoom)?;
    }
    let mut received = Vec::new();
    for buy in &trade.buying {
        let place = stock.iter().position(|item| item.id == buy.item);
        let item = stock.remove(place.ok_or(Refusal::NotStocked)?);
        if let Err(item) = inventory.give(item, limits, new_id) {
            received.push(item);
        }
    }
    Ok(Completed {
        inventory,
        received,
        stock,
        totals,
        paid,
        payment,
    })
}
