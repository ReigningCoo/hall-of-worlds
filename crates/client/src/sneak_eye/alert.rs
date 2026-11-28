//! Keeps `common::MobAlert` as the server says (mob behaviour plan PR 7:
//! `ServerMessage::MobAlert` when a mob is Alert to or Aware of the player);
//! off again on joining.

use bevy::prelude::*;
use shared::protocol::ServerMessage;

use crate::common::MobAlert;
use crate::connection::FromServer;

pub(super) fn take_alert(mut from_server: MessageReader<FromServer>, mut alert: ResMut<MobAlert>) {
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::MobAlert(on) => alert.set_if_neq(MobAlert(*on)),
            ServerMessage::Welcome { .. } => alert.set_if_neq(MobAlert(false)),
            _ => false,
        };
    }
}
