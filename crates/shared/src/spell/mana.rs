//! Mana as an effect (magic plan PR 13): Mana gives Mana, never more in all
//! than the spell cost (so casting it on yourself is never a gain), and
//! reversed drains it, the caster getting back what was drained up to what
//! the spell cost. Numbers are first guesses.

use crate::combat::skill_percent;
use crate::skill::Skills;
use crate::vitals::Points;

use super::effect::Effect;
use super::harm::line_by;

/// The share of the spell's cost Mana gives at no skill, in percent; all of
/// it at 100%.
pub const MANA_GIVEN_PERCENT: u64 = 50;

/// How much Mana each of `targets` gains from a spell that cost `cost`, by a
/// caster with `skills`: half the cost at no skill, all of it at 100%,
/// shared out between them.
pub fn mana_given(cost: Points, targets: usize, skills: Option<&Skills>) -> Points {
    let percent = skills.map_or(0, |s| skill_percent(s.value(Effect::Mana.skill())));
    let share = MANA_GIVEN_PERCENT + (100 - MANA_GIVEN_PERCENT) * percent.min(100) / 100;
    let total = u64::from(cost.milli()) * share / 100;
    Points((total / targets.max(1) as u64) as u32)
}

/// How much Mana `count` drains take from each target before resisting:
/// Heal's line by the Mana skill (12 to 120) each.
pub fn mana_drained(count: u32, skills: Option<&Skills>) -> Points {
    let each = line_by(Effect::Mana, skills).milli();
    Points(each.saturating_mul(count))
}
