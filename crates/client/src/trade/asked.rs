//! Being asked to trade by another player (their asker's request reaching a
//! character set to Ask): a requester saying who, with Allow and Deny. Allow
//! opens the trade window on them (the server's `TradeStock` does); Deny, or
//! a press outside, tells them no. It goes by itself if they give up or go.

use bevy::prelude::*;
use shared::protocol::{ClientMessage, TradeAsk};

use super::markup::ASKED;
use super::state::TradeWindow;
use crate::connection::ToServer;
use crate::dom::{Dom, UiEvent, UiValue, document, top_strip};

pub(super) fn spawn_asked(mut commands: Commands) {
    commands.spawn(document(&ASKED)).insert(top_strip());
}

/// Shows the requester while someone is asking, saying who.
pub(super) fn show_asked(
    window: Res<TradeWindow>,
    mut dom: Dom,
    mut shown: Local<Option<shared::id::Id>>,
) {
    let now = window.asked.as_ref().map(|(by, _)| *by);
    if *shown == now {
        return;
    }
    if let Some((_, name)) = &window.asked {
        dom.set_text("trade_asked/who", format!("{name} asks to trade with you."));
    }
    dom.set_flag("trade_asked/dialog", "hidden", now.is_none());
    *shown = now;
}

/// Allow or Deny (or a press outside, which denies) answers the one asking.
pub(super) fn answer_asked(
    mut events: MessageReader<UiEvent>,
    mut window: ResMut<TradeWindow>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("trade_asked", name) = event.id.parts() else {
            continue;
        };
        let allow = match (&event.value, name) {
            (UiValue::Pressed, "allow") => true,
            (UiValue::Pressed, "deny") | (UiValue::Dismissed, _) => false,
            _ => continue,
        };
        if let Some((by, _)) = window.asked.take() {
            to_server.write(ToServer(ClientMessage::Trade(TradeAsk::Answer {
                by,
                allow,
            })));
        }
    }
}
