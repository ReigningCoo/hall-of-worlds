//! Each grain's flour picture for its page's Flour section (`vegetable_page.rs`),
//! saved by `cargo xtask wiki --pictures` (`pictures.rs`) in
//! `docs/assets/flour/`, one cell.

use std::fs;
use std::path::{Path, PathBuf};

use shared::id::Id;

/// Where flours' pictures go, from the workspace root.
pub(super) const FLOUR_PICTURES: &str = "docs/assets/flour";
/// Pixels along a flour picture's side: one cell.
const FLOUR_PIXELS: u32 = 128;

/// Each grain's flour picture to save: where, its pixels and a sack of it as
/// JSON. Removes pictures no grain has any more.
pub(super) fn flour_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String)>, String> {
    let folder = root.join(FLOUR_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let mut pictures = Vec::new();
    for grain in shared::flour::grains() {
        let item = shared::flour::new_item(grain, 1, Id::stand_in(1), None)
            .ok_or_else(|| format!("{} doesn't mill", grain.key))?;
        let json = serde_json::to_string(&item).map_err(|error| error.to_string())?;
        let path = folder.join(format!("{}.png", grain.key));
        pictures.push((path, FLOUR_PIXELS, json));
    }
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {FLOUR_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}
