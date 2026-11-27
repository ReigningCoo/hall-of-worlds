//! The middle and the bottom of the window: what each side is worth (its
//! items' values, `shared::value`) and what the shop pays or asks for it, who
//! gains by the trade and by how much, what is due at the bottom right, and
//! Complete trade, which only does anything with something in the trade that
//! can be paid for. Pressing it completes the trade: all of it or nothing.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, TradeAsk};
use shared::value::{copper_shown, total};

use super::state::TradeWindow;
use super::words::{coins_words, due_words};
use crate::common::Trading;
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue};
use crate::inventory::Carried;
use crate::widgets::UiId;

/// What the middle says, worked out from the trade.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Middle {
    pub(super) own: String,
    pub(super) theirs: String,
    pub(super) gain: String,
}

/// The middle's words: each side's worth and its price, and who gains (the
/// worth received less the worth given and what is paid).
pub(super) fn middle(trading: &Trading, carried: &Carried) -> Middle {
    let own_worth: u64 = (trading.selling.iter())
        .filter_map(|&id| carried.0.find(id))
        .map(total)
        .sum();
    let their_worth: u64 = trading.buying.iter().map(|(_, item)| total(item)).sum();
    let totals = trading.totals;
    let own = format!(
        "Yours: worth {}; they pay {}",
        coins_words(copper_shown(own_worth)),
        coins_words(totals.pays / shared::value::FINE_PER_COPPER),
    );
    let theirs = format!(
        "Theirs: worth {}; they ask {}",
        coins_words(copper_shown(their_worth)),
        coins_words(copper_shown(totals.asks)),
    );
    // In fine units: what comes in less what goes out, coins included.
    let paid = totals.due * shared::value::FINE_PER_COPPER as i64;
    let gain = their_worth as i64 - own_worth as i64 - paid;
    let gain = match gain {
        _ if !trading.is_busy() => String::new(),
        0 => "Neither side gains".to_owned(),
        gain if gain > 0 => format!("You gain {}", coins_words(copper_shown(gain as u64))),
        gain => format!(
            "They gain {}",
            coins_words(copper_shown(gain.unsigned_abs()))
        ),
    };
    Middle { own, theirs, gain }
}

/// Writes the middle and the bottom again when the trade or the window
/// changes.
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
    let Middle { own, theirs, gain } = middle(&trading, &carried);
    dom.set_text("trade/own_value", own);
    dom.set_text("trade/their_value", theirs);
    dom.set_flag("trade/gain", "hidden", gain.is_empty());
    dom.set_text("trade/gain", gain);
    let due = trading.totals.due;
    let words = match trading.with {
        Some(_) => due_words(due),
        None => String::new(),
    };
    dom.set_text("trade/due", words);
    let affordable = trading.totals.affordable(carried.0.coins_carried(), None);
    let can = trading.with.is_some() && trading.is_busy() && affordable;
    dom.set_flag("trade/complete", "disabled", !can);
    let said = window.said.clone().unwrap_or_default();
    dom.set_flag("trade/said", "hidden", said.is_empty());
    dom.set_text("trade/said", said);
}

/// Complete trade: the server completes it all or refuses.
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
