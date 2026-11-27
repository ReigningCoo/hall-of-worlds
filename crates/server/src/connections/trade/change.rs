//! Changing an open trade: something to buy or sell added or taken back, or
//! what to pay. Each change is checked as it comes, so the trade the client
//! is told of is one that could be completed now.

use shared::item::Item;
use shared::protocol::TradeAsk;
use shared::trade::{Refusal, ShopTrade, for_sale};

use super::terms_rates;
use crate::connections::Client;
use crate::sites::ShopTerms;

/// The trade after `ask`, or why not.
pub(super) fn changed(
    client: &mut Client,
    trade: &ShopTrade,
    terms: &ShopTerms,
    ask: TradeAsk,
) -> Result<ShopTrade, Refusal> {
    let mut trade = trade.clone();
    match ask {
        TradeAsk::Buy(buy) => {
            if trade.has(buy.item) {
                return Err(Refusal::AlreadyIn);
            }
            trade.buying.push(buy);
            fits(client, &trade, &terms.stock)?;
        }
        TradeAsk::Unbuy { item } => trade.buying.retain(|buy| buy.item != item),
        TradeAsk::Sell { item } => {
            if trade.has(item) {
                return Err(Refusal::AlreadyIn);
            }
            let found = client.inventory.find(item).ok_or(Refusal::NotCarried)?;
            if !for_sale(found) || !client.may_take_off(item) {
                return Err(Refusal::NotForSale);
            }
            trade.selling.push(item);
        }
        TradeAsk::Unsell { item } => trade.selling.retain(|&one| one != item),
        TradeAsk::Pay(paying) => trade.paying = paying,
        TradeAsk::Complete | TradeAsk::Close => {}
    }
    let rates = terms_rates(client, terms);
    let totals = trade.totals(&terms.stock, &client.inventory, rates, terms.kind)?;
    // Nothing bought that can't be paid for (coins + sales − buys).
    let coins = client.inventory.coins_carried();
    if !trade.buying.is_empty() && !totals.affordable(coins, None) {
        return Err(Refusal::CantAfford);
    }
    Ok(trade)
}

/// Whether everything being bought fits where it is to go, beside what is
/// carried and each other.
fn fits(client: &Client, trade: &ShopTrade, stock: &[Item]) -> Result<(), Refusal> {
    let mut trial = client.inventory.clone();
    for buy in &trade.buying {
        let item = stock.iter().find(|item| item.id == buy.item);
        let item = item.ok_or(Refusal::NotStocked)?.clone();
        (trial.add(item, buy.to, buy.x, buy.y, buy.rotation)).map_err(|_| Refusal::NoRoom)?;
    }
    Ok(())
}
