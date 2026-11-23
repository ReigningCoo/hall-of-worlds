//! Checking a design against the rules and the caster's skills: what the
//! server refuses and the panel shows as wrong. Unlocks: ⌊∛(Magic% × 6)⌋
//! wheres, in `Where::ALL`'s order; filters and effects are known from the
//! start.

use serde::{Deserialize, Serialize};

use super::design::{MAX_STEPS, MAX_WHERES, Spell, Step};
use super::place::Where;
use crate::skill::{SkillId, Skills, VALUE_SCALE};

/// What is wrong with a design, the first thing found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignError {
    /// No steps at all.
    Empty,
    /// More than `MAX_STEPS`.
    TooManySteps,
    /// No effect anywhere: it would do nothing.
    NoEffect,
    /// A where step with nothing after it does nothing.
    WhereAtEnd,
    /// An effect step holding no effects.
    EmptyStep,
    /// A where or an effect stacked 0 times.
    ZeroCount,
    /// More than `MAX_WHERES` wheres merged in one step.
    TooManyWheres,
    /// Filters in a step other than the first.
    FilterNotFirst,
    /// A where the caster hasn't unlocked.
    Locked(Where),
}

/// How many wheres a caster can use, 0 to 5: ⌊∛(Magic% × 6)⌋, Magic being
/// the `magic` node's own value (`None` for a creature without skills).
pub fn unlocked_wheres(skills: Option<&Skills>) -> usize {
    let Some(skills) = skills else { return 0 };
    let magic = SkillId::find("magic").expect("the skill tree has Magic");
    // n³ ≤ value / 100 × 6, in whole numbers: n³ × 100 ≤ value × 6.
    let six_times = u64::from(skills.value(magic)) * 6;
    let mut count = 0;
    while count < Where::ALL.len() {
        let next = (count + 1) as u64;
        if next * next * next * u64::from(VALUE_SCALE) > six_times {
            break;
        }
        count += 1;
    }
    count
}

/// Whether a caster can use this where.
pub fn is_unlocked(place: Where, skills: Option<&Skills>) -> bool {
    (place as usize) < unlocked_wheres(skills)
}

/// Whether a design can be cast by someone of these skills.
pub fn check(spell: &Spell, skills: Option<&Skills>) -> Result<(), DesignError> {
    if spell.steps.is_empty() {
        return Err(DesignError::Empty);
    }
    if spell.steps.len() > MAX_STEPS {
        return Err(DesignError::TooManySteps);
    }
    if spell.effect_steps().next().is_none() {
        return Err(DesignError::NoEffect);
    }
    if matches!(spell.steps.last(), Some(Step::Where(_))) {
        return Err(DesignError::WhereAtEnd);
    }
    let unlocked = unlocked_wheres(skills);
    for (index, step) in spell.steps.iter().enumerate() {
        match step {
            Step::Where(step) => {
                if step.len() > MAX_WHERES {
                    return Err(DesignError::TooManyWheres);
                }
                if step.parts().any(|part| part.count == 0) {
                    return Err(DesignError::ZeroCount);
                }
                if index > 0 && step.filters.any() {
                    return Err(DesignError::FilterNotFirst);
                }
                if let Some(part) = step.parts().find(|part| part.place as usize >= unlocked) {
                    return Err(DesignError::Locked(part.place));
                }
            }
            Step::Effects(step) => {
                if step.effects.is_empty() {
                    return Err(DesignError::EmptyStep);
                }
                if step.effects.iter().any(|(_, count)| *count == 0) {
                    return Err(DesignError::ZeroCount);
                }
            }
        }
    }
    Ok(())
}
