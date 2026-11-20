//! A fungus's poison as effects (owner, fungi decisions 8, 9 and 11): when it
//! starts (`onset_ticks`, game time), how long, a deadly one draining Health
//! to death unless healed (halved on Easy), the amatoxins' false recovery,
//! and whether it is recognised before it starts (`recognised`).

use super::{FungusDef, FungusKnowledge, FungusTag, Onset, Poison, Severity, poison_ticks};
use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fruit::bonus_of;
use crate::herb::Stat;
use crate::vitals::{PER_POINT, Vital};
use crate::world::GAME_MINUTE_SECONDS;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

/// A game minute in action ticks.
pub const GAME_MINUTE: u64 = GAME_MINUTE_SECONDS * SECOND;

/// A false recovery's first sickness lasts this long, then eases for
/// `EASED_TICKS` before the worst. First guesses.
pub const SICK_TICKS: u64 = 30 * SECOND;
pub const EASED_TICKS: u64 = 60 * GAME_MINUTE;

/// A deadly poison drains this many times its eater's most Health over its
/// time (Easy half that), so it kills partway through unless healed: on
/// Normal in a third of it, on Easy in two thirds.
pub const DEADLY_TIMES: u32 = 3;

/// How soon after eating it starts, in action ticks (decision 8): soon at
/// once, delayed about 10 game minutes, late a game hour.
pub fn onset_ticks(onset: Onset) -> u64 {
    match onset {
        Onset::Soon => 0,
        Onset::Delayed => 10 * GAME_MINUTE,
        Onset::Late => 60 * GAME_MINUTE,
    }
}

/// Who is poisoned, for a deadly poison's drain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Eater {
    /// Their most Health, in whole points.
    pub most_health: u32,
    /// Playing on Easy: a deadly drain is halved (still deadly).
    pub easy: bool,
}

/// A deadly drain for this eater, in thousandths of a point a second over
/// `ticks`.
pub fn deadly_drain(eater: Eater, ticks: u64) -> u32 {
    let seconds = (ticks / SECOND).max(1);
    let total = u64::from(eater.most_health) * u64::from(PER_POINT) * u64::from(DEADLY_TIMES);
    let drain = (total / seconds) >> u8::from(eater.easy);
    drain.clamp(1, u64::from(u32::MAX)) as u32
}

/// The poison's effects as bonuses; a deadly one's Health as a deadly drain.
fn bonuses(poison: &Poison, deadly: Option<u32>) -> Vec<Bonus> {
    let mut bonuses: Vec<Bonus> = (poison.effects.iter())
        .filter(|e| deadly.is_none() || e.stat != Stat::Health)
        .filter_map(|e| bonus_of(e.stat, e.change))
        .collect();
    if let Some(drain) = deadly {
        let drain = i32::try_from(drain).unwrap_or(i32::MAX);
        bonuses.push(Bonus::flat(bonus::Stat::DeadlyDrain(Vital::Health), drain));
    }
    bonuses
}

fn effect(tag: FungusTag, bonuses: Vec<Bonus>, start: u64, ticks: u64) -> Effect {
    Effect {
        source: Source::FungusPoison(tag),
        bonuses,
        start,
        end: Some(start + ticks),
        ends_on: EndsOn::NOTHING,
    }
}

/// `def`'s poison brought on at `now`: one effect from its onset for its
/// severity's time (`poison_ticks`), or with a false recovery a short
/// sickness, an easing, then the worst. Some start later than `now`.
pub fn poisoning(def: &FungusDef, poison: &Poison, now: u64, eater: Eater) -> Vec<Effect> {
    let tag = FungusTag::of(&def.key);
    let start = now + onset_ticks(poison.onset);
    let ticks = poison_ticks(poison.severity);
    let deadly = (poison.severity == Severity::Deadly).then(|| deadly_drain(eater, ticks));
    if deadly.is_some() && poison.false_recovery {
        let worst = start + SICK_TICKS + EASED_TICKS;
        return vec![
            effect(tag, bonuses(poison, None), start, SICK_TICKS),
            effect(tag, bonuses(poison, deadly), worst, ticks),
        ];
    }
    vec![effect(tag, bonuses(poison, deadly), start, ticks)]
}

/// Whether a poison eaten is recognised before it starts (decision 8): its
/// poison known (`FungusKnowledge::knows_poison`: suffered, or the fungus
/// known by name), or a check passed (`roll` 0 to 99 below `skill`, the
/// better of the Healing and Mycology skills' percents, never more than 95).
/// No Medicine skill yet; magic later.
pub fn recognised(knowledge: &FungusKnowledge, def: &FungusDef, skill: u32, roll: u32) -> bool {
    knowledge.knows_poison(&def.key) || roll < skill.min(95)
}
