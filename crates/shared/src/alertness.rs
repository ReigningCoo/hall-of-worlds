//! Alertness (mob behaviour plan PR 7): whether a creature has noticed anyone.
//! Unaware, it has neither seen nor heard anything; Alert, it heard something
//! and turns towards it after a moment (its reaction, by its Mind row);
//! Aware, it sees someone. A blow on one caught off guard (Unaware, or Alert
//! but not yet turned) is a critical (`combat::Critical`). The server keeps each mob's (`Wariness`); players are
//! only told when a mob is Alert to them, for the sneaking eye.

mod wariness;

pub use wariness::Wariness;

use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::sheet::{Attributes, Row};

/// How far a creature has noticed anyone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Alertness {
    /// Neither seen nor heard anyone.
    #[default]
    Unaware,
    /// Heard something, and doesn't see who.
    Alert,
    /// Sees someone.
    Aware,
}

const TICKS: u64 = ACTION_TICKS_PER_SECOND as u64;

/// The moment before turning to a noise, with a Mind row at the median (100
/// each): a second and a half.
pub const REACTION_TICKS: u64 = 3 * TICKS / 2;

/// The quickest reaction, however sharp: half a second.
pub const QUICKEST_TICKS: u64 = TICKS / 2;

/// The slowest, however dull: three seconds.
pub const SLOWEST_TICKS: u64 = 3 * TICKS;

/// How long it stays Alert after the last noise heard or the last sight of
/// anyone: ten seconds.
pub const ALERT_TICKS: u64 = 10 * TICKS;

/// The moment `attributes` take to turn to a noise: `REACTION_TICKS` at a Mind
/// row averaging 100, shorter the higher it is, between `QUICKEST_TICKS` and
/// `SLOWEST_TICKS`.
pub fn reaction_ticks(attributes: &Attributes) -> u64 {
    let mind: u64 = (Row::Mind.attributes().iter())
        .map(|&which| u64::from(attributes.get(which)))
        .sum();
    // Averaging 100 is 300 in all.
    let ticks = (REACTION_TICKS * 300)
        .checked_div(mind)
        .unwrap_or(SLOWEST_TICKS);
    ticks.clamp(QUICKEST_TICKS, SLOWEST_TICKS)
}

#[cfg(test)]
mod tests;
