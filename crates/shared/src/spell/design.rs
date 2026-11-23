//! A spell's design: its steps, each a where (with filters in the first step
//! only) or effects in order. What the client sends and the spellbook keeps.

use serde::{Deserialize, Serialize};

use super::effect::Effect;
use super::filters::Filters;
use super::place::Where;

/// At most this many steps: a safety net, as Mana is the real limit.
pub const MAX_STEPS: usize = 10;

/// A whole spell: its steps in order.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Spell {
    pub steps: Vec<Step>,
}

/// One step of a spell.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    /// Where the next step happens.
    Where(WhereStep),
    /// What happens to what the step before hit (or the caster, first).
    Effects(EffectStep),
}

/// One where, stacked `count` times, and (in the first step only) filters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhereStep {
    pub place: Where,
    pub count: u8,
    pub filters: Filters,
}

/// Effects in order, each stacked a number of times (Heal ×2 is two Heals).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EffectStep {
    pub effects: Vec<(Effect, u8)>,
}

impl Spell {
    /// A spell of these steps.
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }

    /// The first step's where, if the spell starts with one: the only where
    /// the caster's target matters to. `None` happens on the caster.
    pub fn first_where(&self) -> Option<&WhereStep> {
        match self.steps.first() {
            Some(Step::Where(step)) => Some(step),
            _ => None,
        }
    }

    /// Every effect step, in order.
    pub fn effect_steps(&self) -> impl Iterator<Item = &EffectStep> {
        self.steps.iter().filter_map(|step| match step {
            Step::Effects(step) => Some(step),
            Step::Where(_) => None,
        })
    }
}

impl Step {
    /// A where step with no filters.
    pub const fn at(place: Where, count: u8) -> Self {
        Self::Where(WhereStep {
            place,
            count,
            filters: Filters::NONE,
        })
    }

    /// A where step with filters.
    pub const fn filtered(place: Where, count: u8, filters: Filters) -> Self {
        Self::Where(WhereStep {
            place,
            count,
            filters,
        })
    }

    /// An effect step of these effects and counts.
    pub fn effects(effects: &[(Effect, u8)]) -> Self {
        Self::Effects(EffectStep {
            effects: effects.to_vec(),
        })
    }
}
