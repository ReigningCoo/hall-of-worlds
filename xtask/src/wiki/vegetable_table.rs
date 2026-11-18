//! The Vegetables, pulses and grains page's "Where they grow" table, written
//! between two comments so the words round it stay as written by hand: a row
//! per land with the vegetables, pulses and grains that grow there (wild, and
//! the farmed ones in farms' fields too), those never farmed marked.

use shared::vegetable::{VegetableDef, VegetableSet, Vegetables};

use super::fruit_table::LANDS;
use super::herb_page::file_name;
use super::vegetable_page::sorted;

pub(super) const START: &str = "<!-- vegetable table: written by `cargo xtask wiki` -->\n";
pub(super) const END: &str = "<!-- end of vegetable table -->\n";

/// The page with its table written afresh between its comments.
pub(super) fn put_in(page: &str, vegetables: &Vegetables) -> Result<String, String> {
    let (before, rest) = page
        .split_once(START)
        .ok_or_else(|| format!("The Vegetables page has no {START:?} comment"))?;
    let (_, after) = rest
        .split_once(END)
        .ok_or_else(|| format!("The Vegetables page has no {END:?} comment"))?;
    // Blank lines round it: Markdown takes a comment with lines straight after
    // it as one block of HTML, and would show a table as plain text.
    Ok(format!(
        "{before}{START}\n{}\n{END}{after}",
        table(vegetables)
    ))
}

fn table(vegetables: &Vegetables) -> String {
    let sorted = sorted(vegetables);
    let mut table = String::from(
        "| Land | Vegetables | Pulses | Grains |\n| ---- | ---------- | ------ | ------ |\n",
    );
    for land in LANDS {
        let names = |set: VegetableSet| {
            let names: Vec<String> = (sorted.iter())
                .filter(|def| def.set == set && def.biomes.contains(&land))
                .map(|def| link(def))
                .collect();
            names.join(", ")
        };
        table += &format!(
            "| {} | {} | {} | {} |\n",
            super::herb_page::land_name(land),
            names(VegetableSet::Vegetable),
            names(VegetableSet::Pulse),
            names(VegetableSet::Grain)
        );
    }
    table
}

/// A vegetable's name linked to its page, marked if never farmed.
fn link(def: &VegetableDef) -> String {
    let wild = match def.wild_only {
        true => " (wild)",
        false => "",
    };
    format!("[{}](vegetables/{}){wild}", def.name, file_name(&def.key))
}
