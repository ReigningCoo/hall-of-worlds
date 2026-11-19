//! Each wood's log picture, and one of kindling, for the Logs and kindling
//! page (`wood_table.rs`), saved by `cargo xtask wiki --pictures`
//! (`pictures.rs`) in `docs/assets/logs/`.

use std::fs;
use std::path::{Path, PathBuf};

use shared::id::Id;
use shared::material::{Material, WOODS};
use shared::woodcutting::{new_kindling, new_log};

/// Where logs' pictures go, from the workspace root.
pub(super) const LOG_PICTURES: &str = "docs/assets/logs";
/// Pixels along a log picture's long side (eight cells), and kindling's (two).
const LOG_PIXELS: u32 = 256;
const KINDLING_PIXELS: u32 = 128;
/// The wood the kindling picture is of.
pub(super) const KINDLING_WOOD: Material = Material::Oak;

/// A wood's picture's file name: "red-cedar.png".
pub(super) fn log_file(wood: Material) -> String {
    format!("{}.png", wood.name().to_lowercase().replace(' ', "-"))
}

/// Every log picture to save, and the kindling's: where, its pixels and the
/// item as JSON. Removes pictures no wood has any more.
pub(super) fn log_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String)>, String> {
    let folder = root.join(LOG_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let json = |item| serde_json::to_string(&item).map_err(|error| error.to_string());
    let mut pictures = Vec::new();
    let logs = WOODS.iter().map(|wood| wood.material).filter(|&wood| {
        !super::wood_table::trees_of(wood).is_empty() && !super::wood_table::only_kindling(wood)
    });
    for wood in logs {
        let item = new_log(wood, Id::stand_in(1), None);
        pictures.push((folder.join(log_file(wood)), LOG_PIXELS, json(item)?));
    }
    let kindling = new_kindling(KINDLING_WOOD, 1, Id::stand_in(1), None);
    pictures.push((
        folder.join("kindling.png"),
        KINDLING_PIXELS,
        json(kindling)?,
    ));
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {LOG_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}
