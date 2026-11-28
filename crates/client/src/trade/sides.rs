//! How the window is laid out for what it is doing (trading plan PR 6): a
//! shop's stock and Complete trade; a character's side (seen, not touched)
//! with the Trade toggle, "Asking..." over it while the one asked hasn't
//! answered; or, once a trade has settled, only the receive box with Take
//! all. A side locked in is drawn darker with a solid edge.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, TradeAsk};

use super::between::placed;
use super::markup::OWN_COLUMNS;
use super::state::TradeWindow;
use crate::common::Trading;
use crate::connection::ToServer;
use crate::dom::{Dom, GridShown, GridThing, UiEvent, UiValue};
use crate::widgets::UiId;

/// What the window is showing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Mode {
    /// A shop's stock, or nobody yet.
    Shop,
    /// A trade with a character, or asking for one.
    Between,
    /// The receive box, until it is empty.
    Receiving,
}

pub(super) fn mode(window: &TradeWindow, trading: &Trading) -> Mode {
    if window.receiving() {
        Mode::Receiving
    } else if trading.between.is_some() || trading.asking {
        Mode::Between
    } else {
        Mode::Shop
    }
}

/// What the parts last showed: the mode, locked in either side, and asking.
type Shown = (Mode, bool, bool, bool);

/// Shows and hides the window's parts for its mode, and the Trade toggle.
pub(super) fn show_sides(
    window: Res<TradeWindow>,
    trading: Res<Trading>,
    mut dom: Dom,
    mut shown: Local<Option<Shown>>,
) {
    if window.document.is_none() {
        *shown = None;
        return;
    }
    let mode = mode(&window, &trading);
    let between = trading.between.as_ref();
    let mine = between.is_some_and(|between| between.mine.locked);
    let theirs = between.is_some_and(|between| between.theirs.locked);
    let now = (mode, mine, theirs, trading.asking);
    if shown.as_ref() == Some(&now) {
        return;
    }
    let receiving = mode == Mode::Receiving;
    let with_character = mode == Mode::Between;
    dom.set_flag("trade/trading", "hidden", receiving);
    dom.set_flag("trade/received_frame", "hidden", !receiving);
    dom.set_flag("trade/bottom", "hidden", receiving || with_character);
    dom.set_flag("trade/scroll", "hidden", with_character);
    dom.set_flag("trade/buying_label", "hidden", with_character);
    dom.set_flag("trade/buying", "hidden", with_character);
    dom.set_flag("trade/theirs", "hidden", !with_character);
    dom.set_flag("trade/lock", "hidden", !with_character);
    dom.set_flag("trade/lock", "disabled", between.is_none());
    dom.set_flag("trade/lock", "selected", mine);
    dom.set_flag("trade/their_lock", "hidden", !(with_character && theirs));
    dom.set_flag("trade/asking", "hidden", !trading.asking);
    *shown = Some(now);
}

/// Draws a character's side where they put things, darkened once they are
/// locked in or while they are being asked; nothing there can be taken.
pub(super) fn draw_their_side(
    window: Res<TradeWindow>,
    trading: Res<Trading>,
    mut commands: Commands,
    mut drawn: Local<Option<(Entity, GridShown)>>,
) {
    let Some(holder) = window.between_holder else {
        *drawn = None;
        return;
    };
    let between = trading.between.as_ref();
    let things = between.map_or_else(Vec::new, |between| {
        let placed = placed(&between.theirs, &window.stock);
        (placed.into_iter())
            .map(|placed| GridThing {
                item: placed.item,
                x: placed.x,
                y: placed.y,
                rotation: placed.rotation,
                dimmed: false,
                mark: Default::default(),
            })
            .collect()
    });
    let locked = trading.asking || between.is_some_and(|between| between.theirs.locked);
    let now = GridShown {
        columns: OWN_COLUMNS,
        rows: OWN_COLUMNS,
        things,
        locked,
        ..default()
    };
    if drawn.as_ref() == Some(&(holder, now.clone())) {
        return;
    }
    commands.entity(holder).try_insert(now.clone());
    *drawn = Some((holder, now));
}

/// The Trade toggle locks the own side in, or out again; Take all empties
/// the receive box as far as there is room.
pub(super) fn press_lock(
    mut events: MessageReader<UiEvent>,
    trading: Res<Trading>,
    mut window: ResMut<TradeWindow>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        if event.value != UiValue::Pressed {
            continue;
        }
        let ask = if event.id == UiId::new("trade", "lock") {
            let Some(between) = &trading.between else {
                continue;
            };
            TradeAsk::Lock(!between.mine.locked)
        } else if event.id == UiId::new("trade", "take_all") {
            TradeAsk::TakeAll
        } else {
            continue;
        };
        window.said = None;
        window.dirty = true;
        to_server.write(ToServer(ClientMessage::Trade(ask)));
    }
}
