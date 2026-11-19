//! Taking in what the server says about marks.

use bevy::prelude::*;
use shared::protocol::{MarkNews, ServerMessage};

use crate::common::MapMarks;
use crate::connection::FromServer;

/// Keeps `MapMarks` to what the server says: all of them after each `Welcome`,
/// then each added or removed.
pub(super) fn read_server(mut from_server: MessageReader<FromServer>, mut marks: ResMut<MapMarks>) {
    for FromServer(message) in from_server.read() {
        match message {
            // Another planet, or another server: until it says.
            ServerMessage::Welcome { .. } => marks.0.clear(),
            ServerMessage::Marks(MarkNews::All(all)) => marks.0.clone_from(all),
            ServerMessage::Marks(MarkNews::Added(mark)) => {
                marks.0.retain(|kept| kept.id != mark.id);
                marks.0.push(*mark);
            }
            ServerMessage::Marks(MarkNews::Removed(id)) => marks.0.retain(|kept| kept.id != *id),
            _ => {}
        }
    }
}
