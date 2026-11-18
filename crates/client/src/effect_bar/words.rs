//! What an effect's popup says, through `effect.ftl` and the bonus wording
//! item popups share (`common::bonus_line`): its name, a line per bonus, what
//! ends it early, and how long it has left.

use shared::bonus::{Effect, Source};
use shared::energy::ACTION_TICKS_PER_SECOND;
use shared::fruit::{FruitTag, Fruits};
use shared::lang::{Args, Lang};
use shared::vegetable::{VegetableTag, Vegetables};

use crate::common::{amount_of, bonus_line, written};

/// "Eating: Bread", "Eating: Apple", "Apple" (a fruit's or vegetable's effects),
/// "Poisoned", "Dev Bonus", "Raw herb".
pub(super) fn name(source: Source, lang: &Lang) -> String {
    let words = match source {
        Source::Eaten(kind) => {
            let args = Args::new().text("item", kind.def().name);
            lang.render("effect-eaten", &args)
        }
        Source::EatenFruit(tag) => {
            let args = Args::new().text("item", fruit_name(tag));
            lang.render("effect-eaten", &args)
        }
        Source::FruitEffects(tag) => {
            let args = Args::new().text("fruit", fruit_name(tag));
            lang.render("effect-fruit", &args)
        }
        Source::EatenVegetable(tag) => {
            let args = Args::new().text("item", vegetable_name(tag));
            lang.render("effect-eaten", &args)
        }
        Source::VegetableEffects(tag) => {
            let args = Args::new().text("fruit", vegetable_name(tag));
            lang.render("effect-fruit", &args)
        }
        Source::Poison => lang.render("effect-poison", &Args::new()),
        Source::Dev => lang.render("effect-dev", &Args::new()),
        Source::RawHerb => lang.render("effect-raw-herb", &Args::new()),
    };
    written(words)
}

/// A fruit's name by its tag, or "Fruit" for one the table lacks.
fn fruit_name(tag: FruitTag) -> &'static str {
    Fruits::built_in()
        .by_tag(tag)
        .map_or("Fruit", |def| def.name.as_str())
}

/// A vegetable's name by its tag, or "Vegetable" for one the table lacks.
fn vegetable_name(tag: VegetableTag) -> &'static str {
    Vegetables::built_in()
        .by_tag(tag)
        .map_or("Vegetable", |def| def.name.as_str())
}

/// A line for each bonus it gives: "Restores 1 Health a second".
pub(super) fn bonus_lines(effect: &Effect, lang: &Lang) -> Vec<String> {
    let bonuses = effect.bonuses.iter().filter(|bonus| amount_of(bonus) != 0);
    bonuses.map(|bonus| bonus_line(bonus, lang)).collect()
}

/// What ends it early ("Ends if you move or are hurt"), if anything does.
pub(super) fn ends_early(effect: &Effect, lang: &Lang) -> Option<String> {
    let ends = match (effect.ends_on.moving, effect.ends_on.hurt) {
        (true, true) => "both",
        (true, false) => "moving",
        (false, true) => "hurt",
        (false, false) => return None,
    };
    let args = Args::new().text("ends", ends);
    Some(written(lang.render("effect-ends", &args)))
}

/// "Ends in 12 s", "Ends in 4 min 30 s": whole seconds left at tick `now`,
/// rounded up, so it reads 1 until it goes.
pub(super) fn ends_in(effect: &Effect, now: f64, lang: &Lang) -> Option<String> {
    let end = effect.end? as f64;
    let seconds = ((end - now).max(0.0) / ACTION_TICKS_PER_SECOND).ceil() as i64;
    let args = Args::new()
        .count("minutes", seconds / 60)
        .count("seconds", seconds % 60);
    Some(written(lang.render("effect-ends-in", &args)))
}
