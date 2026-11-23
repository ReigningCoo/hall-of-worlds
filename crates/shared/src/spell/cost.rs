//! A spell's Mana cost, in thousandths of a point as Mana is kept (magic plan,
//! Mana cost). An effect costs 5 less ∛(its skill %), never below 1; an effect
//! step its effects added up, +1 for each after the first; a where step tier ×
//! count, +1 for each extra, less up to 50% by its Form skill, never below 1;
//! filters are free; the spell its steps multiplied together.

use super::design::{EffectStep, Spell, Step, WhereStep};
use super::effect::Effect;
use crate::skill::{SkillId, Skills};
use crate::vitals::{PER_POINT, Points};

/// What an effect costs with no skill: 5 points.
pub const EFFECT_BASE: u64 = 5 * PER_POINT as u64;
/// The least anything costs: a point.
pub const LEAST: u64 = PER_POINT as u64;
/// The most a where's Form skill takes off at 100%, per mille: half.
pub const FORM_SAVING: u64 = 500;

/// A skill's value in hundredths of a percent, up to 100% (10,000).
fn hundredths(skills: Option<&Skills>, id: SkillId) -> u64 {
    skills.map_or(0, |skills| u64::from(skills.value(id)).min(10_000))
}

/// The whole number cube root, rounded down.
fn cube_root(n: u64) -> u64 {
    let (mut low, mut high) = (0u64, 2_642_245u64);
    while low < high {
        let middle = (low + high).div_ceil(2);
        if middle * middle * middle <= n {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    low
}

/// One of an effect, in thousandths: 5 less the cube root of its skill's
/// percentage, never below 1 (5 at 0%, 3 at 8%, 2 at 27%, 1 from 64%).
pub fn effect_cost(effect: Effect, skills: Option<&Skills>) -> u64 {
    // ∛(hundredths ÷ 100) in thousandths is ∛(hundredths × 10⁷).
    let root = cube_root(hundredths(skills, effect.skill()) * 10_000_000);
    EFFECT_BASE.saturating_sub(root).max(LEAST)
}

/// An effect step: its effects (each as many times as stacked) added up, plus
/// a point for each after the first. Void costs as any effect.
pub fn effect_step_cost(step: &EffectStep, skills: Option<&Skills>) -> u64 {
    let mut total = 0;
    let mut units = 0u64;
    for &(effect, count) in &step.effects {
        total += effect_cost(effect, skills) * u64::from(count);
        units += u64::from(count);
    }
    total + units.saturating_sub(1) * LEAST
}

/// A where step: tier × count, plus a point for each extra, less up to half
/// by its Form skill, never below 1. Filters are free.
pub fn where_step_cost(step: &WhereStep, skills: Option<&Skills>) -> u64 {
    let count = u64::from(step.count);
    let base = (u64::from(step.place.tier()) * count + count.saturating_sub(1)) * LEAST;
    let saving = FORM_SAVING * hundredths(skills, step.place.skill()) / 10_000;
    (base * (1000 - saving) / 1000).max(LEAST)
}

/// One step's cost, in thousandths.
pub fn step_cost(step: &Step, skills: Option<&Skills>) -> u64 {
    match step {
        Step::Where(step) => where_step_cost(step, skills),
        Step::Effects(step) => effect_step_cost(step, skills),
    }
}

/// The whole spell: its steps multiplied together, as much as Mana can hold.
pub fn spell_cost(spell: &Spell, skills: Option<&Skills>) -> Points {
    let mut total: u128 = u128::from(LEAST);
    for step in &spell.steps {
        total = total * u128::from(step_cost(step, skills)) / u128::from(LEAST);
        total = total.min(u128::from(u32::MAX));
    }
    if spell.steps.is_empty() {
        total = 0;
    }
    Points(total as u32)
}
