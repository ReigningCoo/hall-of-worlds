//! A fungus's popup (fungi plan PR 7, never wrong information): named as the
//! character knows it (`common::item_name`: "Unknown mushroom", "Looks like
//! Chanterelle"). One not known by name says only how many servings a piece
//! is. One known says what eating it raw does: its healing and food effects
//! for one serving (`fungus::eating`) or "Heals nothing raw", its poison (when
//! it starts, deadly or not, a false recovery) and the common ink cap's window
//! with what alcohol brings then; and for those used like herbs, each of the
//! four herb effects if known, else "?".

use shared::bonus::{Effect, Source};
use shared::fungus::{self, Fungi, FungusDef, FungusKnowledge, Onset, Severity, Trigger};
use shared::item::Item;
use shared::lang::{Args, Lang};

use super::herb::{HerbWords, effect_words};
use crate::common::{effect_lines, item_name, written};

/// A deadly poison's drain depends on the eater, so it is said in words
/// (`fungus-deadly`) and its line is left out: an eater of no Health drains 0.
const NO_EATER: fungus::Eater = fungus::Eater {
    most_health: 0,
    easy: false,
};

/// What to say of this item, if it is a fungus: by its top, as stacks are named.
pub(super) fn fungus_words(item: &Item, known: &FungusKnowledge, lang: &Lang) -> Option<HerbWords> {
    let top = item.top();
    let form = top.fungus()?;
    let name = item_name(top, known, lang);
    let Some(def) = Fungi::built_in().get(&form.fungus) else {
        return Some(HerbWords {
            name,
            ..HerbWords::default()
        });
    };
    let named = known.knows_name(&def.key);
    let mut lines = Vec::new();
    if named {
        lines.extend(food_lines(def, lang));
    }
    if form.is_whole(def) {
        let args = Args::new().count("count", i64::from(def.servings));
        lines.push(written(lang.render("item-servings", &args)));
    }
    if named {
        if known.knows_poison(&def.key) {
            lines.extend(poison_lines(def, lang));
        }
        lines.extend(herb_lines(def, known, lang));
    }
    Some(HerbWords {
        name,
        part: None,
        effects: lines,
    })
}

/// Its raw healing and food effects for one serving, or "Heals nothing raw".
fn food_lines(def: &FungusDef, lang: &Lang) -> Vec<String> {
    let food = |e: &Effect| matches!(e.source, Source::EatenFungus(_) | Source::FungusEffects(_));
    let effects = fungus::eating(def, 0, NO_EATER).into_iter().filter(food);
    let mut lines: Vec<String> = effects.flat_map(|e| effect_lines(&e, lang)).collect();
    if fungus::healing(def) == 0 {
        lines.push(written(lang.render("item-not-raw", &Args::new())));
    }
    lines
}

/// Its poison when eaten ("Poison: −2 Strength for 60 s"), when it starts,
/// deadly or not, a false recovery; or the ink cap's window and what alcohol
/// brings in it. Poisons not brought on yet (touch, breath, old, repeated)
/// say nothing.
fn poison_lines(def: &FungusDef, lang: &Lang) -> Vec<String> {
    let Some(poison) = &def.poison else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    let wrapped = |effects: Vec<Effect>, key: &str, lines: &mut Vec<String>| {
        for line in effects.iter().flat_map(|e| effect_lines(e, lang)) {
            let args = Args::new().text("line", line);
            lines.push(written(lang.render(key, &args)));
        }
    };
    match poison.trigger {
        Trigger::Eaten => {
            let onset = match poison.onset {
                Onset::Soon => "soon",
                Onset::Delayed => "delayed",
                Onset::Late => "late",
            };
            let args = Args::new().text("onset", onset);
            lines.push(written(lang.render("fungus-onset", &args)));
            let effects = fungus::poisoning(def, poison, 0, NO_EATER);
            wrapped(effects, "fungus-poison", &mut lines);
        }
        Trigger::WithAlcohol => {
            let window = fungus::eating(def, 0, NO_EATER).into_iter();
            let window = window.filter(|e| matches!(e.source, Source::FungusWindow(_)));
            wrapped(window.collect(), "fungus-window", &mut lines);
            let effects = fungus::poisoning(def, poison, 0, NO_EATER);
            wrapped(effects, "fungus-with-alcohol", &mut lines);
            return lines;
        }
        _ => return lines,
    }
    if poison.severity == Severity::Deadly {
        lines.push(written(lang.render("fungus-deadly", &Args::new())));
    }
    if poison.false_recovery {
        lines.push(written(lang.render("fungus-false-recovery", &Args::new())));
    }
    lines
}

/// For fungi used like herbs, each of its four effects worded if known, else "?".
fn herb_lines(def: &FungusDef, known: &FungusKnowledge, lang: &Lang) -> Vec<String> {
    let effects = def.herb.iter().enumerate();
    effects
        .map(|(n, effect)| match known.knows_effect(&def.key, n) {
            true => effect_words(effect, lang),
            false => written(lang.render("herb-effect-unknown", &Args::new())),
        })
        .collect()
}

#[cfg(test)]
mod tests;
