//! What a cellar cask holds when the server first finds it (Claude's first
//! guesses for the owner to tune): a fifth empty, about a seventh vinegar,
//! the rest drink (ale a third of the time, else a fruit's), aged up to 120
//! game days; most sealed and full, some opened and partly drunk, counting
//! down to vinegar.

use crate::cask::{CaskForm, CaskTime};
use crate::fermentation::{
    Base, Drink, GAME_DAY_SECS, Mash, aged, best_before_secs, ferment, rolled_drink,
    rolled_vinegar, tolerance,
};
use crate::item::Quality;
use crate::potion::Liquid;
use crate::rng::Rng;
use crate::vegetable::Vegetables;

use super::CellarCask;

/// In percent: empty, vinegar, the rest drink.
const EMPTY: u64 = 20;
const VINEGAR: u64 = 15;
/// In percent of drinks: opened and partly drunk.
const OPENED: u64 = 30;
/// In percent of drinks: barley ale.
const ALE: u64 = 35;
/// The longest a cellar's drink has aged.
const MOST_AGED_DAYS: u64 = 120;

/// The cask `cask` as found: what it holds, how much, and its clocks.
pub fn stock(cask: CellarCask, rng: &mut Rng) -> CaskForm {
    let form = CaskForm::new(cask.size, cask.wood);
    let roll = rng.range_u64(0, 100);
    if roll < EMPTY {
        return form.opened();
    }
    if roll < EMPTY + VINEGAR {
        let vinegar = if rng.range_u64(0, 3) == 0 {
            Liquid::Vinegar(Box::new(Base::Grain("barley".to_owned())))
        } else {
            rolled_vinegar(rng)
        };
        let ml = part_of(&form, rng);
        return form.holding(Some(vinegar), ml).opened();
    }
    let made = rng.pick_weighted(&[
        (Quality::Poor, 15),
        (Quality::Common, 55),
        (Quality::Uncommon, 22),
        (Quality::Rare, 8),
    ]);
    let Some(mut drink) = drink(rng, made) else {
        return form.opened();
    };
    let aged_secs = rng.range_u64(0, MOST_AGED_DAYS * GAME_DAY_SECS + 1);
    drink.quality = aged(made, drink.base.kind(), cask.wood, aged_secs);
    let time = CaskTime {
        made: Some(made),
        aged: aged_secs,
        ..CaskTime::default()
    };
    let opened = rng.range_u64(0, 100) < OPENED;
    let keeps = best_before_secs(drink.strength, drink.vinegar);
    let ml = if opened {
        part_of(&form, rng)
    } else {
        form.most_ml()
    };
    let mut form = form.holding(Some(Liquid::Drink(Box::new(drink))), ml);
    form.time = Box::new(time);
    if opened {
        form = form.opened();
        // Opened some while ago, but not yet turned.
        form.time.opened_for = keeps.map_or(0, |keeps| rng.range_u64(0, keeps));
    }
    form
}

/// A drink fermented with a yeast of `made`'s quality: barley ale or a
/// rolled fruit's.
fn drink(rng: &mut Rng, made: Quality) -> Option<Drink> {
    if rng.range_u64(0, 100) < ALE {
        let barley = Vegetables::built_in().get("barley")?;
        let fermented = ferment(&Mash::grain(barley, 7, 1000)?, tolerance(made))?;
        return Some(Drink::new(
            Base::Grain(barley.key.clone()),
            &fermented,
            made,
        ));
    }
    match rolled_drink(rng) {
        Liquid::Drink(drink) => Some(*drink),
        _ => None,
    }
}

/// Between a tenth and nine tenths of what it holds.
fn part_of(form: &CaskForm, rng: &mut Rng) -> u32 {
    let tenths = rng.range_u64(1, 10) as u32;
    form.most_ml() / 10 * tenths
}
