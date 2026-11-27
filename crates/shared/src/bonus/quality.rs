//! Quality bonuses (owner, 2026-11-27): each quality step above Common gives a
//! worn or held item one more bonus (Uncommon 1 … Artifact 5), each a
//! different stat suiting it (`GEAR_STATS`, `gear_skills`). Rolled from the
//! item's id, kind and quality, so nothing is saved and both sides agree.
//! Now and then one is negative, doubling another; uniques (artifacts and
//! set items) never have one, and give twice as much.

use super::gear::{GEAR_STATS, SKILL_LEVEL, places_of};
use super::gear_skills::gear_skills;
use super::stat::{Amount, Bonus, Stat};
use crate::item::{Item, ItemCategory, Quality};

/// One bonus in this many is negative, on items with two or more (first
/// guess).
pub const NEGATIVE_ODDS: u64 = 20;

/// How many bonuses this quality gives: one a step above Common.
pub const fn quality_steps(quality: Quality) -> usize {
    match quality {
        Quality::Poor | Quality::Common => 0,
        Quality::Uncommon => 1,
        Quality::Rare => 2,
        Quality::Epic => 3,
        Quality::Legendary => 4,
        Quality::Artifact => 5,
    }
}

/// How many quality bonuses this item has of its own: set items one fewer
/// than their quality gives, never fewer than 1 (owner, 2026-11-27), as their
/// set gives more.
pub fn own_count(item: &Item, quality: Quality) -> usize {
    match item.set {
        Some(_) => quality_steps(quality).saturating_sub(1).max(1),
        None => quality_steps(quality),
    }
}

/// Artifacts and set items: twice as strong, never with a negative.
pub fn is_unique(item: &Item) -> bool {
    item.set.is_some() || item.quality == Some(Quality::Artifact)
}

/// Whether this item can have quality bonuses: gear that is worn or held
/// (weapons, armour, clothes, jewellery), not bags or tools.
pub fn takes_quality_bonuses(item: &Item) -> bool {
    let gear = matches!(
        item.kind.category(),
        ItemCategory::Weapons
            | ItemCategory::Armour
            | ItemCategory::Clothing
            | ItemCategory::Jewellery
    );
    gear && item.kind.def().wear.is_some()
}

/// The bonuses its quality gives this item, in the order rolled.
pub fn quality_bonuses(item: &Item) -> Vec<Bonus> {
    let Some(quality) = item.quality else {
        return Vec::new();
    };
    if !takes_quality_bonuses(item) {
        return Vec::new();
    }
    let mut left = candidates(item);
    let seed = item.id.seed().derive("quality bonuses");
    let seed = seed
        .derive(item.kind.def().name)
        .derive(&format!("{quality:?}"));
    let mut rng = seed.rng();
    let mut bonuses = Vec::new();
    while bonuses.len() < own_count(item, quality) && !left.is_empty() {
        let at = rng.range_u64(0, left.len() as u64) as usize;
        bonuses.push(left.remove(at));
    }
    if is_unique(item) {
        return bonuses.iter().map(|bonus| times(bonus, 2)).collect();
    }
    if bonuses.len() < 2 {
        return bonuses;
    }
    let negative: Vec<bool> = (0..bonuses.len())
        .map(|_| rng.range_u64(0, NEGATIVE_ODDS) == 0)
        .collect();
    with_negatives(&bonuses, negative)
}

/// Every bonus the item could roll, in a fixed order: the gear stats suiting
/// where it is worn, then its skills.
fn candidates(item: &Item) -> Vec<Bonus> {
    let places = places_of(&item.worn_locations());
    let stats = GEAR_STATS.iter().filter(|gear| gear.places & places != 0);
    let stats = stats.map(|gear| Bonus {
        stat: gear.stat,
        amount: gear.amount,
    });
    let skills = gear_skills(item).into_iter();
    let skills = skills.map(|skill| Bonus::flat(Stat::Skill(skill), SKILL_LEVEL));
    stats.chain(skills).collect()
}

/// Each bonus marked negative turned round, doubling the first bonus left
/// that is neither negative nor doubled; one with nothing left to double
/// stays as it was.
fn with_negatives(bonuses: &[Bonus], mut negative: Vec<bool>) -> Vec<Bonus> {
    let mut doubled = vec![false; bonuses.len()];
    for at in 0..bonuses.len() {
        if !negative[at] {
            continue;
        }
        let partner = (0..bonuses.len()).find(|&other| !negative[other] && !doubled[other]);
        match partner {
            Some(other) => doubled[other] = true,
            None => negative[at] = false,
        }
    }
    let factor = |at: usize| match (negative[at], doubled[at]) {
        (true, _) => -1,
        (false, true) => 2,
        (false, false) => 1,
    };
    (bonuses.iter().enumerate())
        .map(|(at, bonus)| times(bonus, factor(at)))
        .collect()
}

/// A bonus with its amount multiplied by `factor`.
fn times(bonus: &Bonus, factor: i32) -> Bonus {
    let amount = match bonus.amount {
        Amount::Flat(milli) => Amount::Flat(milli.saturating_mul(factor)),
        Amount::Percent(milli) => Amount::Percent(milli.saturating_mul(factor)),
    };
    Bonus { amount, ..*bonus }
}
