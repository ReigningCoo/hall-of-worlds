//! How each race behaves on this server (`shared::behaviour`): the game's own,
//! with its mods' replacing a race's by `kind`. Mods are the `behaviours/*.ron`
//! files in the mods folder (`crate::mods`), each a list of races' behaviours,
//! read in name order at start. Never sent to clients. Kept as the resource
//! `BehaviourTable`, shared with every world (`ServerWorlds::use_behaviours`)
//! for mobs' choices (`entities::choice`).

use std::path::Path;
use std::sync::Arc;

use bevy::log::{debug, info, warn};
use bevy::prelude::{Res, Resource};
use shared::behaviour::Behaviours;

/// The behaviours this server's mobs follow.
#[derive(Resource, Clone, Debug)]
pub(crate) struct BehaviourTable(pub(crate) Arc<Behaviours>);

/// The behaviours for a server keeping its data in `data` (none: the game's own).
pub(crate) fn load(data: Option<&Path>) -> BehaviourTable {
    let behaviours = match crate::mods::folder(data) {
        Some(folder) => with_mods(&folder.join("behaviours")),
        None => Behaviours::built_in(),
    };
    BehaviourTable(Arc::new(behaviours))
}

/// The game's behaviours with those in every `.ron` file in `folder`, in name
/// order. A file that can't be read or has a wrong entry is warned about and
/// left out.
fn with_mods(folder: &Path) -> Behaviours {
    let mut behaviours = Behaviours::built_in();
    for (file, text) in crate::mods::ron_files(folder) {
        match text.and_then(|text| Behaviours::read(&text)) {
            Ok(defs) => {
                let count = defs.len();
                behaviours.merge(defs);
                info!(
                    "Behaviours from {}: {count} races replacing the game's",
                    file.display()
                );
            }
            Err(error) => warn!("Leaving out the behaviours in {}: {error}", file.display()),
        }
    }
    behaviours
}

/// Says at start how many races have behaviours.
pub(crate) fn announce(table: Res<BehaviourTable>) {
    debug!("Behaviours for {} races", table.0.len());
}

#[cfg(test)]
mod tests;
