//! A cask's popup lines (drinks plan PR 6): the drink in it, how strong,
//! sweet and good; how long it has left fermenting; and, once opened, how
//! long until it turns to vinegar, or until juice ferments by itself. Days
//! are game days, as the server last told of the cask's clocks.

use shared::ferment::days_left;
use shared::fermentation::JUICE_TURNS_SECS;
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::potion::Liquid;

use crate::common::written;

/// The lines under a cask's "Holds", if it holds a drink or opened juice.
pub(super) fn cask_lines(item: &Item, lang: &Lang) -> Option<String> {
    let cask = item.cask()?;
    let days = |secs: u64| Args::new().count("days", days_left(secs) as i64);
    let mut lines = Vec::new();
    if let Some(Liquid::Drink(drink)) = &cask.liquid {
        let args = Args::new()
            .text("strength", drink.strength_words())
            .text("sweetness", drink.sweetness.words())
            .text("quality", format!("{:?}", drink.quality));
        lines.push(lang.render("item-drink", &args));
    }
    if cask.time.is_fermenting() {
        lines.push(lang.render("item-fermenting", &days(cask.time.fermenting)));
    } else if let Some(secs) = cask.turns_in() {
        lines.push(lang.render("item-turns-vinegar", &days(secs)));
    } else if cask.opened && matches!(cask.liquid, Some(Liquid::Juice(_) | Liquid::Cordial(_))) {
        let secs = JUICE_TURNS_SECS.saturating_sub(cask.time.opened_for);
        lines.push(lang.render("item-turns-wild", &days(secs)));
    }
    let lines: Vec<String> = lines.into_iter().map(written).collect();
    (!lines.is_empty()).then(|| lines.join("\n"))
}
