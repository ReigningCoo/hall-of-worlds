//! The Mushrooms, fungi and lichens page's "Where they grow" table, written
//! between two comments so the words round it stay as written by hand: a row
//! per land with what grows above it and what grows under it, and a row for
//! the cave kinds found under any land.

use shared::fungus::{Fungi, FungusDef, Where};

use super::fruit_table::LANDS;
use super::fungus_page::sorted;
use super::herb_page::{file_name, land_name};

pub(super) const START: &str = "<!-- fungus table: written by `cargo xtask wiki` -->\n";
pub(super) const END: &str = "<!-- end of fungus table -->\n";

/// The page with its table written afresh between its comments.
pub(super) fn put_in(page: &str, fungi: &Fungi) -> Result<String, String> {
    let (before, rest) = page
        .split_once(START)
        .ok_or_else(|| format!("The Fungi page has no {START:?} comment"))?;
    let (_, after) = rest
        .split_once(END)
        .ok_or_else(|| format!("The Fungi page has no {END:?} comment"))?;
    // Blank lines round it, as the Vegetables page's.
    Ok(format!("{before}{START}\n{}\n{END}{after}", table(fungi)))
}

fn table(fungi: &Fungi) -> String {
    let sorted = sorted(fungi);
    let names = |keep: &dyn Fn(&FungusDef) -> bool| {
        let names: Vec<String> = sorted
            .iter()
            .filter(|def| keep(def))
            .map(|def| link(def))
            .collect();
        names.join(", ")
    };
    let mut table = String::from(
        "| Land | Above ground | Underground |\n| ---- | ------------ | ----------- |\n",
    );
    for land in LANDS {
        let here = |def: &FungusDef| def.biomes.contains(&land);
        table += &format!(
            "| {} | {} | {} |\n",
            land_name(land),
            names(&|def| here(def) && def.grows == Where::Surface),
            names(&|def| here(def) && def.grows != Where::Surface),
        );
    }
    table += &format!("| Any land | | {} |\n", names(&|def| def.biomes.is_empty()));
    table
}

/// A fungus's name linked to its page.
fn link(def: &FungusDef) -> String {
    format!("[{}](fungi/{})", def.name, file_name(&def.key))
}
