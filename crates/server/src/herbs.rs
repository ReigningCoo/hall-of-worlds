//! The herbs this server has (`shared::herb`): the game's own, with its mods'
//! added or replacing them by key. Mods are the `herbs/*.ron` files in the mods
//! folder (`crate::mods`), each a list of herbs, read in name order at start.
//! Every client is sent the table after `Welcome`.

use std::path::Path;

use bevy::log::{info, warn};
use shared::herb::Herbs;

/// The herbs for a server keeping its data in `data` (none: the game's own).
pub(crate) fn load(data: Option<&Path>) -> Herbs {
    match crate::mods::folder(data) {
        Some(folder) => with_mods(&folder.join("herbs")),
        None => Herbs::built_in(),
    }
}

/// The game's herbs with those in every `.ron` file in `folder`, in name order.
/// A file that can't be read or has a wrong herb is warned about and left out.
fn with_mods(folder: &Path) -> Herbs {
    let mut herbs = Herbs::built_in();
    for (file, text) in crate::mods::ron_files(folder) {
        match text.and_then(|text| Herbs::read(&text)) {
            Ok(defs) => {
                let count = defs.len();
                let replaced = herbs.merge(defs);
                info!(
                    "Herbs from {}: {count}, {replaced} of them replacing the game's",
                    file.display()
                );
            }
            Err(error) => warn!("Leaving out the herbs in {}: {error}", file.display()),
        }
    }
    herbs
}

#[cfg(test)]
mod tests;
