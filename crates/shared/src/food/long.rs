//! The long processes (cooking plan PR 12, owner 2026-11-21): soy sauce and
//! miso from koji, and cheese aged from fresh cheese. Each is a row stopped
//! early, so these are kinds of intermediate (`Rest`), found here before the
//! others:
//!
//! - soybeans boiled in a Cauldron and strained in a Sieve are Cooked
//!   Soybeans, which grow into Soybean Koji as they rest (a koji culture
//!   does best, ordinary yeast works, wild is slowest and a step poorer);
//! - koji salted (`SALTED_PERCENT`) in a step without heat is Moromi when
//!   kept wet with water (at least half its foods' weight), else Miso;
//! - fresh cheese salted the same way is a Cheese.
//!
//! Moromi, miso and cheese age a step better each few weeks of game days
//! (`Curve::Ages`); moromi is then pressed into bottles of soy sauce with
//! the oil press (`potion::Pressing::Moromi`, `soy_sauce_bottles`).

use super::{Rest, SALTED_PERCENT};
use crate::cooking::{Ingredients, Output, Portion, Source, Tool, is_seasoning};
use crate::item::Item;

/// Soybeans, in grams, for each bottle of soy sauce a moromi gives.
pub const SOY_GRAMS_PER_BOTTLE: u32 = 100;

/// The long process an output is part of, if any.
pub(super) fn long_process(output: &Output, ing: &Ingredients) -> Option<Rest> {
    let history = &output.history;
    let mut foods = 0;
    let mut salt = 0;
    let mut soy_only = true;
    let mut any_soy = false;
    for portion in &output.portions {
        let Portion::Food {
            source, key, grams, ..
        } = portion
        else {
            continue;
        };
        foods += grams;
        if *source == Source::Pantry && key == "salt" {
            salt += grams;
        }
        let soy = *source == Source::Vegetable && key == "soybean";
        any_soy |= soy;
        let seasoning = ing
            .get(*source, key)
            .and_then(|def| def.main)
            .is_some_and(is_seasoning);
        soy_only &= soy || seasoning;
    }
    let salted = salt > 0 && salt * 100 >= foods * SALTED_PERCENT;
    let cold = history.tools.last().is_some_and(|tool| !tool.needs_heat());
    match history.rests.last() {
        Some(Rest::Koji) if salted && cold => {
            let wet = output.water * 2 >= foods;
            return Some(if wet { Rest::Moromi } else { Rest::Miso });
        }
        Some(Rest::FreshCheese) if salted && cold => return Some(Rest::Cheese),
        _ => {}
    }
    let boiled = history.tools.ends_with(&[Tool::Cauldron, Tool::Sieve]);
    (boiled && history.rests.is_empty() && any_soy && soy_only).then_some(Rest::CookedSoybeans)
}

/// How many bottles of soy sauce `item` presses into: by the soybeans in it,
/// at least one; none if it is not moromi.
pub fn soy_sauce_bottles(item: &Item) -> u32 {
    let Some(food) = item.food() else {
        return 0;
    };
    if food.rest.as_ref().map(|rest| rest.kind) != Some(Rest::Moromi) {
        return 0;
    }
    let soy: u32 = (food.portions.iter())
        .map(|portion| match portion {
            Portion::Food {
                source: Source::Vegetable,
                key,
                grams,
                ..
            } if key == "soybean" => *grams,
            _ => 0,
        })
        .sum();
    (soy / SOY_GRAMS_PER_BOTTLE).max(1)
}
