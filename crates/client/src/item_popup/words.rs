//! An item's lines in its popup: what it gives while worn or when eaten
//! (`shared::bonus`), each worded through the language files (`item.ftl`).
//! Only what the client's copy of the item knows goes in.

use shared::bonus::{Others, food, food_effect, item_bonuses, poison};
use shared::fruit::{Fruits, eating};
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::vegetable::{self, VegetableSet, Vegetables};

use crate::common::{amount_of, bonus_line, effect_lines, written};

/// One line for each bonus the item gives: worn ("+40 Toughness") or eaten
/// ("Restores 1 Health a second for 30 s; stops on moving or being hurt").
/// A stack's are its top's.
pub(super) fn bonus_lines(item: &Item, lang: &Lang) -> Vec<String> {
    let item = item.top();
    let worn = item_bonuses(item).into_iter();
    let worn = worn.filter(|bonus| amount_of(bonus) != 0);
    let mut lines: Vec<String> = worn.map(|bonus| bonus_line(&bonus, lang)).collect();
    if let Some(effect) = food_effect(item.kind, 0) {
        lines.extend(effect_lines(&effect, lang));
    }
    lines.extend(fruit_lines(item, lang));
    lines.extend(vegetable_lines(item, lang));
    lines.extend(super::ointment::ointment_lines(item, lang));
    if food(item.kind).is_some_and(|food| food.others == Others::Poisoned) {
        // "Poisons other kinds: −1 Health a second for 60 s".
        for line in effect_lines(&poison(0), lang) {
            let args = Args::new().text("line", line);
            lines.push(written(lang.render("item-poisons", &args)));
        }
    }
    lines
}

/// A fruit's healing and effects for one serving (`fruit::eating`), and a
/// whole one's servings: "Cut into 11 servings when eaten".
fn fruit_lines(item: &Item, lang: &Lang) -> Vec<String> {
    let Some(form) = item.fruit() else {
        return Vec::new();
    };
    let Some(def) = Fruits::built_in().get(&form.fruit) else {
        return Vec::new();
    };
    let effects = eating(def, 0);
    let mut lines: Vec<String> = effects.iter().flat_map(|e| effect_lines(e, lang)).collect();
    if form.is_whole(def) {
        let args = Args::new().count("count", i64::from(def.servings));
        lines.push(written(lang.render("item-servings", &args)));
    }
    lines
}

/// A vegetable's raw healing and effects for one serving (`vegetable::eating`),
/// "Heals nothing raw" or "Heals nothing until milled" when it doesn't, and a
/// whole one's servings.
fn vegetable_lines(item: &Item, lang: &Lang) -> Vec<String> {
    let Some(form) = item.vegetable() else {
        return Vec::new();
    };
    let Some(def) = Vegetables::built_in().get(&form.vegetable) else {
        return Vec::new();
    };
    let effects = vegetable::eating(def, 0);
    let mut lines: Vec<String> = effects.iter().flat_map(|e| effect_lines(e, lang)).collect();
    if vegetable::healing(def) == 0 {
        let key = match def.set {
            VegetableSet::Grain => "item-unmilled",
            _ => "item-not-raw",
        };
        lines.push(written(lang.render(key, &Args::new())));
    }
    if form.is_whole(def) {
        let args = Args::new().count("count", i64::from(def.servings));
        lines.push(written(lang.render("item-servings", &args)));
    }
    lines
}

#[cfg(test)]
mod tests;
