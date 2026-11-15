//! An item's lines in its popup: what it gives while worn or when eaten
//! (`shared::bonus`), each worded through the language files (`item.ftl`).
//! Only what the client's copy of the item knows goes in.

use shared::bonus::{Others, food, food_effect, item_bonuses, poison};
use shared::item::Item;
use shared::lang::{Args, Lang};

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
    if food(item.kind).is_some_and(|food| food.others == Others::Poisoned) {
        // "Poisons other kinds: −1 Health a second for 60 s".
        for line in effect_lines(&poison(0), lang) {
            let args = Args::new().text("line", line);
            lines.push(written(lang.render("item-poisons", &args)));
        }
    }
    lines
}

#[cfg(test)]
mod tests;
