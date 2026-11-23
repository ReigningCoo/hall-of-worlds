//! The server's news of spells on the player's level, kept for drawing:
//! missiles in `common::Missiles` (with their looks here), the rest queued
//! until the view draws them (`spawn_fx`).

use std::collections::HashMap;

use bevy::prelude::*;
use shared::protocol::{MissileId, MissileNews, ServerMessage, SpellNews};
use shared::spell::{Look, Shown};
use shared::spot::Spot;

use crate::common::Missiles;
use crate::connection::FromServer;

/// Something to draw once.
pub(super) enum Queued {
    /// A where step (not a missile).
    Where(Shown),
    /// An effect step landing on whoever stands at these spots.
    Landed(Vec<Spot>, Look),
    /// A missile landing.
    Struck(Spot, Look),
}

/// What is waiting to be drawn, and each flying missile's look.
#[derive(Resource, Default)]
pub(crate) struct FxQueue {
    pub(super) pending: Vec<Queued>,
    pub(super) looks: HashMap<MissileId, Look>,
}

/// Keeps `Missiles` and the queue as the server tells, forgetting both when
/// the player goes to another map.
pub(crate) fn take_news(
    mut from_server: MessageReader<FromServer>,
    mut missiles: ResMut<Missiles>,
    mut queue: ResMut<FxQueue>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::Missile(MissileNews::Flying { id, path, look }) => {
                missiles.launch(*id, *path, now);
                queue.looks.insert(*id, *look);
            }
            ServerMessage::Missile(MissileNews::Landed { id, at }) => {
                missiles.land(*id);
                let look = queue.looks.remove(id).unwrap_or_default();
                queue.pending.push(Queued::Struck(*at, look));
            }
            ServerMessage::Spell(SpellNews::Where(shown)) => {
                queue.pending.push(Queued::Where(shown.clone()));
            }
            ServerMessage::Spell(SpellNews::Landed { on, look }) => {
                queue.pending.push(Queued::Landed(on.clone(), *look));
            }
            ServerMessage::Welcome { .. }
            | ServerMessage::EnterLevel { .. }
            | ServerMessage::EnterHall { .. }
            | ServerMessage::OnWorldMap { .. }
            | ServerMessage::Teleported { .. } => {
                missiles.clear();
                queue.pending.clear();
                queue.looks.clear();
            }
            _ => {}
        }
    }
    if !missiles.is_empty() {
        missiles.forget_old(now);
    }
    if missiles.is_empty() {
        queue.looks.clear();
    }
    // Out of the view nothing is drawn: never let it pile up.
    if queue.pending.len() > 256 {
        queue.pending.clear();
    }
}
