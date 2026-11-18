//! `cargo xtask wiki [--pictures]`: the players' wiki pages made from the
//! game's data, so they say what the game does: a page per built-in herb
//! (`docs/herbs/`) and the Herbs page's "Where they grow" table, from
//! `shared::herb`; a page per fruit and nut (`docs/fruit/`) and the Fruit and
//! nuts page's tables, from `shared::fruit`; and the Weapons page, a page per
//! family and a page per named weapon (`docs/weapons/`), from `shared::blade`. A test fails if the pages
//! checked in differ from what this writes. With `--pictures` it also starts a
//! development copy of the game and saves each herb's, fruit's and named
//! weapon's picture into `docs/assets/herbs/`, `docs/assets/fruit/` and
//! `docs/assets/weapons/`.

mod fruit_facts;
mod fruit_page;
mod fruit_table;
mod herb_page;
mod herb_table;
mod pictures;
#[cfg(test)]
mod tests;
mod weapon_facts;
mod weapon_family;
mod weapon_lines;
mod weapon_page;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use shared::fruit::Fruits;
use shared::herb::Herbs;

pub const USAGE: &str = "\
  wiki [--pictures]
            Write the wiki pages made from the game's data: a page per herb
            in docs/herbs/ and the Herbs page's table, a page per fruit and nut
            in docs/fruit/ and that page's tables, and the weapons' pages in
            docs/weapons/. --pictures also builds and starts the game (debug
            API on, scratch data, title \"Dev:\") and saves each herb's,
            fruit's and named weapon's picture into docs/assets/herbs/,
            docs/assets/fruit/ and docs/assets/weapons/.";

/// Where the herbs' pages are, from the workspace root.
const HERB_FOLDER: &str = "docs/herbs";
/// The Herbs page, whose table is written between `herb_table::START` and `END`.
const HERB_INDEX: &str = "docs/herbs/index.md";
/// Where the fruits' and nuts' pages are, from the workspace root.
const FRUIT_FOLDER: &str = "docs/fruit";
/// The Fruit and nuts page, whose tables are written between `fruit_table`'s
/// comments.
const FRUIT_INDEX: &str = "docs/fruit/index.md";
/// Where the weapons' pages are, from the workspace root: the Weapons page and
/// a folder per family.
const WEAPON_FOLDER: &str = "docs/weapons";

pub fn run(root: &Path, args: &[String]) -> ExitCode {
    let pictures = match args {
        [] => false,
        [arg] if arg == "--pictures" => true,
        _ => {
            eprintln!("Usage:\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(error) = write(root) {
        eprintln!("==> {error}");
        return ExitCode::FAILURE;
    }
    if pictures && let Err(error) = pictures::save(root) {
        eprintln!("==> {error}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Every page this writes, by its path from the root, with its text: the
/// herbs' pages and the Herbs page with its table put right, the same for
/// fruit, and the weapons' pages.
pub(crate) fn pages(root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let herbs = Herbs::built_in();
    let mut pages: Vec<_> = herb_page::pages(&herbs)
        .into_iter()
        .map(|(file, text)| (Path::new(HERB_FOLDER).join(file), text))
        .collect();
    let index = fs::read_to_string(root.join(HERB_INDEX))
        .map_err(|error| format!("Can't read {HERB_INDEX}: {error}"))?;
    pages.push((HERB_INDEX.into(), herb_table::put_in(&index, &herbs)?));
    let fruits = Fruits::built_in();
    let fruit_pages = fruit_page::pages(fruits).into_iter();
    pages.extend(fruit_pages.map(|(file, text)| (Path::new(FRUIT_FOLDER).join(file), text)));
    let index = fs::read_to_string(root.join(FRUIT_INDEX))
        .map_err(|error| format!("Can't read {FRUIT_INDEX}: {error}"))?;
    pages.push((
        FRUIT_INDEX.into(),
        fruit_table::put_in(&index, fruits, &herbs)?,
    ));
    let weapons = weapon_family::pages().into_iter();
    pages.extend(weapons.map(|(file, text)| (Path::new(WEAPON_FOLDER).join(file), text)));
    Ok(pages)
}

/// The folders whose pages are all written by this, so a page in one that
/// isn't written any more is removed.
pub(crate) fn folders() -> Vec<PathBuf> {
    let families = weapon_family::FAMILIES.iter();
    let families = families.map(|family| Path::new(WEAPON_FOLDER).join(family.folder));
    [HERB_FOLDER, FRUIT_FOLDER, WEAPON_FOLDER]
        .into_iter()
        .map(PathBuf::from)
        .chain(families)
        .collect()
}

/// Writes every page, and removes pages in its folders it no longer writes.
fn write(root: &Path) -> Result<(), String> {
    let pages = pages(root)?;
    let entries = folders().into_iter().flat_map(|folder| {
        let found = fs::read_dir(root.join(&folder))
            .into_iter()
            .flatten()
            .flatten();
        found.map(move |entry| (folder.join(entry.file_name()), entry))
    });
    for (path, entry) in entries {
        let is_page = path.extension().is_some_and(|extension| extension == "md");
        if is_page && !pages.iter().any(|(page, _)| *page == path) {
            fs::remove_file(entry.path())
                .map_err(|error| format!("Can't remove {}: {error}", path.display()))?;
            println!("==> Removed {}", path.display());
        }
    }
    for (path, text) in &pages {
        let full = root.join(path);
        if let Some(folder) = full.parent() {
            fs::create_dir_all(folder)
                .map_err(|error| format!("Can't make {}: {error}", folder.display()))?;
        }
        if fs::read_to_string(&full).ok().as_deref() == Some(text.as_str()) {
            continue;
        }
        fs::write(&full, text)
            .map_err(|error| format!("Can't write {}: {error}", path.display()))?;
        println!("==> Wrote {}", path.display());
    }
    println!("==> {} wiki pages up to date", pages.len());
    Ok(())
}
