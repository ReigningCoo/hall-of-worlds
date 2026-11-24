//! Spots a spell's Light lit (magic plan PR 13, `SpellNews::Light`): fixed
//! lights for as long as the server said, pale and bright, forgotten when
//! they go out or the player leaves the level. Only for the eye: the server
//! keeps none, so whoever comes later sees nothing of them.

use bevy::prelude::*;
use shared::protocol::{ServerMessage, SpellNews};

use super::FixedLight;
use crate::connection::FromServer;

/// A Light's colour and strength at its own cell: a pale, cool white.
const SPELL_LIGHT: Vec3 = Vec3::new(0.85, 0.9, 1.0);

/// Lit spots: the cell, how far its light reaches, and when it goes out (in
/// `Time::elapsed_secs_f64`).
#[derive(Resource, Default)]
pub(in crate::dungeon_map) struct SpellLights(Vec<((i64, i64), u32, f64)>);

impl SpellLights {
    /// The lights they give.
    pub(super) fn lights(&self) -> impl Iterator<Item = FixedLight> + '_ {
        self.0.iter().map(|&(cell, radius, _)| FixedLight {
            cell,
            radius,
            colour: SPELL_LIGHT,
        })
    }
}

/// Keeps the lit spots as the server tells, letting go of those gone out
/// and all of them on another map. Changes only when they do.
pub(in crate::dungeon_map) fn follow_spell_lights(
    mut from_server: MessageReader<FromServer>,
    time: Res<Time>,
    mut lit: ResMut<SpellLights>,
) {
    let now = time.elapsed_secs_f64();
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::Spell(SpellNews::Light {
                cells,
                reach,
                millis,
            }) => {
                let until = now + f64::from(*millis) / 1000.0;
                lit.0
                    .extend(cells.iter().map(|&cell| (cell, *reach, until)));
            }
            ServerMessage::EnterLevel { .. }
            | ServerMessage::EnterHall { .. }
            | ServerMessage::OnWorldMap { .. }
            | ServerMessage::Teleported { .. }
                if !lit.0.is_empty() =>
            {
                lit.0.clear();
            }
            _ => {}
        }
    }
    if lit.0.iter().any(|&(_, _, until)| until <= now) {
        lit.0.retain(|&(_, _, until)| until > now);
    }
}
