//! A fungus's lines in its popup: its raw healing and food effects for one
//! serving (`fungus::eating`), "Heals nothing raw" when it doesn't, and a
//! piece's servings. Its poison isn't told here, nor the common ink cap's
//! window (its bonus and alcohol's sickness): knowing which fungi are
//! poisonous comes with the fungi plan's PR 7 (never wrong information).

use shared::bonus::{Effect, Source};
use shared::fungus::{self, Fungi};
use shared::item::Item;
use shared::lang::{Args, Lang};

use crate::common::{effect_lines, written};

/// Its poison is left out, so who eats it doesn't matter.
const NO_EATER: fungus::Eater = fungus::Eater {
    most_health: 0,
    easy: false,
};

/// One line for each effect of eating the top one, and the notes after them.
pub(super) fn fungus_lines(item: &Item, lang: &Lang) -> Vec<String> {
    let Some(form) = item.fungus() else {
        return Vec::new();
    };
    let Some(def) = Fungi::built_in().get(&form.fungus) else {
        return Vec::new();
    };
    let effects = fungus::eating(def, 0, NO_EATER).into_iter();
    let poison = |e: &Effect| matches!(e.source, Source::FungusPoison(_) | Source::FungusWindow(_));
    let effects = effects.filter(|e| !poison(e));
    let mut lines: Vec<String> = effects.flat_map(|e| effect_lines(&e, lang)).collect();
    if fungus::healing(def) == 0 {
        lines.push(written(lang.render("item-not-raw", &Args::new())));
    }
    if form.is_whole(def) {
        let args = Args::new().count("count", i64::from(def.servings));
        lines.push(written(lang.render("item-servings", &args)));
    }
    lines
}
