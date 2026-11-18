//! A page per built-in fruit and nut (not herbs' fruits, which are herbs): its
//! picture (and a serving's, for a big fruit), names and facts.

use shared::fruit::{FruitDef, Fruits, Unit};

use super::fruit_facts::rows;
use super::herb_page::file_name;

/// Every fruit's and nut's page, by file name, in name order.
pub(super) fn pages(fruits: &Fruits) -> Vec<(String, String)> {
    sorted(fruits)
        .into_iter()
        .enumerate()
        .map(|(at, def)| (format!("{}.md", file_name(&def.key)), page(def, at + 1)))
        .collect()
}

/// The fruit items in name order, as the menu and the table list them.
pub(super) fn sorted(fruits: &Fruits) -> Vec<&FruitDef> {
    let mut sorted: Vec<_> = fruits.items().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    sorted
}

fn page(def: &FruitDef, order: usize) -> String {
    let name = &def.name;
    let key = &def.key;
    let picture = |file: &str, alt: &str| {
        format!("![{alt}]({{{{ '/assets/fruit/{file}.png' | relative_url }}}})")
    };
    let mut pictures = picture(key, name);
    if def.unit == Unit::Whole {
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
parent: Fruit and nuts
grand_parent: Items
---

<!-- Written by `cargo xtask wiki` from the game's fruit: change them, not this page. -->

# {name}

{pictures}

*{latin}*. Back to [Fruit and nuts](../fruit).

| | |
| - | - |
{table}",
        latin = def.latin,
    )
}
