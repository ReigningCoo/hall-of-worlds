//! `cargo xtask wiki --pictures`: starts a development copy of the game and has
//! it save each herb's picture (`client.save_picture`) for the wiki.

use std::fs;
use std::path::Path;

use shared::herb::Herbs;

/// Where herbs' pictures go, from the workspace root.
const HERB_PICTURES: &str = "docs/assets/herbs";
/// Pixels along a herb picture's side (it takes one cell).
const HERB_PIXELS: u32 = 128;

/// Saves every built-in herb's picture, variant 0, removing pictures no herb
/// has any more.
pub(super) fn save(root: &Path) -> Result<(), String> {
    let folder = root.join(HERB_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let herbs = Herbs::built_in();
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let kept = name
            .strip_suffix(".png")
            .is_some_and(|key| herbs.get(key).is_some());
        if !kept {
            let _ = fs::remove_file(entry.path());
            println!("==> Removed {HERB_PICTURES}/{name}");
        }
    }
    let mut game = crate::scenario::start_game(root)?;
    let saved = herbs.iter().try_for_each(|herb| {
        let path = folder.join(format!("{}.png", herb.key));
        crate::api::perform(&[
            "client.save_picture".into(),
            format!("herb={}", herb.key),
            "variant=0".into(),
            format!("pixels={HERB_PIXELS}"),
            format!("path={}", path.display()),
        ])
    });
    let _ = game.kill();
    let _ = game.wait();
    saved?;
    println!("==> Saved {} herb pictures in {HERB_PICTURES}", herbs.len());
    Ok(())
}
