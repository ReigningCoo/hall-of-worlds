//! A page per built-in mushroom, fungus and lichen: its picture (and a
//! serving's, for a big one cut into servings), names and facts.

use shared::fungus::{Fungi, FungusDef};

use super::fungus_facts::{has_slice, rows};
use super::herb_page::file_name;

/// Every fungus's page, by file name, in name order.
pub(super) fn pages(fungi: &Fungi) -> Vec<(String, String)> {
    sorted(fungi)
        .into_iter()
        .enumerate()
        .map(|(at, def)| (format!("{}.md", file_name(&def.key)), page(def, at + 1)))
        .collect()
}

/// The fungi in name order, as the menu and the table list them.
pub(super) fn sorted(fungi: &Fungi) -> Vec<&FungusDef> {
    let mut sorted: Vec<_> = fungi.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted
}

fn page(def: &FungusDef, order: usize) -> String {
    let name = &def.name;
    let key = &def.key;
    let picture = |file: &str, alt: &str| {
        format!("![{alt}]({{{{ '/assets/fungi/{file}.png' | relative_url }}}})")
    };
    let mut pictures = picture(key, name);
    if has_slice(def) {
        pictures += " ";
        pictures += &picture(&format!("{key}-slice"), &format!("{name} Slice"));
    }
    let second = match (def.invented, def.latin.is_empty()) {
        (true, _) | (_, true) => "Found only in this game".to_owned(),
        (false, false) => format!("*{}*", def.latin),
    };
    let table: String = rows(def)
        .iter()
        .map(|(what, says)| format!("| {what} | {says} |\n"))
        .collect();
    format!(
        "---
title: {name}
nav_order: {order}
parent: Mushrooms, fungi and lichens
grand_parent: Items
---

<!-- Written by `cargo xtask wiki` from the game's fungi: change them, not this page. -->

# {name}

{pictures}

{second}. Back to [Mushrooms, fungi and lichens](../fungi).

| | |
| - | - |
{table}"
    )
}
