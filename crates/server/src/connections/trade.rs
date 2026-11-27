//! Trading with a shop (trading plan PR 3, `shared::trade`): the trade a
//! client has open, changed by `TradeAsk`s and settled all at once on
//! Complete. Until then nothing moves: what is bought stays the shop's and
//! what is sold stays carried, so closing, walking out of reach, leaving the
//! level or going simply forgets the trade.

use bevy::prelude::*;
use shared::id::Id;
use shared::poi::ChunkPos;
use shared::protocol::{ServerMessage, TradeAsk, TradeEnd, TradeNews};
use shared::site::BuildingKind;
use shared::skill::SkillId;
use shared::trade::{Rates, Refusal, ShopTrade};

use super::Client;
use crate::ServerWorld;
use crate::sites::{ShopTerms, shop_terms};

mod change;
mod settle;

/// The Trading skill's path.
const TRADING: &str = "social/trading";

/// A trade was opened with the shop `with` keeps, in `town`: any other open
/// one is closed, and its rates for this character are told. `false` if the
/// client has gone.
pub(crate) fn trade_opened(
    client: &mut Client,
    with: Id,
    town: ChunkPos,
    shop: BuildingKind,
) -> bool {
    if let Some(old) = client.trade.take().filter(|old| old.with != with)
        && !tell(
            client,
            TradeNews::Ended {
                with: old.with,
                why: TradeEnd::Closed,
            },
        )
    {
        return false;
    }
    client.trade = Some(ShopTrade::new(with));
    let rates = rates(client, town);
    tell(client, TradeNews::Opened { with, rates, shop })
        && tell(client, TradeNews::State(ShopTrade::new(with)))
}

/// Answers a change to the open trade, completing or closing it. `false` if
/// the client has gone.
pub(super) fn answer(client: &mut Client, world: &mut ServerWorld, ask: TradeAsk) -> bool {
    let Some(trade) = client.trade.clone() else {
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
                client.trade = Some(trade.clone());
                tell(client, TradeNews::State(trade))
            }
            Err(why) => {
                debug!("Refused a trade change: {why:?}");
                tell(client, TradeNews::Refused(why)) && tell(client, TradeNews::State(trade))
            }
        },
    }
}

/// Closes the open trade if the player is no longer within reach of its
/// shop (walked away, another level, another world). `false` if the client
/// has gone.
pub(super) fn keep_in_reach(client: &mut Client, world: &mut ServerWorld) -> bool {
    let Some(with) = client.trade.as_ref().map(|trade| trade.with) else {
        return true;
    };
    crate::sites::near_shop(client, world, with) || end(client, with, TradeEnd::OutOfReach)
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

/// Forgets the open trade, telling the client why. Nothing moves.
fn end(client: &mut Client, with: Id, why: TradeEnd) -> bool {
    client.trade = None;
    tell(client, TradeNews::Ended { with, why })
}

/// Tells the client about its trade. `false` if it has gone.
fn tell(client: &mut Client, news: TradeNews) -> bool {
    client.link.send(&ServerMessage::Trade(news))
}

#[cfg(test)]
mod tests;
