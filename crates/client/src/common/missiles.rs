//! Missiles flying on the player's level (magic plan PR 6), as the server
//! tells of them (`MissileNews`): each one's way and when it was launched, so
//! the views can follow it (`at`). Kept by `dungeon_map` (`take_missiles`),
//! which also draws them.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::protocol::MissileId;
use shared::spell::Path;
use shared::spot::Spot;

/// How long a missile is kept past the end of its way with no word that it
/// landed (its news lost, or its caster gone), in seconds.
const KEPT_PAST_END: f64 = 1.0;

/// The missiles flying now, by id: each one's way and when it was launched
/// (`Time`'s elapsed seconds).
#[derive(Resource, Default, Debug)]
pub struct Missiles {
    flying: HashMap<MissileId, (Path, f64)>,
}

impl Missiles {
    /// One launched `now`.
    pub fn launch(&mut self, id: MissileId, path: Path, now: f64) {
        self.flying.insert(id, (path, now));
    }

    /// One landed: it is gone.
    pub fn land(&mut self, id: MissileId) {
        self.flying.remove(&id);
    }

    /// Every one, as on another level they mean nothing.
    pub fn clear(&mut self) {
        self.flying.clear();
    }

    /// Forgets those long past the end of their way.
    pub fn forget_old(&mut self, now: f64) {
        self.flying.retain(|_, (path, launched)| {
            now - *launched < path.millis() as f64 / 1000.0 + KEPT_PAST_END
        });
    }

    /// Where each one is `now`, and its way.
    pub fn at(&self, now: f64) -> impl Iterator<Item = (MissileId, Spot, &Path)> + '_ {
        self.flying.iter().map(move |(id, (path, launched))| {
            let millis = ((now - launched).max(0.0) * 1000.0) as u64;
            (*id, path.at(path.along_after(millis)), path)
        })
    }

    pub fn is_empty(&self) -> bool {
        self.flying.is_empty()
    }
}
