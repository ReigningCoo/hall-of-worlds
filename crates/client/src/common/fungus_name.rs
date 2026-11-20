//! What a fungus is called to this player (fungi plan PR 7, never wrong
//! information): its name if their character knows it, "Looks like X" when
//! they know the name of a lookalike X, else "Unknown mushroom" or "Unknown
//! lichen" (`shared::fungus::Shown`). Every place a fungus is named goes
//! through this: hover names, item popups, the effect bar, notices.

use shared::fungus::{Fungi, FungusDef, FungusKnowledge, Shown};
use shared::item::Item;
use shared::lang::{Args, Lang};

use super::written;

/// Its name as `known` lets it be shown.
pub fn fungus_name(def: &FungusDef, known: &FungusKnowledge, lang: &Lang) -> String {
    match known.shown(def, Fungi::built_in()) {
        Shown::Known(def) => def.name.clone(),
        Shown::LooksLike(other) => {
            let args = Args::new().text("fungus", other.name.as_str());
            written(lang.render("fungus-looks-like", &args))
        }
        Shown::Unknown { lichen: true } => written(lang.render("lichen-unknown", &Args::new())),
        Shown::Unknown { lichen: false } => written(lang.render("fungus-unknown", &Args::new())),
    }
}

/// A fungus by its key as `known` lets it be shown, "Fungus" for one the
/// table lacks.
pub fn fungus_key_name(key: &str, known: &FungusKnowledge, lang: &Lang) -> String {
    (Fungi::built_in().get(key))
        .map_or_else(|| "Fungus".to_owned(), |def| fungus_name(def, known, lang))
}

/// An item's name (`Item::name`), a fungus's as `known` lets it be shown ("Unknown
/// mushroom Slice" for a slice of one). Herbs are named by their popups.
pub fn item_name(item: &Item, known: &FungusKnowledge, lang: &Lang) -> String {
    let Some(form) = item.fungus() else {
        return item.name();
    };
    let name = fungus_key_name(&form.fungus, known, lang);
    match form.serving {
        true => format!("{name} Slice"),
        false => name,
    }
}

#[cfg(test)]
mod tests;
