//! Designed spells' rules (magic plan, `docs/context/magic-system.md`): a
//! spell's steps (wheres with filters, effects in order), checking a design
//! against the caster's skills, its Mana cost, Void's reversal, the
//! combination table, automatic casting and automatic names. See
//! `docs/context/shared/spell.md`.

mod auto_cast;
mod check;
mod combination;
mod cost;
mod design;
mod effect;
mod filters;
mod name;
mod part;
mod place;

pub use auto_cast::{Aim, AutoCast, auto_cast};
pub use check::{DesignError, check, is_unlocked, unlocked_wheres};
pub use combination::{COMBINATIONS, Combination, Mixture, Outcome, outcomes};
pub use cost::{
    EFFECT_BASE, FORM_SAVING, LEAST, effect_cost, effect_step_cost, spell_cost, step_cost,
    where_step_cost,
};
pub use design::{EffectStep, MAX_STEPS, Spell, Step, WhereStep};
pub use effect::Effect;
pub use filters::Filters;
pub use name::auto_name;
pub use part::{Part, parts};
pub use place::Where;

#[cfg(test)]
mod tests;
