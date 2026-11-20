//! What eating a fungus raw does (owner, 2026-11-19, decision 6). Healing as
//! vegetables': only what people eat raw commonly or sometimes heals, for a
//! quarter of a serving's kcal ÷ 4 in seconds; its raw food effects last as
//! long as cooked healing would. A poison that eating brings on (decision 8)
//! starts by its onset, for a while by its severity (`poisoning`); one that
//! alcohol brings on (the common ink cap, decision 11) gives a bonus instead,
//! for `WINDOW_TICKS`, while drinking would bring it on (`with_alcohol`). Its
//! herb effects are for Alchemy (PR 8), not eating.

use super::poisoning::{Eater, GAME_MINUTE, poisoning};
use super::{FungusDef, FungusTag, Severity, Trigger};
use crate::bonus::{self, Bonus, Effect, Effects, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::{RESTORE, bonus_of};
use crate::herb::MINUTES_PER_DAY;
use crate::sheet::Attribute;
use crate::vegetable::RAW_SHARE;
use crate::vitals::Vital;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// How long after eating a fungus alcohol brings on its poison: 3 game days
/// (decision 11), its bonus lasting as long.
pub const WINDOW_TICKS: u64 = 3 * MINUTES_PER_DAY * GAME_MINUTE;

/// That bonus, so eating one is a real choice (decision 11): Toughness and
/// Will, a point each.
pub const WINDOW_BONUS: [Attribute; 2] = [Attribute::Toughness, Attribute::Will];

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
/// its raw food effects, its poison if eating brings it on (some starting
/// later, `poisoning`), or its window if alcohol does (nothing ends those
/// early). Anyone may eat fungi.
pub fn eating(def: &FungusDef, now: u64, eater: Eater) -> Vec<Effect> {
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
    match def.poison.as_ref() {
        Some(poison) if poison.when_eaten() => effects.extend(poisoning(def, poison, now, eater)),
        Some(poison) if poison.trigger == Trigger::WithAlcohol => effects.push(Effect {
            source: Source::FungusWindow(tag),
            bonuses: (WINDOW_BONUS.iter())
                .map(|&a| Bonus::flat(bonus::Stat::Attribute(a), crate::fruit::ATTRIBUTE))
                .collect(),
            start: now,
            end: Some(now + WINDOW_TICKS),
            ends_on: EndsOn::NOTHING,
        }),
        _ => {}
    }
    effects
}

/// Alcohol drunk at `now` with these effects on: the poison of every fungus
/// whose window lasts then (the common ink cap's flushing and sickness).
pub fn with_alcohol(effects: &Effects, now: u64, eater: Eater) -> Vec<Effect> {
    (effects.0.iter())
        .filter(|effect| effect.lasts(now))
        .filter_map(|effect| match effect.source {
            Source::FungusWindow(tag) => super::Fungi::built_in().by_tag(tag),
            _ => None,
        })
        .filter_map(|def| Some((def, def.poison.as_ref()?)))
        .filter(|(_, poison)| poison.trigger == Trigger::WithAlcohol)
        .flat_map(|(def, poison)| poisoning(def, poison, now, eater))
        .collect()
}
