use std::fs;
use std::path::Path;

use super::weapon_lines::LINES;
use super::weapon_page::file_name;
use super::{folders, pages};

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
    for folder in folders() {
        for entry in fs::read_dir(root.join(&folder))
            .expect("a folder")
            .flatten()
        {
            let path = folder.join(entry.file_name());
            if path.extension().is_some_and(|extension| extension == "md") {
                assert!(
                    pages.iter().any(|(page, _)| *page == path),
                    "{} isn't written any more: run `cargo xtask wiki`",
                    path.display()
                );
            }
        }
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
        if path.ends_with("index.md") || !path.starts_with("docs/herbs") {
            continue;
        }
        assert!(
            !text.to_lowercase().contains("effect"),
            "{} speaks of effects",
            path.display()
        );
    }
}

/// Every fruit's and nut's page shows pictures that are there: a big fruit's
/// slice too.
#[test]
fn every_fruit_has_a_picture() {
    use shared::fruit::{Fruits, Unit};
    for def in Fruits::built_in().items() {
        let mut files = vec![def.key.clone()];
        if def.unit == Unit::Whole {
            files.push(format!("{}-slice", def.key));
        }
        for file in files {
            let picture = root().join(format!("docs/assets/fruit/{file}.png"));
            assert!(
                picture.exists(),
                "{} is missing: run `cargo xtask wiki --pictures`",
                picture.display()
            );
        }
    }
}

/// Every vegetable's, pulse's and grain's page shows pictures that are there:
/// a slice too, for one cut into servings.
#[test]
fn every_vegetable_has_a_picture() {
    use super::vegetable_facts::has_slice;
    for def in shared::vegetable::Vegetables::built_in().iter() {
        let mut files = vec![def.key.clone()];
        if has_slice(def) {
            files.push(format!("{}-slice", def.key));
        }
        for file in files {
            let picture = root().join(format!("docs/assets/vegetables/{file}.png"));
            assert!(
                picture.exists(),
                "{} is missing: run `cargo xtask wiki --pictures`",
                picture.display()
            );
        }
    }
}

/// Every grain's page shows its flour's picture, which is there.
#[test]
fn every_flour_has_a_picture() {
    for grain in shared::flour::grains() {
        let picture = root().join(format!("docs/assets/flour/{}.png", grain.key));
        assert!(
            picture.exists(),
            "{} is missing: run `cargo xtask wiki --pictures`",
            picture.display()
        );
    }
}

/// Every picture the Logs and kindling page shows is there.
#[test]
fn every_log_has_a_picture() {
    let page = std::fs::read_to_string(root().join("docs/logs.md")).unwrap();
    let pictures: Vec<&str> = (page.split("{{ '/").skip(1))
        .filter_map(|rest| rest.split_once('\'').map(|(path, _)| path))
        .collect();
    assert!(pictures.len() > 50, "{}", pictures.len());
    for path in pictures {
        let picture = root().join("docs").join(path);
        assert!(
            picture.exists(),
            "{} is missing: run `cargo xtask wiki --pictures`",
            picture.display()
        );
    }
}

/// A vegetable's page tells only its raw effects, as its popup does: what it
/// would do cooked stays out until there is cooking.
#[test]
fn vegetable_pages_keep_cooked_effects_out() {
    use shared::vegetable::Vegetables;
    let pages = pages(root()).expect("the pages");
    for def in Vegetables::built_in().iter() {
        let file = format!("{}.md", super::herb_page::file_name(&def.key));
        let path = Path::new("docs/vegetables").join(file);
        let (_, text) = pages
            .iter()
            .find(|(page, _)| *page == path)
            .expect("its page");
        let row = text
            .lines()
            .find(|line| line.starts_with("| Effects |"))
            .expect("an Effects row");
        if def.effects.iter().all(|effect| !effect.when.raw()) {
            assert_eq!(row, "| Effects | None |", "{}", path.display());
        }
    }
}

/// Every named weapon has its line, and every line a weapon.
#[test]
fn every_weapon_has_a_line() {
    let named = shared::blade::named_weapons();
    for weapon in &named {
        assert!(
            LINES.iter().any(|(name, _)| *name == weapon.name),
            "{} has no line in wiki/weapon_lines.rs",
            weapon.name
        );
    }
    for (name, _) in LINES {
        assert!(
            named.iter().any(|weapon| weapon.name == *name),
            "{name} in wiki/weapon_lines.rs is no named weapon"
        );
    }
}

/// Every named weapon's page shows a picture that is there, and no two share
/// a file name.
#[test]
fn every_weapon_has_a_picture() {
    let mut files: Vec<String> = Vec::new();
    for weapon in shared::blade::named_weapons() {
        let file = file_name(weapon.name);
        assert!(!files.contains(&file), "two weapons are {file}");
        let picture = root().join(format!("docs/assets/weapons/{file}.png"));
        assert!(
            picture.exists(),
            "{} is missing: run `cargo xtask wiki --pictures`",
            picture.display()
        );
        files.push(file);
    }
}

/// No table follows a comment straight away: Markdown would take both as one
/// block of HTML and show the table as plain text.
#[test]
fn tables_stand_apart_from_comments() {
    for (path, text) in pages(root()).expect("the pages") {
        let lines: Vec<&str> = text.lines().collect();
        for pair in lines.windows(2) {
            assert!(
                !(pair[0].trim_end().ends_with("-->") && pair[1].starts_with('|')),
                "{}: a table straight after a comment",
                path.display()
            );
            assert!(
                !(pair[0].starts_with('|') && pair[1].starts_with("<!--")),
                "{}: a comment straight after a table",
                path.display()
            );
        }
    }
}
