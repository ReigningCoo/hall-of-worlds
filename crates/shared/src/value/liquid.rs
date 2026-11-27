//! What a litre of each liquid is worth: first guesses.

use crate::potion::{Liquid, Oil, Water};

/// A potion's litre for each effect it holds (at least one).
const POTION_PER_EFFECT: u64 = 50_000;

/// A litre of `liquid` in fine units, before any quality.
pub fn per_litre(liquid: &Liquid) -> u64 {
    match liquid {
        Liquid::Water(water) => match water {
            Water::Plain | Water::Rain => 100,
            Water::Salt => 50,
            Water::Cave => 500,
            Water::Moon => 5_000,
        },
        Liquid::Oil(oil) => match oil {
            Oil::Neutral | Oil::Seed => 8_000,
            Oil::Nut | Oil::Grain => 10_000,
        },
        Liquid::Potion(contents) => POTION_PER_EFFECT * contents.doses().len().max(1) as u64,
        Liquid::Juice(_) => 3_000,
        Liquid::Cordial(_) => 1_500,
        Liquid::Drink(_) => 6_000,
        Liquid::Vinegar(_) => 2_000,
        Liquid::Alcohol => 20_000,
        Liquid::Milk(_) => 2_000,
        Liquid::Honey(_) => 15_000,
        Liquid::SoySauce => 10_000,
    }
}

/// `ml` of `liquid` in fine units, before any quality.
pub(super) fn of(liquid: &Liquid, ml: u32) -> u64 {
    per_litre(liquid) * ml as u64 / 1000
}
