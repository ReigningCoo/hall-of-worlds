//! Opening and closing the trade window: its key (T) or its button opens it or
//! closes it, and so does its close button; leaving the game closes it.

use bevy::prelude::*;

use super::button::TradeButton;
use super::markup::TRADE;
use crate::common::{Action, GameView, Preferences, Session};
use crate::dom::{UiEvent, UiValue, document, top_strip};
use crate::widgets::{ButtonPressed, UiId};

/// The window's document, while it is open.
#[derive(Resource, Default)]
pub(super) struct TradePanel {
    document: Option<Entity>,
}

pub(super) fn toggle_trade(
    mut pressed: MessageReader<ButtonPressed>,
    button: Query<(), With<TradeButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    preferences: Res<Preferences>,
    view: Res<State<GameView>>,
    mut panel: ResMut<TradePanel>,
    mut commands: Commands,
) {
    let by_button = pressed.read().any(|press| button.contains(press.0));
    let by_key = preferences.controls.just_pressed(&keys, Action::Trade);
    if *view.get() == GameView::Connecting || !(by_button || by_key) {
        return;
    }
    if let Some(document) = panel.document.take() {
        commands.entity(document).despawn();
        return;
    }
    let document = commands.spawn(document(&TRADE)).insert(top_strip()).id();
    panel.document = Some(document);
}

/// Forgets the window once its close button has closed it, and closes it on
/// leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut commands: Commands,
    mut panel: ResMut<TradePanel>,
    session: Res<Session>,
) {
    let closed =
        |event: &UiEvent| event.id == UiId::new("trade", "panel") && event.value == UiValue::Closed;
    let closed = events.read().filter(|event| closed(event)).count() > 0;
    let Some(document) = panel.document else {
        return;
    };
    if closed || !session.is_playing() {
        // The rest of the document goes with its panel.
        commands.entity(document).despawn();
        panel.document = None;
    }
}
