//! What the server says of trades: who keeps each shop, a shop's stock (which
//! opens the window on it, as a click on its door asks), the trade opened, as
//! it stands after each change, refused, or ended; trades with characters,
//! the receive box and the setting are `between`'s. The trade is kept in
//! `common::Trading`, its sums worked out as the server works them.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, TradeEnd, TradeNews};
use shared::trade::ShopTrade;

use super::between::{follow_between, hear_between};
use super::open::open_window;
use super::state::TradeWindow;
use super::words::{ended_words, refusal_words};
use crate::common::{TradeSetting, Trading};
use crate::connection::{FromServer, ToServer};
use crate::inventory::{Carried, OpenCharacter};

#[allow(clippy::too_many_arguments)]
pub(super) fn read_server(
    mut from_server: MessageReader<FromServer>,
    mut window: ResMut<TradeWindow>,
    mut trading: ResMut<Trading>,
    mut setting: ResMut<TradeSetting>,
    carried: Res<Carried>,
    mut inventory: MessageWriter<OpenCharacter>,
    mut to_server: MessageWriter<ToServer>,
    mut commands: Commands,
) {
    for FromServer(message) in from_server.read() {
        match message {
            // (A welcome may be from another server, after logging out.)
            ServerMessage::Welcome { .. }
            | ServerMessage::EnterLevel(_)
            | ServerMessage::EnterHall { .. } => {
                window.keepers.clear();
                window.forget_trade();
                window.asked = None;
                *trading = Trading::default();
            }
            ServerMessage::Keepers(list) => {
                window.keepers = list.iter().map(|one| (one.building, one.clone())).collect();
            }
            ServerMessage::TradeStock { with, items } => {
                window.stock.clone_from(items);
                window.their_grid(*with);
                if window.document.is_none() {
                    // A shop's door was clicked: the window opens on it, with
                    // the inventory to drag into.
                    open_window(&mut window, &mut commands);
                    window.just_opened = false;
                    inventory.write(OpenCharacter);
                }
                follow(&mut window, &mut trading, &carried);
            }
            ServerMessage::Trade(news) => {
                if hear_between(news, &mut window, &mut trading, &mut setting) {
                    follow(&mut window, &mut trading, &carried);
                    continue;
                }
                hear(news, &mut window, &mut trading, &carried);
                // Completed, the window stays on the same shop for more.
                if let TradeNews::Ended {
                    with,
                    why: TradeEnd::Completed { .. },
                } = news
                    && window.document.is_some()
                {
                    let with = *with;
                    to_server.write(ToServer(ClientMessage::OpenTrade { with }));
                }
            }
            _ => {}
        }
    }
}

/// One piece of a trade's news.
fn hear(news: &TradeNews, window: &mut TradeWindow, trading: &mut Trading, carried: &Carried) {
    match news {
        TradeNews::Opened { with, rates, shop } => {
            *trading = Trading {
                with: Some(*with),
                terms: Some((*rates, *shop)),
                ..Trading::default()
            };
        }
        TradeNews::State(trade) if Some(trade.with) == trading.with => {
            take_state(trade, window, trading);
            // A change under way: what was said before has gone, but not a
            // refusal, which comes just before the state as it was.
            let refused = std::mem::take(&mut window.just_refused);
            if !refused && trading.is_busy() {
                window.said = None;
            }
            follow(window, trading, carried);
        }
        TradeNews::State(_) => {}
        TradeNews::Refused(why) => {
            window.said = Some(refusal_words(*why).to_owned());
            window.just_refused = true;
            window.dirty = true;
        }
        TradeNews::Ended { with, why } => {
            // Whoever asked gave up, or went: the requester goes too.
            if window.asked.as_ref().is_some_and(|(by, _)| by == with) {
                window.asked = None;
            }
            if Some(*with) == trading.with {
                *trading = Trading::default();
                window.forget_trade();
                window.said = ended_words(*why);
            }
        }
        // Trades with characters: `between`'s.
        _ => {}
    }
}

/// The trade as the server holds it: what is bought, with the shop's items.
fn take_state(trade: &ShopTrade, window: &TradeWindow, trading: &mut Trading) {
    trading.selling.clone_from(&trade.selling);
    trading.buying = (trade.buying.iter())
        .filter_map(|buy| {
            let item = window.stock.iter().find(|item| item.id == buy.item)?;
            Some((*buy, item.clone()))
        })
        .collect();
}

/// Works out the trade's sums again, and what each side shows: the shop's
/// stock but what is being bought, and what is being sold as it is carried.
/// Also when what is carried changes.
pub(super) fn follow(window: &mut TradeWindow, trading: &mut Trading, carried: &Carried) {
    if let Some(between) = &trading.between {
        follow_between(window, between, carried);
        window.dirty = true;
        return;
    }
    if let (Some(with), Some((rates, shop))) = (trading.with, trading.terms) {
        let trade = ShopTrade {
            with,
            buying: trading.buying.iter().map(|(buy, _)| *buy).collect(),
            selling: trading.selling.clone(),
            paying: None,
        };
        // Refused only while something has gone: the server says so too.
        let totals = trade.totals(&window.stock, &carried.0, rates, shop);
        trading.totals = totals.unwrap_or_default();
        let theirs: Vec<_> = (window.stock.iter())
            .filter(|item| trading.unpaid(item.id).is_none())
            .cloned()
            .collect();
        window.their_grid(with).heard(&theirs);
    }
    let selling: Vec<_> = (trading.selling.iter())
        .filter_map(|&id| carried.0.find(id).cloned())
        .collect();
    window.own.heard(&selling);
    window.dirty = true;
}

/// What is carried changed: the own side and the sums follow it.
pub(super) fn follow_carried(
    carried: Res<Carried>,
    mut window: ResMut<TradeWindow>,
    mut trading: ResMut<Trading>,
) {
    if carried.is_changed() && trading.with.is_some() {
        follow(&mut window, &mut trading, &carried);
    }
}
