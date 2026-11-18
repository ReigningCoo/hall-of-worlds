//! What eating a fruit does (owner, 2026-11-17): 1 Health a second for a
//! quarter of its kcal in seconds (100 kcal: 25 Health), half as long again if
//! exceptional, and its effects for as long; harmful raw, no healing and only
//! its effects.

use super::{FruitDef, FruitTag};
use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::Stat;
use crate::vitals::{PER_POINT, Vital};

/// Healing a second, in thousandths of a point.
pub const RESTORE: u32 = PER_POINT;
/// How much longer an exceptional fruit heals: 3/2.
pub const EXCEPTIONAL: (u64, u64) = (3, 2);
/// An attribute's change for each step of an effect (1 or 2), in thousandths
/// of a point. First guess, as a raw herb's (`herb::RAW_ATTRIBUTE`).
pub const ATTRIBUTE: i32 = 1000;
/// Health or Mana restored or drained a second for each step of an effect, in
/// thousandths of a point. First guess, as a raw herb's (`herb::RAW_VITAL`).
pub const VITAL: i32 = 500;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// How long eating one serving of `def` heals (and its effects last), in
/// action ticks: a quarter of its kcal in seconds, times `EXCEPTIONAL` if it
/// is, in whole seconds rounded down (an apple's 94 kcal: 23 s). A harmful one's effects last as long as its healing would have.
pub fn ticks(def: &FruitDef) -> u64 {
    let (more, of) = match def.exceptional {
        true => EXCEPTIONAL,
        false => (1, 1),
    };
    let servings = u64::from(def.servings.max(1));
    u64::from(def.kcal) * more / (4 * servings * of) * SECOND
}

/// The Health eating one serving gives back in all, in whole points.
pub fn healing(def: &FruitDef) -> u32 {
    match def.harmful_raw {
        true => 0,
        false => (ticks(def) / SECOND) as u32,
    }
}

/// The effects of eating one serving of `def` from `now`: its healing (an
/// eaten food's, ended by moving or being hurt; none if harmful raw) and its
/// effects (nothing ends them early). Anyone may eat fruit.
pub fn eating(def: &FruitDef, now: u64) -> Vec<Effect> {
    let tag = FruitTag::of(&def.key);
    let end = Some(now + ticks(def));
    let mut effects = Vec::new();
    if !def.harmful_raw {
        effects.push(Effect {
            source: Source::EatenFruit(tag),
            bonuses: vec![Bonus::flat(
                bonus::Stat::Restore(Vital::Health),
                RESTORE as i32,
            )],
            start: now,
            end,
            ends_on: EndsOn {
                moving: true,
                hurt: true,
            },
        });
    }
    let bonuses: Vec<Bonus> = def
        .effects
        .iter()
        .filter_map(|e| bonus_of(e.stat, e.change))
        .collect();
    if !bonuses.is_empty() {
        effects.push(Effect {
            source: Source::FruitEffects(tag),
            bonuses,
            start: now,
            end,
            ends_on: EndsOn::NOTHING,
        });
    }
    effects
}

/// One effect of a fruit (or a vegetable, `vegetable::eating`) as a bonus.
pub(crate) fn bonus_of(stat: Stat, change: i8) -> Option<Bonus> {
    let vital = |vital: Vital| {
        let amount = i32::from(change) * VITAL;
        match amount > 0 {
            true => Bonus::flat(bonus::Stat::Restore(vital), amount),
            false => Bonus::flat(bonus::Stat::Drain(vital), -amount),
        }
    };
    Some(match stat {
        Stat::Health => vital(Vital::Health),
        Stat::Mana => vital(Vital::Mana),
        stat => Bonus::flat(
            bonus::Stat::Attribute(stat.attribute()?),
            i32::from(change) * ATTRIBUTE,
        ),
    })
}
