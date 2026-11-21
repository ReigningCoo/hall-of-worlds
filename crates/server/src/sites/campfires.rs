//! Campfires (`shared::campfire`, cooking plan PR 7) on visits' levels: kept
//! in the save of the level they burn on, by cell where its plants are
//! (`PlantKey`: the wilderness's by cell round the planet, a cutout's by its
//! site's; `LevelSave::campfires`), lit and fed by players (`light.rs`,
//! `connections/make_campfire.rs`). Every tick each player's level is given
//! the fires on it as they are now (`sync.rs`), its cells sent again where
//! they changed; a fire found burnt out is forgotten and its Ash left on the
//! ground where it burnt.

use shared::campfire::Campfire;

use super::plants::PlantKey;
use crate::ServerWorld;

mod light;
mod sync;

pub(crate) use light::{FireTarget, fire_target, light_or_feed};
pub(crate) use sync::sync_campfires;

/// The server's clock now, in whole seconds: what fires burn by (real time,
/// stopped while the server is paused).
pub(crate) fn fire_now(world: &ServerWorld) -> u64 {
    world.server_clock.seconds().max(0.0) as u64
}

/// The fire kept at `key`, if one is.
pub(crate) fn saved_campfire(
    world: &mut ServerWorld,
    (site, index, at): PlantKey,
) -> Option<Campfire> {
    let level = world.site(site).levels.get(&index)?;
    level.campfires.get(&at).cloned()
}

/// Keeps `fire` at `key`, or forgets the one there for `None`.
pub(crate) fn keep_campfire(
    world: &mut ServerWorld,
    (site, index, at): PlantKey,
    fire: Option<Campfire>,
) {
    let levels = &mut world.site_mut(site).levels;
    match fire {
        Some(fire) => {
            levels.entry(index).or_default().campfires.insert(at, fire);
        }
        None => {
            if let Some(level) = levels.get_mut(&index) {
                level.campfires.remove(&at);
            }
        }
    }
}
