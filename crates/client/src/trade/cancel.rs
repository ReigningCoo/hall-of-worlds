//! Cancel, under the Trade button (final trade tweaks, owner): empties the
//! Selling and Buying boxes, nothing changing hands, by the asks that take
//! each thing out (`Unsell`, `Unbuy`). With a character it takes back the own
//! side and unlocks it.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, TradeAsk};

use super::state::TradeWindow;
use crate::common::Trading;
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::widgets::UiId;

/// The asks that empty the own side of the trade (and with a character
/// unlock it): none if there is nothing to cancel.
pub(super) fn cancelling(trading: &Trading) -> Vec<TradeAsk> {
    let mut asks: Vec<TradeAsk> = Vec::new();
    match &trading.between {
        Some(between) => {
            let offered = between.mine.offers.iter();
            asks.extend(offered.map(|offer| TradeAsk::Unsell { item: offer.item }));
            if between.mine.locked || !asks.is_empty() {
                asks.push(TradeAsk::Lock(false));
            }
        }
        None if trading.with.is_some() => {
            let sold = trading.selling.iter();
            asks.extend(sold.map(|&item| TradeAsk::Unsell { item }));
            let bought = trading.buying.iter();
            asks.extend(bought.map(|(offer, _)| TradeAsk::Unbuy { item: offer.item }));
        }
        None => {}
    }
    asks
}

/// Cancel pressed: everything taken out of the trade.
pub(super) fn press_cancel(
    mut events: MessageReader<UiEvent>,
    trading: Res<Trading>,
    mut window: ResMut<TradeWindow>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        if event.id != UiId::new("trade", "cancel") || event.value != UiValue::Pressed {
            continue;
        }
        window.said = None;
        window.dirty = true;
        for ask in cancelling(&trading) {
            to_server.write(ToServer(ClientMessage::Trade(ask)));
        }
    }
}

/// Cancel is enabled only with something to cancel.
pub(super) fn show_cancel(trading: Res<Trading>, window: Res<TradeWindow>, mut dom: Dom) {
    if window.document.is_none() || !(trading.is_changed() || window.is_changed()) {
        return;
    }
    dom.set_flag("trade/cancel", "disabled", cancelling(&trading).is_empty());
}
