//! Designed spells' rules (magic plan, `docs/context/magic-system.md`): a
//! spell's steps (wheres with filters, effects in order), checking a design
//! against the caster's skills, its Mana cost, Void's reversal, the
//! combination table, automatic casting, automatic names, what each where
//! step hits, and the spellbook. See
//! `docs/context/shared/spell.md`.

mod auto_cast;
mod beam;
mod book;
mod check;
mod combination;
mod cost;
mod design;
mod effect;
mod filters;
mod hit;
mod hits;
mod line;
mod missile;
mod name;
mod part;
mod place;
mod point;
mod surround;
mod touch;
mod toward;

pub use auto_cast::{Aim, AutoCast, auto_cast};
pub use beam::{BEAM_RANGE, BEAM_SPREAD, beam, beam_headings};
pub use book::{
    BookRefusal, BookSpell, LONGEST_NAME, MOST_SPELLS, Spellbook, heal_spell, tidy_name,
};
pub use check::{DesignError, check, is_unlocked, unlocked_wheres};
pub use combination::{COMBINATIONS, Combination, Mixture, Outcome, outcomes};
pub use cost::{
    EFFECT_BASE, FORM_SAVING, LEAST, effect_cost, effect_step_cost, spell_cost, step_cost,
    where_step_cost,
};
pub use design::{EffectStep, MAX_STEPS, Spell, Step, WhereStep};
pub use effect::Effect;
pub use filters::Filters;
pub use hit::{Creature, Hit, Origin, Side, hit_anyone};
pub use hits::where_hits;
pub use missile::{
    Flight, MISSILE_FASTER, MISSILE_RANGE, MISSILE_SPEED, flight_millis, missile, missile_speed,
};
pub use name::auto_name;
pub use part::{Part, parts};
pub use place::Where;
pub use point::{POINT_RANGE, point};
pub use surround::{SURROUND_RING, SURROUND_SPOTS, surround, surround_reach};
pub use touch::{TOUCH_HALF_ARC, TOUCH_WIDER, touch, touch_half_arc};
pub use toward::Toward;

#[cfg(test)]
mod book_tests;
#[cfg(test)]
mod hits_tests;
#[cfg(test)]
mod tests;
