//! `cargo xtask wiki --pictures`: starts a development copy of the game and has
//! it save each herb's, fruit's and vegetable's (and a big one's slice), grain's
//! flour and named weapon's picture (`client.save_picture`) for the wiki.

use std::fs;
use std::path::{Path, PathBuf};

use shared::herb::Herbs;

/// Where herbs' pictures go, from the workspace root.
const HERB_PICTURES: &str = "docs/assets/herbs";
/// Pixels along a herb picture's side (it takes one cell), and per cell of a
/// weapon's longest side.
const HERB_PIXELS: u32 = 128;
/// Where fruits' and nuts' pictures go, from the workspace root.
const FRUIT_PICTURES: &str = "docs/assets/fruit";
/// Where vegetables', pulses' and grains' pictures go, from the workspace root.
const VEGETABLE_PICTURES: &str = "docs/assets/vegetables";
/// Where weapons' pictures go, from the workspace root.
const WEAPON_PICTURES: &str = "docs/assets/weapons";

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
    let weapons = weapon_pictures(root)?;
    let flours = super::flour_pictures::flour_pictures(root)?;
    let logs = super::log_pictures::log_pictures(root)?;
    let fruits = fruit_pictures(root)?;
    let vegetables = vegetable_pictures(root)?;
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
    let saved = saved.and_then(|()| {
        (weapons.iter().chain(&flours).chain(&logs)).try_for_each(|(path, pixels, item)| {
            crate::api::perform(&[
                "client.save_picture".into(),
                format!("item={item}"),
                format!("pixels={pixels}"),
                format!("path={}", path.display()),
            ])
        })
    });
    let saved = saved.and_then(|()| {
        fruits.iter().try_for_each(|(path, pixels, key, serving)| {
            crate::api::perform(&[
                "client.save_picture".into(),
                format!("fruit={key}"),
                format!("serving={serving}"),
                "variant=0".into(),
                format!("pixels={pixels}"),
                format!("path={}", path.display()),
            ])
        })
    });
    let saved = saved.and_then(|()| {
        vegetables
            .iter()
            .try_for_each(|(path, pixels, key, serving)| {
                crate::api::perform(&[
                    "client.save_picture".into(),
                    format!("vegetable={key}"),
                    format!("serving={serving}"),
                    "variant=0".into(),
                    format!("pixels={pixels}"),
                    format!("path={}", path.display()),
                ])
            })
    });
    let _ = game.kill();
    let _ = game.wait();
    saved?;
    println!("==> Saved {} herb pictures in {HERB_PICTURES}", herbs.len());
    println!(
        "==> Saved {} fruit pictures in {FRUIT_PICTURES}",
        fruits.len()
    );
    println!(
        "==> Saved {} vegetable pictures in {VEGETABLE_PICTURES}",
        vegetables.len()
    );
    println!(
        "==> Saved {} weapon pictures in {WEAPON_PICTURES}",
        weapons.len()
    );
    println!(
        "==> Saved {} flour pictures in {}",
        flours.len(),
        super::flour_pictures::FLOUR_PICTURES
    );
    println!(
        "==> Saved {} log and kindling pictures in {}",
        logs.len(),
        super::log_pictures::LOG_PICTURES
    );
    Ok(())
}

/// Each named weapon's picture to save: where, its pixels along its longest
/// side and the weapon pictured, as JSON. Removes pictures no weapon has any
/// more.
fn weapon_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String)>, String> {
    let folder = root.join(WEAPON_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let mut pictures = Vec::new();
    for weapon in shared::blade::named_weapons() {
        let item = super::weapon_facts::facts(&weapon).example;
        let shape = item.shape();
        let pixels = HERB_PIXELS * u32::from(shape.width.max(shape.height));
        let json = serde_json::to_string(&item).map_err(|error| error.to_string())?;
        let path = folder.join(format!(
            "{}.png",
            super::weapon_page::file_name(weapon.name)
        ));
        pictures.push((path, pixels, json));
    }
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {WEAPON_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}

/// Each fruit's and nut's picture to save, variant 0 (and a slice of a big
/// fruit): where, its pixels along its longest side, its key and whether a
/// serving. Removes pictures no fruit has any more.
fn fruit_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String, bool)>, String> {
    use shared::fruit::{Fruits, Unit};
    let folder = root.join(FRUIT_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let mut pictures = Vec::new();
    for def in Fruits::built_in().items() {
        let pixels = super::fruit_facts::pixels(def, false);
        pictures.push((
            folder.join(format!("{}.png", def.key)),
            pixels,
            def.key.clone(),
            false,
        ));
        if def.unit == Unit::Whole {
            let pixels = super::fruit_facts::pixels(def, true);
            let path = folder.join(format!("{}-slice.png", def.key));
            pictures.push((path, pixels, def.key.clone(), true));
        }
    }
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {FRUIT_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}

/// Each vegetable's, pulse's and grain's picture to save, variant 0 (and a
/// slice of one cut into servings): where, its pixels along its longest side,
/// its key and whether a serving. Removes pictures no vegetable has any more.
fn vegetable_pictures(root: &Path) -> Result<Vec<(PathBuf, u32, String, bool)>, String> {
    use super::vegetable_facts::{has_slice, pixels};
    let folder = root.join(VEGETABLE_PICTURES);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
    let mut pictures = Vec::new();
    for def in shared::vegetable::Vegetables::built_in().iter() {
        let path = folder.join(format!("{}.png", def.key));
        pictures.push((path, pixels(def, false), def.key.clone(), false));
        if has_slice(def) {
            let path = folder.join(format!("{}-slice.png", def.key));
            pictures.push((path, pixels(def, true), def.key.clone(), true));
        }
    }
    for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
        if !pictures.iter().any(|(path, ..)| *path == entry.path()) {
            let _ = fs::remove_file(entry.path());
            println!(
                "==> Removed {VEGETABLE_PICTURES}/{}",
                entry.file_name().to_string_lossy()
            );
        }
    }
    Ok(pictures)
}
