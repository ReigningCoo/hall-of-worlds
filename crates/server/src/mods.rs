//! A server's mods: the folder they are in (`HOW_MODS`, or `mods` in the data
//! folder) and the `.ron` files in one of its folders (`herbs`, `behaviours`),
//! read in name order at start.

use std::path::{Path, PathBuf};
use std::{env, fs};

/// Where the server's mods are, when not `mods` in its data folder.
pub const MODS_VARIABLE: &str = "HOW_MODS";

/// The mods folder for a server keeping its data in `data` (none: no mods).
pub(crate) fn folder(data: Option<&Path>) -> Option<PathBuf> {
    match env::var_os(MODS_VARIABLE) {
        Some(folder) => Some(PathBuf::from(folder)),
        None => data.map(|data| data.join("mods")),
    }
}

/// Every `.ron` file in `folder`, in name order, with its text or why it
/// couldn't be read. None if there is no such folder.
pub(crate) fn ron_files(folder: &Path) -> Vec<(PathBuf, Result<String, String>)> {
    let Ok(entries) = fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "ron"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|file| {
            let text = fs::read_to_string(&file).map_err(|error| error.to_string());
            (file, text)
        })
        .collect()
}
