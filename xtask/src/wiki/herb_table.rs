//! The Herbs page's "Where they grow" table: a row per land with the herbs
//! that grow wild there, each linked to its page, rare ones marked, and a row
//! for herbs of marshes. Written between two comments in the page, so the
//! words round it stay as written by hand.

use shared::climate::Biome;
use shared::herb::{Abundance, HerbDef, Herbs};

use super::herb_page::{file_name, land_name, sorted};

pub(super) const START: &str = "<!-- herbs table: written by `cargo xtask wiki` -->\n";
pub(super) const END: &str = "<!-- end of herbs table -->\n";

/// The lands in the table's order.
const LANDS: [Biome; 9] = [
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

/// The page with its table written afresh between `START` and `END`.
pub(super) fn put_in(page: &str, herbs: &Herbs) -> Result<String, String> {
    let (before, rest) = page
        .split_once(START)
        .ok_or("The Herbs page has no table start comment")?;
    let (_, after) = rest
        .split_once(END)
        .ok_or("The Herbs page has no table end comment")?;
    Ok(format!("{before}{START}{}{END}{after}", table(herbs)))
}

fn table(herbs: &Herbs) -> String {
    let herbs = sorted(herbs);
    let row = |land: &str, grows: &dyn Fn(&HerbDef) -> bool| {
        let names: Vec<String> = herbs.iter().filter(|h| grows(h)).map(|h| link(h)).collect();
        format!("| {land} | {} |\n", names.join(", "))
    };
    let mut table = String::from("| Land | Herbs |\n| ---- | ----- |\n");
    for land in LANDS {
        table += &row(land_name(land), &|herb| herb.biomes.contains(&land));
    }
    table += &row("Marshes and wet ground", &|herb| herb.marsh);
    table
}

/// A herb's name linked to its page, marked if rare.
fn link(herb: &HerbDef) -> String {
    let rare = if herb.abundance == Abundance::Rare {
        " (rare)"
    } else {
        ""
    };
    format!("[{}](herbs/{}){rare}", herb.name, file_name(&herb.key))
}
