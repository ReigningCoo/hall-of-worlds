//! Opening and closing the trade window: its key (T) or its button opens it or
//! closes it, and so does its close button; a click on a shop's door opens it
//! on that shop (`server`); leaving the game closes it. Closing it closes the
//! trade: nothing moves (owner).

use bevy::prelude::*;
use shared::protocol::{ClientMessage, TradeAsk};

use super::button::TradeButton;
use super::markup::TRADE;
use super::state::TradeWindow;
use crate::common::{Action, GameView, Preferences, Session, Trading};
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue, document, top_strip};
use crate::inventory::OpenCharacter;
use crate::widgets::{ButtonPressed, UiId};

/// Spawns the window's document.
pub(super) fn open_window(window: &mut TradeWindow, commands: &mut Commands) {
    let document = commands.spawn(document(&TRADE)).insert(top_strip()).id();
    window.document = Some(document);
    // Its grids and words are told again once built.
    window.their_holder = None;
    window.own_holder = None;
    window.shops.clear();
    window.just_opened = true;
    window.dirty = true;
}

/// Despawns it, closing any trade open.
fn close_window(
    window: &mut TradeWindow,
    trading: &mut Trading,
    to_server: &mut MessageWriter<ToServer>,
    commands: &mut Commands,
) {
    if let Some(document) = window.document.take() {
        // (It may have gone with its panel already.)
        commands.entity(document).try_despawn();
    }
    // Its grids went with it.
    window.their_holder = None;
    window.own_holder = None;
    if trading.with.is_some() {
        to_server.write(ToServer(ClientMessage::Trade(TradeAsk::Close)));
    }
    *trading = Trading::default();
    window.forget_trade();
    window.said = None;
}

#[allow(clippy::too_many_arguments)]
pub(super) fn toggle_trade(
    mut pressed: MessageReader<ButtonPressed>,
    button: Query<(), With<TradeButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    preferences: Res<Preferences>,
    view: Res<State<GameView>>,
    (mut window, mut trading): (ResMut<TradeWindow>, ResMut<Trading>),
    mut to_server: MessageWriter<ToServer>,
    mut inventory: MessageWriter<OpenCharacter>,
    mut commands: Commands,
) {
    let by_button = pressed.read().any(|press| button.contains(press.0));
    let by_key = preferences.controls.just_pressed(&keys, Action::Trade);
    if *view.get() == GameView::Connecting || !(by_button || by_key) {
        return;
    }
    if window.document.is_some() {
        close_window(&mut window, &mut trading, &mut to_server, &mut commands);
        return;
    }
    open_window(&mut window, &mut commands);
    // What is bought is dragged into the inventory.
    inventory.write(OpenCharacter);
}

/// Closes the trade once its close button has closed the window, and closes
/// the window on leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut commands: Commands,
    (mut window, mut trading): (ResMut<TradeWindow>, ResMut<Trading>),
    mut to_server: MessageWriter<ToServer>,
    session: Res<Session>,
) {
    let closed =
        |event: &UiEvent| event.id == UiId::new("trade", "panel") && event.value == UiValue::Closed;
    let closed = events.read().filter(|event| closed(event)).count() > 0;
    if window.document.is_some() && (closed || !session.is_playing()) {
        // The rest of the document goes with its panel.
        close_window(&mut window, &mut trading, &mut to_server, &mut commands);
    }
}
