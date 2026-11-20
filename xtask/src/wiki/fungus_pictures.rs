//! Each mushroom's, fungus's and lichen's picture for its page
//! (`fungus_page.rs`), and a slice's for a big one, saved by
//! `cargo xtask wiki --pictures` (`pictures.rs`) in `docs/assets/fungi/`.

use std::fs;
use std::path::{Path, PathBuf};

use shared::fungus::Fungi;
use shared::id::Id;
use shared::item_form::ItemForm;

use super::fungus_facts::{form, has_slice, pixels};

/// Where fungi's pictures go, from the workspace root.
pub(super) const FUNGUS_PICTURES: &str = "docs/assets/fungi";

/// Each fungus's picture to save, variant 0 (and a slice of a big one): where,
/// its pixels along its longest side and the item as JSON. Removes pictures no
/// fungus has any more.
pub(super) fn fungus_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String)>, String> {
    let folder = root.join(FUNGUS_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let mut pictures = Vec::new();
    for def in Fungi::built_in().iter() {
        let servings = match has_slice(def) {
            true => vec![false, true],
            false => vec![false],
        };
        for serving in servings {
            let mut item = shared::fungus::new_item(def, 1, Id::stand_in(1), None);
            item.form = Some(ItemForm::Fungus(form(def, serving)));
            let json = serde_json::to_string(&item).map_err(|error| error.to_string())?;
            let file = match serving {
                true => format!("{}-slice.png", def.key),
                false => format!("{}.png", def.key),
            };
            pictures.push((folder.join(file), pixels(def, serving), json));
        }
    }
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {FUNGUS_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}
