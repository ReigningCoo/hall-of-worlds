//! The sums under the boxes and the Trade button (trading tweaks PR 3): with
//! a shop, under Selling what it pays and under Buying what it asks, and
//! under the Trade button the difference, always signed ("+" when the player
//! gains coins, "-" when they pay); with a character, each side's worth
//! (`shared::value`) and no difference. All as coin pictures, right-aligned,
//! any of none left out (`common::show_coins`). Pressing Trade completes the
//! trade: all of it or nothing, and only with something in it that can be
//! paid for.

use bevy::prelude::*;
use shared::item::Item;
use shared::protocol::{ClientMessage, TradeAsk};
use shared::trade::Between;
use shared::value::{FINE_PER_COPPER, copper_shown, total};

use super::state::TradeWindow;
use crate::common::{CoinPiece, Trading, coin_pieces, show_coins};
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::inventory::Carried;
use crate::widgets::UiId;

/// What the window's sums are, in copper as shown.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Sums {
    /// Under Selling: what the shop pays, or what the own side is worth.
    pub(super) own: u64,
    /// Under Buying: what the shop asks, or what their side is worth.
    pub(super) theirs: u64,
    /// Under Trade, with a shop and something in the trade: what the player
    /// gains in coins (negative: pays).
    pub(super) difference: Option<i64>,
}

/// The sums, worked out from the trade.
pub(super) fn sums(trading: &Trading, carried: &Carried, offered: &[Item]) -> Sums {
    if let Some(between) = &trading.between {
        return between_sums(between, carried, offered);
    }
    let totals = trading.totals;
    Sums {
        // As the server rounds them: what is paid down, what is asked up.
        own: totals.pays / FINE_PER_COPPER,
        theirs: copper_shown(totals.asks),
        difference: trading.is_busy().then_some(-totals.due),
    }
}

/// With a character: each side's worth; no coins change hands.
fn between_sums(between: &Between, carried: &Carried, offered: &[Item]) -> Sums {
    let worth = |side: &shared::trade::Side, find: &dyn Fn(shared::id::Id) -> Option<u64>| {
        let found = side.offers.iter().filter_map(|offer| find(offer.item));
        copper_shown(found.sum())
    };
    let own = worth(&between.mine, &|id| carried.0.find(id).map(total));
    let find_theirs = |id| offered.iter().find(|item| item.id == id).map(total);
    Sums {
        own,
        theirs: worth(&between.theirs, &find_theirs),
        difference: None,
    }
}

/// The difference as coins, always signed: "(+1 1 1)" gained, "(-1 1 1)"
/// paid, "(0)" even.
pub(super) fn difference_pieces(difference: i64) -> Vec<CoinPiece> {
    let words = |text: &str| CoinPiece::Words(text.to_owned());
    if difference == 0 {
        return vec![words("(0)")];
    }
    let sign = if difference > 0 { "(+" } else { "(-" };
    let mut pieces = vec![words(sign)];
    pieces.extend(coin_pieces(difference.unsigned_abs()));
    pieces.push(words(")"));
    pieces
}

/// Writes the sums again when the trade or the window changes, and enables
/// Trade only when it can be done.
pub(super) fn show_middle(
    trading: Res<Trading>,
    carried: Res<Carried>,
    mut window: ResMut<TradeWindow>,
    mut dom: Dom,
) {
    let changed = trading.is_changed() || carried.is_changed();
    if window.document.is_none() || !(changed || window.dirty) {
        return;
    }
    window.dirty = false;
    let Sums {
        own,
        theirs,
        difference,
    } = sums(&trading, &carried, &window.stock);
    // Only once trading with someone.
    let with = trading.with.is_some();
    let pieces = |copper| {
        if with {
            coin_pieces(copper)
        } else {
            Vec::new()
        }
    };
    show_coins(&mut dom, "trade", "own_total", 0, pieces(own));
    show_coins(&mut dom, "trade", "their_total", 100, pieces(theirs));
    let difference = difference.filter(|_| trading.terms.is_some());
    show_coins(
        &mut dom,
        "trade",
        "difference",
        200,
        difference.map(difference_pieces).unwrap_or_default(),
    );
    let affordable = trading.totals.affordable(carried.0.coins_carried(), None);
    let can = with && trading.is_busy() && affordable;
    dom.set_flag("trade/complete", "disabled", !can);
    let said = window.said.clone().unwrap_or_default();
    dom.set_flag("trade/said", "hidden", said.is_empty());
    dom.set_text("trade/said", said);
}

/// Trade: the server completes it all or refuses.
pub(super) fn press_complete(
    mut events: MessageReader<UiEvent>,
    trading: Res<Trading>,
    mut window: ResMut<TradeWindow>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        if event.id == UiId::new("trade", "complete")
            && event.value == UiValue::Pressed
            && trading.with.is_some()
        {
            window.said = None;
            window.dirty = true;
            to_server.write(ToServer(ClientMessage::Trade(TradeAsk::Complete)));
        }
    }
}
