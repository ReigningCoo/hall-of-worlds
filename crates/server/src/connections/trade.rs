//! Trading (trading plan PRs 3 and 5, `shared::trade`). With a shop: the
//! trade a client has open, changed by `TradeAsk`s and settled all at once on
//! Complete. With a player or NPC (`deals.rs`): asked for by the other's
//! Trading setting, each side changed and locked in, settled into each
//! other's receive box once both are. Until then nothing moves: what is
//! bought stays the shop's and what is sold or offered stays carried, so
//! closing, walking out of reach, leaving the level or going simply forgets
//! the trade. While anything waits in the receive box (`received.rs`), no
//! trade can start.

use bevy::prelude::*;
use shared::id::Id;
use shared::poi::ChunkPos;
use shared::protocol::{ServerMessage, TradeAsk, TradeEnd, TradeNews};
use shared::site::BuildingKind;
use shared::skill::SkillId;
use shared::trade::{Rates, Refusal, ShopTrade, TradingSetting};

use super::Client;
use crate::ServerWorld;
use crate::sites::{ShopTerms, shop_terms};

mod change;
mod deal_change;
mod deal_open;
mod deal_settle;
mod deal_tell;
mod deals;
mod received;
mod settle;
mod trading;

pub(super) use deals::do_deals;
pub(super) use received::{set_setting, told_on_joining};
pub(crate) use trading::Trading;
use trading::{DealAsk, DealState, Dealing};

/// The Trading skill's path.
const TRADING: &str = "social/trading";

/// Opens a trade with `with`: a shop's keeper straight away, a player or NPC
/// once every client's messages have been read (`deals.rs`). Refused while
/// anything waits in the receive box. `false` if the client has gone.
pub(super) fn open(client: &mut Client, world: &mut ServerWorld, with: Id) -> bool {
    if !client.trade.may_start() {
        debug!(
            "{} refused a trade: their receive box isn't empty",
            client.in_log()
        );
        return tell(client, TradeNews::Refused(Refusal::Unclaimed));
    }
    if crate::sites::keeper_building(client, world, with).is_none() {
        client.trade.asked.push(DealAsk::Open(with));
        return true;
    }
    // A shop's trade closes any with a character (`deals` tells the other).
    if let Some(dealing) = client.trade.dealing.take()
        && !end(client, dealing.with, TradeEnd::Closed)
    {
        return false;
    }
    crate::sites::open_trade(client, world, with)
}

/// A trade was opened with the shop `with` keeps, in `town`: any other open
/// one is closed, and its rates for this character are told. `false` if the
/// client has gone.
pub(crate) fn trade_opened(
    client: &mut Client,
    with: Id,
    town: ChunkPos,
    shop: BuildingKind,
) -> bool {
    if let Some(old) = client.trade.shop.take().filter(|old| old.with != with)
        && !end(client, old.with, TradeEnd::Closed)
    {
        return false;
    }
    client.trade.shop = Some(ShopTrade::new(with));
    let rates = rates(client, town);
    tell(client, TradeNews::Opened { with, rates, shop })
        && tell(client, TradeNews::State(ShopTrade::new(with)))
}

/// Answers a change to the open trade, completing or closing it; taking from
/// the receive box needs none open. A trade with a character is answered
/// once messages are read (`deals.rs`). `false` if the client has gone.
pub(super) fn answer(client: &mut Client, world: &mut ServerWorld, ask: TradeAsk) -> bool {
    match ask {
        TradeAsk::Take(to) => return received::take(client, world, Some(to)),
        TradeAsk::TakeAll => return received::take(client, world, None),
        _ => {}
    }
    let for_dealing = matches!(
        ask,
        TradeAsk::Offer(_) | TradeAsk::Lock(_) | TradeAsk::Answer { .. }
    );
    if client.trade.dealing.is_some() || for_dealing {
        client.trade.asked.push(DealAsk::Change(ask));
        return true;
    }
    let Some(trade) = client.trade.shop.clone() else {
        debug!("Refused {ask:?} with no trade open");
        return tell(client, TradeNews::Refused(Refusal::NotHere));
    };
    let terms = shop_terms(client, world, trade.with).filter(|terms| terms.in_reach);
    let Some(terms) = terms else {
        return end(client, trade.with, TradeEnd::OutOfReach);
    };
    match ask {
        TradeAsk::Close => end(client, trade.with, TradeEnd::Closed),
        TradeAsk::Complete => settle::complete(client, world, trade, &terms),
        ask => match change::changed(client, &trade, &terms, ask) {
            Ok(trade) => {
                client.trade.shop = Some(trade.clone());
                tell(client, TradeNews::State(trade))
            }
            Err(why) => {
                debug!("Refused a trade change: {why:?}");
                tell(client, TradeNews::Refused(why)) && tell(client, TradeNews::State(trade))
            }
        },
    }
}

/// Closes the open shop trade if the player is no longer within reach of
/// its shop (walked away, another level, another world). `false` if the
/// client has gone.
pub(super) fn keep_in_reach(client: &mut Client, world: &mut ServerWorld) -> bool {
    let Some(with) = client.trade.shop.as_ref().map(|trade| trade.with) else {
        return true;
    };
    crate::sites::near_shop(client, world, with) || end(client, with, TradeEnd::OutOfReach)
}

impl Client {
    /// Whether the character is set never to trade, which everyone who sees
    /// them is told (`EntityLook::never_trades`).
    pub(crate) fn never_trades(&self) -> bool {
        self.trade.setting() == TradingSetting::Never
    }
}

/// The rates a shop of `town` gives this character, by their standing there
/// and their Trading.
fn rates(client: &Client, town: ChunkPos) -> Rates {
    let standing = crate::quests::standing(&client.character.quests, town);
    Rates::for_shop(standing, trading(client))
}

/// The character's Trading, as a whole percentage.
fn trading(client: &Client) -> u32 {
    SkillId::find(TRADING).map_or(0, |skill| client.sheet.skills.percent(skill))
}

/// A shop's rates for this character, as `rates` gives them.
fn terms_rates(client: &Client, terms: &ShopTerms) -> Rates {
    rates(client, terms.town)
}

/// Forgets the open trade with `with` (a shop's or a character's), telling
/// the client why. Nothing moves.
fn end(client: &mut Client, with: Id, why: TradeEnd) -> bool {
    if client
        .trade
        .shop
        .as_ref()
        .is_some_and(|shop| shop.with == with)
    {
        client.trade.shop = None;
    }
    if client
        .trade
        .dealing
        .as_ref()
        .is_some_and(|deal| deal.with == with)
    {
        client.trade.dealing = None;
    }
    tell(client, TradeNews::Ended { with, why })
}

/// Tells the client about its trade. `false` if it has gone.
fn tell(client: &mut Client, news: TradeNews) -> bool {
    client.link.send(&ServerMessage::Trade(news))
}

#[cfg(test)]
mod deal_tests;
#[cfg(test)]
mod tests;
