//! `cargo xtask wiki [--pictures]`: the players' wiki pages made from the
//! game's data, so they say what the game does. For now a page per built-in
//! herb (`docs/herbs/`) and the Herbs page's "Where they grow" table, from
//! `shared::herb`. A test fails if the pages checked in differ from what this
//! writes. With `--pictures` it also starts a development copy of the game
//! and saves each herb's picture into `docs/assets/herbs/`.

mod herb_page;
mod herb_table;
mod pictures;
#[cfg(test)]
mod tests;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use shared::herb::Herbs;

pub const USAGE: &str = "\
  wiki [--pictures]
            Write the wiki pages made from the game's data: a page per herb
            in docs/herbs/ and the Herbs page's table. --pictures also builds
            and starts the game (debug API on, scratch data, title \"Dev:\")
            and saves each herb's picture into docs/assets/herbs/.";

/// Where the herbs' pages are, from the workspace root.
const HERB_FOLDER: &str = "docs/herbs";
/// The Herbs page, whose table is written between `herb_table::START` and `END`.
const HERB_INDEX: &str = "docs/herbs/index.md";

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
/// herbs' pages and the Herbs page with its table put right.
pub(crate) fn pages(root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let herbs = Herbs::built_in();
    let mut pages: Vec<_> = herb_page::pages(&herbs)
        .into_iter()
        .map(|(file, text)| (Path::new(HERB_FOLDER).join(file), text))
        .collect();
    let index = fs::read_to_string(root.join(HERB_INDEX))
        .map_err(|error| format!("Can't read {HERB_INDEX}: {error}"))?;
    pages.push((HERB_INDEX.into(), herb_table::put_in(&index, &herbs)?));
    Ok(pages)
}

/// Writes every page, and removes herb pages no herb has any more.
fn write(root: &Path) -> Result<(), String> {
    let pages = pages(root)?;
    for entry in fs::read_dir(root.join(HERB_FOLDER))
        .into_iter()
        .flatten()
        .flatten()
    {
        let path = Path::new(HERB_FOLDER).join(entry.file_name());
        if !pages.iter().any(|(page, _)| *page == path) {
            fs::remove_file(entry.path())
                .map_err(|error| format!("Can't remove {}: {error}", path.display()))?;
            println!("==> Removed {}", path.display());
        }
    }
    for (path, text) in &pages {
        let full = root.join(path);
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
