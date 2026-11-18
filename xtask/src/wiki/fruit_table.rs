//! The Fruit and nuts page's tables, written between two comments so the words
//! round them stay as written by hand: "Where they grow", a row per land with
//! the fruit and the nuts that grow wild there (rare ones marked) and a row for
//! those grown only in orchards; and the herbs that bear fruit.

use shared::climate::Biome;
use shared::fruit::{Abundance, FruitDef, FruitSet, Fruits};
use shared::herb::Herbs;

use super::fruit_page::sorted;
use super::herb_page::{file_name, land_name};

pub(super) const START: &str = "<!-- fruit table: written by `cargo xtask wiki` -->\n";
pub(super) const END: &str = "<!-- end of fruit table -->\n";
pub(super) const HERBS_START: &str =
    "<!-- herbs bearing fruit: written by `cargo xtask wiki` -->\n";
pub(super) const HERBS_END: &str = "<!-- end of herbs bearing fruit -->\n";

/// The lands in the table's order, as the Herbs page has them.
pub(super) const LANDS: [Biome; 9] = [
    Biome::Tundra,
    Biome::BorealForest,
    Biome::Woodland,
    Biome::SeasonalForest,
    Biome::Grassland,
    Biome::TemperateRainforest,
    Biome::Desert,
    Biome::Savanna,
    Biome::TropicalRainforest,
];

/// The page with both tables written afresh between their comments.
pub(super) fn put_in(page: &str, fruits: &Fruits, herbs: &Herbs) -> Result<String, String> {
    let page = between(page, START, END, &table(fruits))?;
    between(&page, HERBS_START, HERBS_END, &herbs_bearing(fruits, herbs))
}

fn between(page: &str, start: &str, end: &str, text: &str) -> Result<String, String> {
    let (before, rest) = page
        .split_once(start)
        .ok_or_else(|| format!("The Fruit and nuts page has no {start:?} comment"))?;
    let (_, after) = rest
        .split_once(end)
        .ok_or_else(|| format!("The Fruit and nuts page has no {end:?} comment"))?;
    // Blank lines round it: Markdown takes a comment with lines straight after
    // it as one block of HTML, and would show a table as plain text.
    Ok(format!("{before}{start}\n{text}\n{end}{after}"))
}

fn table(fruits: &Fruits) -> String {
    let fruits = sorted(fruits);
    let names = |set: FruitSet, grows: &dyn Fn(&FruitDef) -> bool| {
        let names: Vec<String> = fruits
            .iter()
            .filter(|def| def.set == set && grows(def))
            .map(|def| link(def))
            .collect();
        names.join(", ")
    };
    let row = |land: &str, grows: &dyn Fn(&FruitDef) -> bool| {
        format!(
            "| {land} | {} | {} |\n",
            names(FruitSet::Fruit, grows),
            names(FruitSet::Nut, grows)
        )
    };
    let mut table = String::from("| Land | Fruit | Nuts |\n| ---- | ----- | ---- |\n");
    for land in LANDS {
        table += &row(land_name(land), &|def| {
            def.abundance != Abundance::OrchardOnly && def.biomes.contains(&land)
        });
    }
    table += &row("Only in orchards", &|def| {
        def.abundance == Abundance::OrchardOnly
    });
    table
}

/// A fruit's name linked to its page, marked if rare.
fn link(def: &FruitDef) -> String {
    let rare = match def.abundance {
        Abundance::Rare => " (rare)",
        _ => "",
    };
    format!("[{}](fruit/{}){rare}", def.name, file_name(&def.key))
}

/// The herbs that bear fruit, each linked to its herb page, as a list.
fn herbs_bearing(fruits: &Fruits, herbs: &Herbs) -> String {
    let mut bearing: Vec<_> = herbs
        .iter()
        .filter(|herb| {
            herb.fruit
                .as_ref()
                .is_some_and(|key| fruits.get(key).is_some())
        })
        .collect();
    bearing.sort_by(|a, b| a.name.cmp(&b.name));
    bearing
        .iter()
        .map(|herb| format!("- [{}](herbs/{})\n", herb.name, file_name(&herb.key)))
        .collect()
}
