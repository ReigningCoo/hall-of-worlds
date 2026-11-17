use std::fs;
use std::path::Path;

use super::{HERB_FOLDER, pages};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
}

/// The pages checked in are what `cargo xtask wiki` writes now: if the herbs
/// or the writer change, run it.
#[test]
fn wiki_pages_are_up_to_date() {
    let root = root();
    let pages = pages(root).expect("the pages");
    for (path, text) in &pages {
        let checked_in = fs::read_to_string(root.join(path)).unwrap_or_default();
        assert!(
            checked_in.replace("\r\n", "\n") == *text,
            "{} differs from what `cargo xtask wiki` writes: run it",
            path.display()
        );
    }
    for entry in fs::read_dir(root.join(HERB_FOLDER))
        .expect("docs/herbs")
        .flatten()
    {
        let path = Path::new(HERB_FOLDER).join(entry.file_name());
        assert!(
            pages.iter().any(|(page, _)| *page == path),
            "{} is no herb's page any more: run `cargo xtask wiki`",
            path.display()
        );
    }
}

/// Every herb page shows a picture that is there.
#[test]
fn every_herb_has_a_picture() {
    for herb in shared::herb::Herbs::built_in().iter() {
        let picture = root().join(format!("docs/assets/herbs/{}.png", herb.key));
        assert!(
            picture.exists(),
            "{} is missing: run `cargo xtask wiki --pictures`",
            picture.display()
        );
    }
}

/// No page tells a herb's effects: players learn them by playing.
#[test]
fn herb_pages_keep_effects_out() {
    for (path, text) in pages(root()).expect("the pages") {
        if path.ends_with("index.md") {
            continue;
        }
        assert!(
            !text.to_lowercase().contains("effect"),
            "{} speaks of effects",
            path.display()
        );
    }
}
