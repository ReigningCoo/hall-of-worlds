//! What the report needs that no other module keeps: the character's id, the
//! poi the player stands in the cutout of in the wilderness, and the site a
//! debug tool made, if they are in one.

use bevy::prelude::*;
use shared::debug_report::DevPlace;
use shared::poi::PoiKind;
use shared::protocol::{CharacterId, ClientMessage, ServerMessage};

use crate::connection::{FromServer, ToServer};

#[derive(Resource, Default)]
pub(super) struct Known {
    /// The character played, as the server's welcome said.
    pub character: Option<CharacterId>,
    /// In the wilderness, the poi whose cutout the player stands in.
    pub in_poi: Option<PoiKind>,
    /// A site asked of Generate POI, Dev Area or Load, until the player enters it.
    asked: Option<DevPlace>,
    /// The site the player is in, if a debug tool made it.
    pub dev: Option<DevPlace>,
}

/// Notes a site asked for by a debug tool.
pub(super) fn note_sent(mut sent: MessageReader<ToServer>, mut known: ResMut<Known>) {
    for ToServer(message) in sent.read() {
        match message {
            &ClientMessage::DevGenerateSite { kind, seed, ruin } => {
                known.asked = Some(DevPlace::Site { kind, seed, ruin });
            }
            ClientMessage::DevGenerateArea(area) => {
                known.asked = Some(DevPlace::Area(area.checked()));
            }
            // Load: the debug tool's site it makes again.
            ClientMessage::DevGoToPlace(ask) if ask.dev.is_some() => known.asked = ask.dev,
            _ => {}
        }
    }
}

/// Keeps up with what the server tells.
pub(super) fn note_told(mut told: MessageReader<FromServer>, mut known: ResMut<Known>) {
    for FromServer(message) in told.read() {
        match message {
            ServerMessage::Welcome { character, .. } => {
                known.character = *character;
                known.dev = None;
            }
            // Coming into a site: the one asked for, if one was, else no
            // debug tool's.
            ServerMessage::EnterLevel(info) if info.entering_site => {
                known.dev = info.kind.poi().and(known.asked.take());
                known.in_poi = None;
            }
            ServerMessage::InPoi { kind, .. } => known.in_poi = *kind,
            _ => {}
        }
    }
}
