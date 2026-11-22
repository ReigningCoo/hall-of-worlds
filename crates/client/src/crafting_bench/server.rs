//! What the server says of benches: their tools (kept in `common::Benches`,
//! and caught up on an open panel), a refusal, and a new level forgetting
//! them all and closing the panel.

use bevy::prelude::*;
use shared::protocol::{BenchNews, ServerMessage};

use super::BenchPanel;
use super::close::close_panel;
use super::markup::REFUSED;
use crate::common::Benches;
use crate::connection::FromServer;
use crate::dom::Dom;

pub(super) fn read_server(
    mut from_server: MessageReader<FromServer>,
    mut benches: ResMut<Benches>,
    mut panel: ResMut<BenchPanel>,
    mut dom: Dom,
    mut commands: Commands,
) {
    for FromServer(message) in from_server.read() {
        match message {
            // (A welcome may be from another server, after logging out.)
            ServerMessage::Welcome { .. }
            | ServerMessage::EnterLevel(_)
            | ServerMessage::EnterHall { .. }
            | ServerMessage::OnWorldMap { .. } => {
                benches.0.clear();
                benches.1.clear();
                close_panel(&mut panel, &mut commands);
            }
            ServerMessage::Bench(BenchNews::Tools(told)) => {
                for seen in told {
                    benches.0.insert(seen.at, seen.held.clone());
                    match seen.lit {
                        true => benches.1.insert(seen.at),
                        false => benches.1.remove(&seen.at),
                    };
                    let open = panel.0.as_mut().filter(|open| open.piece.at == seen.at);
                    if let Some(open) = open {
                        open.grid.heard(&shown(&seen.held));
                        dom.set_flag("bench/refused", "hidden", true);
                    }
                }
            }
            ServerMessage::Bench(BenchNews::Refused) => {
                // What was let go on the grid goes back to what it was.
                if let Some(open) = &mut panel.0 {
                    let tools = benches.0.get(&open.piece.at);
                    open.grid.heard(&tools.map(shown).unwrap_or_default());
                    dom.set_text("bench/refused", REFUSED);
                    dom.set_flag("bench/refused", "hidden", false);
                }
            }
            _ => {}
        }
    }
}

/// What a piece's panel shows: a bench's tools, then what lies in its
/// storage; a rack's casks.
pub(super) fn shown(held: &shared::furniture::Held) -> Vec<shared::item::Item> {
    held.items().cloned().collect()
}
