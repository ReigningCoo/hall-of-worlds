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

/// At most this many wheres merged in one step (the first and `more`): a
/// safety net, as Mana is the real limit.
pub const MAX_WHERES: usize = 5;

/// A where step: its first where, stacked `count` times, (in the first step
/// only) filters, and any more wheres merged after it (magic plan PR 10),
/// each leading on from what the one before hit. The effects after the step
/// land on everything any of them hit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhereStep {
    pub place: Where,
    pub count: u8,
    pub filters: Filters,
    /// The wheres merged after the first, in order; empty for one where (and
    /// in saves from before merging, which load as they were).
    #[serde(default)]
    pub more: Vec<WherePart>,
}

/// One where in a step, stacked `count` times.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WherePart {
    pub place: Where,
    pub count: u8,
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

impl WhereStep {
    /// Every where in the step, the first first.
    pub fn parts(&self) -> impl Iterator<Item = WherePart> + '_ {
        let first = WherePart {
            place: self.place,
            count: self.count,
        };
        std::iter::once(first).chain(self.more.iter().copied())
    }

    /// How many wheres the step holds: 1 and its `more`.
    pub fn len(&self) -> usize {
        1 + self.more.len()
    }

    /// Never empty: a where step always has its first where.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// The last where in the step: the one its effects follow.
    pub fn last(&self) -> WherePart {
        self.parts().last().unwrap_or(WherePart {
            place: self.place,
            count: self.count,
        })
    }
}

impl Step {
    /// A where step with no filters.
    pub const fn at(place: Where, count: u8) -> Self {
        Self::filtered(place, count, Filters::NONE)
    }

    /// A where step with filters.
    pub const fn filtered(place: Where, count: u8, filters: Filters) -> Self {
        Self::Where(WhereStep {
            place,
            count,
            filters,
            more: Vec::new(),
        })
    }

    /// A where step of several wheres merged, in order, with filters; `None`
    /// with no wheres.
    pub fn merged(parts: &[WherePart], filters: Filters) -> Option<Self> {
        let (first, more) = parts.split_first()?;
        Some(Self::Where(WhereStep {
            place: first.place,
            count: first.count,
            filters,
            more: more.to_vec(),
        }))
    }

    /// An effect step of these effects and counts.
    pub fn effects(effects: &[(Effect, u8)]) -> Self {
        Self::Effects(EffectStep {
            effects: effects.to_vec(),
        })
    }
}
