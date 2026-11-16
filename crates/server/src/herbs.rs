//! The herbs this server has (`shared::herb`): the game's own, with its mods'
//! added or replacing them by key. Mods are the `herbs/*.ron` files in the mods
//! folder (`HOW_MODS`, or `mods` in the data folder), each a list of herbs,
//! read in name order at start. Every client is sent the table after `Welcome`.

use std::path::{Path, PathBuf};
use std::{env, fs};

use bevy::log::{info, warn};
use shared::herb::Herbs;

/// Where the server's mods are, when not `mods` in its data folder.
pub const MODS_VARIABLE: &str = "HOW_MODS";

/// The herbs for a server keeping its data in `data` (none: the game's own).
pub(crate) fn load(data: Option<&Path>) -> Herbs {
    let folder = match env::var_os(MODS_VARIABLE) {
        Some(folder) => Some(PathBuf::from(folder)),
        None => data.map(|data| data.join("mods")),
    };
    match folder {
        Some(folder) => with_mods(&folder.join("herbs")),
        None => Herbs::built_in(),
    }
}

/// The game's herbs with those in every `.ron` file in `folder`, in name order.
/// A file that can't be read or has a wrong herb is warned about and left out.
fn with_mods(folder: &Path) -> Herbs {
    let mut herbs = Herbs::built_in();
    let Ok(entries) = fs::read_dir(folder) else {
        return herbs;
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "ron"))
        .collect();
    files.sort();
    for file in files {
        let read = fs::read_to_string(&file).map_err(|error| error.to_string());
        match read.and_then(|text| Herbs::read(&text)) {
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
