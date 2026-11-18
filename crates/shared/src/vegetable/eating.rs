//! What eating a vegetable raw does (owner, 2026-11-17). Cooked, one would heal
//! as fruit does, 1 Health a second for a quarter of its kcal in seconds (half
//! as long again if exceptional): that figure is kept for cooking, later
//! (`cooked_ticks`). Raw, only what people eat raw commonly or sometimes heals,
//! for a quarter of that (`RAW_SHARE`); grains heal nothing until milled. Its
//! raw effects (raw or both) last as long as cooked healing would, harm too.

use super::{VegetableDef, VegetableSet, VegetableTag};
use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::{self, RESTORE};
use crate::vitals::Vital;

/// The share of the cooked healing eating it raw gives: a quarter.
pub const RAW_SHARE: (u64, u64) = (1, 4);

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// A serving's kcal ÷ 4: the seconds cooked healing lasts, unrounded, as a
/// fraction (kcal, divided by).
fn quarter(def: &VegetableDef) -> (u64, u64) {
    (u64::from(def.kcal), 4 * u64::from(def.servings.max(1)))
}

/// How long one serving's effects last, in action ticks: as long as it would
/// heal cooked, without the exceptional bonus, in whole seconds rounded down.
pub fn effect_ticks(def: &VegetableDef) -> u64 {
    let (kcal, of) = quarter(def);
    kcal / of * SECOND
}

/// How long one serving would heal cooked, in action ticks: `effect_ticks`,
/// times `fruit::EXCEPTIONAL` if exceptional. Kept for cooking, later.
pub fn cooked_ticks(def: &VegetableDef) -> u64 {
    let (more, of) = match def.exceptional {
        true => fruit::EXCEPTIONAL,
        false => (1, 1),
    };
    let (kcal, quarter) = quarter(def);
    kcal * more / (quarter * of) * SECOND
}

/// How long one serving heals eaten raw, in action ticks: `RAW_SHARE` of the
/// cooked figure (never exceptional) in whole seconds, rounded to the nearest
/// (a turnip's 60 kcal: 4 s); none for grain or what is rarely or never eaten raw.
pub fn raw_ticks(def: &VegetableDef) -> u64 {
    if def.set == VegetableSet::Grain || !def.raw.heals() {
        return 0;
    }
    let (kcal, quarter) = quarter(def);
    let (share, of) = RAW_SHARE;
    let under = quarter * of;
    (kcal * share * 2 + under) / (under * 2) * SECOND
}

/// The Health eating one serving raw gives back in all, in whole points.
pub fn healing(def: &VegetableDef) -> u32 {
    (raw_ticks(def) / SECOND) as u32
}

/// The effects of eating one serving of `def` raw from `now`: its healing (an
/// eaten food's, ended by moving or being hurt; none if it doesn't heal raw)
/// and its raw effects (nothing ends them early). Anyone may eat vegetables.
pub fn eating(def: &VegetableDef, now: u64) -> Vec<Effect> {
    let tag = VegetableTag::of(&def.key);
    let mut effects = Vec::new();
    let raw = raw_ticks(def);
    if raw > 0 {
        effects.push(Effect {
            source: Source::EatenVegetable(tag),
            bonuses: vec![Bonus::flat(
                bonus::Stat::Restore(Vital::Health),
                RESTORE as i32,
            )],
            start: now,
            end: Some(now + raw),
            ends_on: EndsOn {
                moving: true,
                hurt: true,
            },
        });
    }
    let bonuses: Vec<Bonus> = (def.effects.iter())
        .filter(|e| e.when.raw())
        .filter_map(|e| fruit::bonus_of(e.stat, e.change))
        .collect();
    if !bonuses.is_empty() {
        effects.push(Effect {
            source: Source::VegetableEffects(tag),
            bonuses,
            start: now,
            end: Some(now + effect_ticks(def)),
            ends_on: EndsOn::NOTHING,
        });
    }
    effects
}
