//! A page per built-in vegetable, pulse and grain: its picture (and a
//! serving's, for one cut into servings), names and facts.

use shared::vegetable::{VegetableDef, Vegetables};

use super::herb_page::file_name;
use super::vegetable_facts::{has_slice, rows};

/// Every vegetable's page, by file name, in name order.
pub(super) fn pages(vegetables: &Vegetables) -> Vec<(String, String)> {
    sorted(vegetables)
        .into_iter()
        .enumerate()
        .map(|(at, def)| (format!("{}.md", file_name(&def.key)), page(def, at + 1)))
        .collect()
}

/// The vegetables in name order, as the menu and the table list them.
pub(super) fn sorted(vegetables: &Vegetables) -> Vec<&VegetableDef> {
    let mut sorted: Vec<_> = vegetables.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted
}

fn page(def: &VegetableDef, order: usize) -> String {
    let name = &def.name;
    let key = &def.key;
    let picture = |file: &str, alt: &str| {
        format!("![{alt}]({{{{ '/assets/vegetables/{file}.png' | relative_url }}}})")
    };
    let mut pictures = picture(key, name);
    if has_slice(def) {
        pictures += " ";
        pictures += &picture(&format!("{key}-slice"), &format!("{name} Slice"));
    }
    let table: String = rows(def)
        .iter()
        .map(|(what, says)| format!("| {what} | {says} |\n"))
        .collect();
    format!(
        "---
title: {name}
nav_order: {order}
parent: Vegetables, pulses and grains
grand_parent: Items
---

<!-- Written by `cargo xtask wiki` from the game's vegetables: change them, not this page. -->

# {name}

{pictures}

*{latin}*. Back to [Vegetables, pulses and grains](../vegetables).

| | |
| - | - |
{table}",
        latin = def.latin,
    )
}
