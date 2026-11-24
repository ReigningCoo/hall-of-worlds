//! What Earth, Poison, Curse, Illusion and Warding leave on a creature for a
//! while (magic plan PR 12), as timed effects (`bonus::Source::Spell`) with
//! bonuses where they change a number: paralysis (held still), Stoneskin
//! (less harm from blows), poison (Health drained), a Curse (its blows do
//! less), an Illusion (more Defence) and Warding (more resistance to magic).
//! Their strength grows with the effect's skill; counts last longer and
//! stack. Numbers are first guesses.

use crate::bonus::{Bonus, Effect as Timed, EndsOn, PER_CENT, Source, Stat};
use crate::combat::skill_percent;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::skill::Skills;
use crate::vitals::{Points, Vital};

use super::effect::Effect;
use super::harm::line_by;
use super::part::Part;
use super::utility::{is_utility, utility_bonuses, utility_seconds};

/// Held still by Earth.
pub const PARALYSED: Part = Part::plain(Effect::Earth);
/// Earth reversed: less harm from blows.
pub const STONESKIN: Part = Part::reversed(Effect::Earth);
/// Poisoned by a spell.
pub const POISONED: Part = Part::plain(Effect::Poison);
/// Cursed: its own blows do less.
pub const CURSED: Part = Part::plain(Effect::Curse);
/// Wrapped in an Illusion: harder to hit.
pub const ILLUSION: Part = Part::plain(Effect::Illusion);
/// Warded: more resistance to magic.
pub const WARDED: Part = Part::plain(Effect::Warding);

/// Seconds Earth holds someone still, each.
pub const PARALYSIS_SECONDS: u64 = 3;
/// Seconds a spell's poison lasts, each.
pub const POISON_SECONDS: u64 = 10;
/// Seconds Stoneskin, a Curse, an Illusion or Warding lasts, each.
pub const LASTING_SECONDS: u64 = 30;
/// How much less Stoneskin and a Curse make blows, each, in percent: this at
/// no skill, twice it at 100%.
pub const LESS_PERCENT: i32 = 15;
/// The most Stoneskin and a Curse take off, in percent.
pub const MOST_LESS_PERCENT: i32 = 75;
/// How much more an Illusion's Defence and Warding's resistance are, each,
/// in percent: this at no skill, twice it at 100%.
pub const MORE_PERCENT: i32 = 25;

/// How long `count` of `part` last, in seconds.
pub fn seconds_of(part: Part, count: u32) -> u64 {
    let each = match part {
        PARALYSED => PARALYSIS_SECONDS,
        POISONED => POISON_SECONDS,
        part if is_utility(part) => utility_seconds(part),
        _ => LASTING_SECONDS,
    };
    each * u64::from(count)
}

/// `count` of a lasting `part` (not poison: `poison`; PR 13's too, `utility`) by a caster with
/// `skills`, from `now`; `None` for a part that leaves nothing lasting.
pub fn lasting(part: Part, count: u32, skills: Option<&Skills>, now: u64) -> Option<Timed> {
    let percent = skills.map_or(0, |s| skill_percent(s.value(part.effect.skill()))) as i32;
    let by = |each: i32| each * (100 + percent) / 100 * count.min(100) as i32;
    let less = |stat| Bonus::percent(stat, -by(LESS_PERCENT).min(MOST_LESS_PERCENT) * PER_CENT);
    let more = |stat| Bonus::percent(stat, by(MORE_PERCENT) * PER_CENT);
    let bonuses = match part {
        PARALYSED => Vec::new(),
        STONESKIN => vec![less(Stat::HarmTaken)],
        CURSED => vec![less(Stat::BlowHarm)],
        ILLUSION => vec![more(Stat::Defence)],
        WARDED => vec![more(Stat::Resistance)],
        part if is_utility(part) => utility_bonuses(part, count, skills),
        _ => return None,
    };
    Some(timed(part, bonuses, seconds_of(part, count), now))
}

/// How much Health `count` poisons take a second before resisting: Heal's
/// line by the Poison skill (12 to 120) over `POISON_SECONDS`, so each adds
/// as much again over as long again.
pub fn poison_per_second(skills: Option<&Skills>) -> Points {
    let line = line_by(Effect::Poison, skills);
    Points(line.milli() / POISON_SECONDS as u32)
}

/// A spell's poison taking `per_second` (after resisting) for `count`
/// poisons' time, from `now`.
pub fn poison(per_second: Points, count: u32, now: u64) -> Timed {
    let milli = per_second.milli().min(i32::MAX as u32) as i32;
    let drain = Bonus::flat(Stat::Drain(Vital::Health), milli);
    timed(POISONED, vec![drain], seconds_of(POISONED, count), now)
}

fn timed(part: Part, bonuses: Vec<Bonus>, seconds: u64, now: u64) -> Timed {
    let ticks = seconds * ACTION_TICKS_PER_SECOND as u64;
    Timed {
        source: Source::Spell(part),
        bonuses,
        start: now,
        end: Some(now + ticks),
        ends_on: EndsOn::NOTHING,
    }
}
