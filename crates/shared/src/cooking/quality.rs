//! A cooked thing's least quality (cooking plan): its ingredients' quality,
//! moved by its balance and, when the chance is certain, by its steps × the
//! sub-skills used (an intermediate's earlier steps and sub-skills
//! included), capped at Rare away from a proper hearth.

use super::{BALANCED_ROLES, Heat, History, Portion};
use crate::alchemy::{CERTAIN, moved};
use crate::item::Quality;

/// The best anything cooked away from a proper hearth can be.
pub const AWAY_CAP: Quality = Quality::Rare;

/// The foods' quality, the mean of their ranks (Poor 0, Common 1, ...)
/// weighted by grams and rounded down; foods with none count as Common, and
/// water not at all. Common when there is no food.
pub fn ingredient_quality(portions: &[Portion]) -> Quality {
    let (mut ranks, mut grams) = (0u64, 0u64);
    for portion in portions {
        if let Portion::Food {
            grams: g, quality, ..
        } = portion
        {
            ranks += rank(quality.unwrap_or(Quality::Common)) * u64::from(*g);
            grams += u64::from(*g);
        }
    }
    match ranks.checked_div(grams) {
        Some(mean) => Quality::ALL[mean as usize],
        None => Quality::Common,
    }
}

fn rank(quality: Quality) -> u64 {
    Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1) as u64
}

/// Whole quality steps the sub-skills add: the steps (in halves) × the mean
/// ∛ of the sub-skills used (each tool once) ÷ 10, rounded down once; so a
/// step at a sub-skill of 1 is a tenth of a step, as Alchemy's.
pub fn skill_steps(history: &History) -> u32 {
    if history.roots.is_empty() {
        return 0;
    }
    let roots: u32 = history.roots.iter().map(|(_, root)| root).sum();
    history.halves * roots / (history.roots.len() as u32 * 20)
}

/// The least quality, what the preview shows: `ingredient_quality` moved by
/// `balance` − `BALANCED_ROLES`, and by `skill_steps` only when `chance` is
/// at least `CERTAIN`; Poor to Legendary, then at most `AWAY_CAP` unless
/// worked at a hearth.
pub fn least_quality(
    ingredients: Quality,
    balance: i32,
    history: &History,
    chance: u32,
    heat: Heat,
) -> Quality {
    let skill = if chance >= CERTAIN {
        skill_steps(history) as i32
    } else {
        0
    };
    capped(moved(ingredients, balance - BALANCED_ROLES + skill), heat)
}

/// `quality`, at most `AWAY_CAP` unless worked at a hearth.
pub fn capped(quality: Quality, heat: Heat) -> Quality {
    if heat == Heat::Hearth {
        quality
    } else {
        quality.min(AWAY_CAP)
    }
}
