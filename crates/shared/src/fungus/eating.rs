//! What eating a fungus raw does (owner, 2026-11-19, decision 6). Healing as
//! vegetables': only what people eat raw commonly or sometimes heals, for a
//! quarter of a serving's kcal ÷ 4 in seconds; its raw food effects last as
//! long as cooked healing would. A poison that eating brings on (decision 8)
//! starts at once for now, for a while by its severity (`poison_ticks`):
//! delayed and hidden onsets come with fungi plan PR 6. Its herb effects are
//! for Alchemy (PR 8), not eating.

use super::{FungusDef, FungusTag, Severity};
use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::{RESTORE, bonus_of};
use crate::vegetable::RAW_SHARE;
use crate::vitals::Vital;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// A serving's kcal ÷ 4: the seconds cooked healing lasts, unrounded, as a
/// fraction (kcal, divided by).
fn quarter(def: &FungusDef) -> (u64, u64) {
    (u64::from(def.kcal), 4 * u64::from(def.servings.max(1)))
}

/// How long one serving's food effects last, in action ticks: as long as it
/// would heal cooked, in whole seconds rounded down.
pub fn effect_ticks(def: &FungusDef) -> u64 {
    let (kcal, of) = quarter(def);
    kcal / of * SECOND
}

/// How long one serving heals eaten raw, in action ticks: `RAW_SHARE` of the
/// cooked figure in whole seconds, rounded to the nearest; none for what is
/// rarely or never eaten raw.
pub fn raw_ticks(def: &FungusDef) -> u64 {
    if !def.raw.heals() {
        return 0;
    }
    let (kcal, quarter) = quarter(def);
    let (share, of) = RAW_SHARE;
    let under = quarter * of;
    (kcal * share * 2 + under) / (under * 2) * SECOND
}

/// The Health eating one serving raw gives back in all, in whole points.
pub fn healing(def: &FungusDef) -> u32 {
    (raw_ticks(def) / SECOND) as u32
}

/// How long a poison eaten lasts, in action ticks, by its severity: 30, 60, 90
/// or 120 seconds. First guesses (an unsuitable food's poison lasts 60 s).
pub fn poison_ticks(severity: Severity) -> u64 {
    let seconds = match severity {
        Severity::Mild => 30,
        Severity::Moderate => 60,
        Severity::Severe => 90,
        Severity::Deadly => 120,
    };
    seconds * SECOND
}

/// The effects of eating one serving of `def` raw from `now`: its healing (an
/// eaten food's, ended by moving or being hurt; none if it doesn't heal raw),
/// its raw food effects, and its poison if eating brings it on (nothing ends
/// those early). Anyone may eat fungi.
pub fn eating(def: &FungusDef, now: u64) -> Vec<Effect> {
    let tag = FungusTag::of(&def.key);
    let mut effects = Vec::new();
    let raw = raw_ticks(def);
    if raw > 0 {
        effects.push(Effect {
            source: Source::EatenFungus(tag),
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
    let bonuses: Vec<Bonus> = (def.food.iter())
        .filter(|e| e.when.raw())
        .filter_map(|e| bonus_of(e.stat, e.change))
        .collect();
    if !bonuses.is_empty() {
        effects.push(Effect {
            source: Source::FungusEffects(tag),
            bonuses,
            start: now,
            end: Some(now + effect_ticks(def)),
            ends_on: EndsOn::NOTHING,
        });
    }
    if let Some(poison) = def.poison.as_ref().filter(|p| p.when_eaten()) {
        let bonuses: Vec<Bonus> = (poison.effects.iter())
            .filter_map(|e| bonus_of(e.stat, e.change))
            .collect();
        effects.push(Effect {
            source: Source::FungusPoison(tag),
            bonuses,
            start: now,
            end: Some(now + poison_ticks(poison.severity)),
            ends_on: EndsOn::NOTHING,
        });
    }
    effects
}
