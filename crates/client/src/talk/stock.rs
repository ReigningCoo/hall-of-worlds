//! Who keeps each shop on this level, as the server says; a new level closes
//! every panel. A shop's stock is the trade window's (`trade`).

use bevy::prelude::*;
use shared::protocol::ServerMessage;

use super::spawn::close_all;
use super::{Heading, Keepers, OpenTalks};
use crate::connection::FromServer;

/// Who keeps what. A new level (or another server) closes every panel.
pub(super) fn read_server(
    mut from_server: MessageReader<FromServer>,
    nodes: Query<(), With<Node>>,
    mut keepers: ResMut<Keepers>,
    mut open: ResMut<OpenTalks>,
    mut heading: ResMut<Heading>,
    mut commands: Commands,
) {
    for FromServer(message) in from_server.read() {
        match message {
            // (A welcome may be from another server, after logging out.)
            ServerMessage::Welcome { .. }
            | ServerMessage::EnterLevel(_)
            | ServerMessage::EnterHall { .. } => {
                heading.0 = None;
                keepers.0.clear();
                close_all(&mut open, &nodes, &mut commands);
            }
            ServerMessage::Keepers(list) => keepers.0 = list.iter().copied().collect(),
            _ => {}
        }
    }
}
