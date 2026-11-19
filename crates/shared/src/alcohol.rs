//! Drinking alcohol (drinks plan PR 7, `docs/context/drinks.md` decisions 21
//! to 25): units of alcohol on the sheet, worn off over real time by body
//! mass; the stages they bring (tipsy, drunk, very drunk with a collapse,
//! poisoning, and a hangover after), each kept as one effect so it shows on
//! the effect bar and its penalties count like any bonus; how much slower
//! each stage is; the odd stumble, from a hash both sides work out; and the
//! Tolerance skill raising the thresholds.

mod drinker;
mod drinking;
mod effect;
mod stage;
mod state;
mod stumble;

pub use drinker::{AVERAGE_GRAMS, Drinker, TOLERANCE, UNIT_TICKS};
pub use drinking::{TOLERANCE_PRACTICE_PER_UNIT, tolerance_practice, units_in};
pub use effect::stage_effect;
pub use stage::{
    COLLAPSE_TICKS, DRUNK_MILLI, HANGOVER_TICKS, POISONED_MILLI, Stage, TIPSY_MILLI,
    VERY_DRUNK_MILLI,
};
pub use state::Alcohol;
pub use stumble::{Veer, stumble, veered_move, veered_step};

#[cfg(test)]
mod tests;
