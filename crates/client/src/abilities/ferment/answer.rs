//! What is done in Ferment's panel: choosing in the dropdowns, and Ferment
//! (asking the server) or Cancel.

use bevy::prelude::*;
use shared::protocol::ClientMessage;

use super::Fermenting;
use crate::connection::ToServer;
use crate::dom::{UiEvent, UiValue};

pub(in crate::abilities) fn answer_panel(
    mut events: MessageReader<UiEvent>,
    mut fermenting: ResMut<Fermenting>,
    mut to_server: MessageWriter<ToServer>,
) {
    for event in events.read() {
        let ("ferment", name) = event.id.parts() else {
            continue;
        };
        match (&event.value, name) {
            (UiValue::Choice { index, .. }, "cask") => fermenting.cask = *index,
            (UiValue::Choice { index, .. }, "add") => fermenting.add = *index,
            (UiValue::Choice { index, .. }, "yeast") => {
                fermenting.yeast = *index;
                fermenting.count = 1;
            }
            (UiValue::Choice { index, .. }, "count") => fermenting.count = *index as u32 + 1,
            (UiValue::Pressed, "ferment") => {
                if let Some(order) = fermenting.order() {
                    to_server.write(ToServer(ClientMessage::Ferment(order)));
                }
                fermenting.open = false;
            }
            (UiValue::Pressed, "cancel") | (UiValue::Dismissed, _) => fermenting.open = false,
            _ => continue,
        }
        fermenting.dirty = true;
    }
}
