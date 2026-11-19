//! Drinking a potion (brewing decisions 17 to 19): Health and Mana given (or
//! taken) at once, 5 a step, as ointment's; each attribute's dose its own
//! effect (`Source::Potion` of its stat), lasting by the bottle's quality, so a
//! new potion's dose on a stat replaces the old one there and the old potion's
//! other stats carry on.

use super::{applied, duration_ticks};
use crate::bonus::{Effect, Source};
use crate::item::Quality;
use crate::potion::Contents;
use crate::vitals::Vital;

/// What drinking a potion does: an effect for each attribute it moves, and
/// Health or Mana given (or taken, below 0) at once, in thousandths of a point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Drunk {
    pub effects: Vec<Effect>,
    pub vitals: Vec<(Vital, i32)>,
}

/// A potion holding `contents`, of `quality` (the bottle's), drunk at `now`.
pub fn drunk(contents: &Contents, quality: Quality, now: u64) -> Drunk {
    let lasts = duration_ticks(quality);
    let mut drunk = Drunk {
        effects: Vec::new(),
        vitals: Vec::new(),
    };
    for &dose in contents.doses() {
        let one = applied(
            &Contents::new([dose]),
            quality,
            lasts,
            Source::Potion(dose.stat),
            now,
        );
        drunk.effects.extend(one.effect);
        drunk.vitals.extend(one.vitals);
    }
    drunk
}
