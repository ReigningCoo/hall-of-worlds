//! Things left lying in the inn's public rooms (`RoomKind::is_public`) are
//! cleared after a while and destroyed: a stand-in for cleaners, until there
//! are staff to do it (The Way Inn plan).

use bevy::prelude::*;
use shared::id::Id;
use shared::inn;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::ServerWorld;
use crate::storage::SiteId;

/// How long something lies in a public room of the inn before it is cleared,
/// in real seconds (owner: about 2 minutes).
pub(crate) const LITTER_SECONDS: u64 = 120;

/// When each thing lying in an instance's public rooms was first seen there,
/// by instance. Not saved: instances never are.
#[derive(Default)]
pub(crate) struct Litter(BTreeMap<SiteId, BTreeMap<Id, Instant>>);

impl Litter {
    /// Forgets an instance's: it is closed, or made anew.
    pub(crate) fn forget(&mut self, id: SiteId) {
        self.0.remove(&id);
    }
}

/// Clears what has lain in the inn's public rooms for `LITTER_SECONDS` by
/// `now`, in every open instance; a bag left empty goes. Everyone near is
/// told by the ground's own telling (`ground::sync`).
pub(crate) fn clear_litter(world: &mut ServerWorld, now: Instant) {
    let most = Duration::from_secs(LITTER_SECONDS);
    for number in world.open_inns() {
        let id = SiteId::InnInstance(number);
        let seen = world.litter.0.remove(&id).unwrap_or_default();
        let mut still = BTreeMap::new();
        let mut cleared = 0;
        for (&index, level) in world.site(id).levels.iter_mut() {
            if level.ground.is_empty() {
                continue;
            }
            let rooms = inn::rooms(index);
            let public = |(x, y)| {
                let room = rooms.iter().find(|room| room.contains(x, y));
                room.is_none_or(|room| room.kind.is_public())
            };
            level.ground.retain(|spot, items| {
                if !public(spot.cell()) {
                    return true;
                }
                items.retain(|item| {
                    let since = seen.get(&item.id).copied().unwrap_or(now);
                    let stays = now.duration_since(since) < most;
                    if stays {
                        still.insert(item.id, since);
                    } else {
                        cleared += 1;
                    }
                    stays
                });
                !items.is_empty()
            });
        }
        if cleared > 0 {
            debug!("Cleared {cleared} things left lying in {id:?}");
        }
        world.litter.0.insert(id, still);
    }
}

#[cfg(test)]
mod tests;
