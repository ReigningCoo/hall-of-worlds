//! `Alcohol`: the units in one creature (on its `Sheet`, saved), stored like
//! `energy` as a value and the tick it was right at, so any later tick is a
//! formula and time away (offline too, by the server's action clock) wears
//! them off.

use serde::{Deserialize, Serialize};

use super::Drinker;
use super::stage::{POISONED_MILLI, Stage, TIPSY_MILLI, VERY_DRUNK_MILLI};
use crate::alcohol::COLLAPSE_TICKS;

/// The alcohol in one creature.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alcohol {
    /// Thousandths of a unit (1 unit is 10 ml of pure alcohol) at `at`.
    pub milli: u32,
    /// The action tick `milli` was right at.
    pub at: u64,
    /// The most it has held since it was last below tipsy: drunk or worse
    /// brings a hangover.
    pub peak: u32,
    /// The action tick a collapse ends at (0 for none).
    pub collapsed_until: u64,
}

impl Alcohol {
    /// Thousandths of a unit left at `now`.
    pub fn milli_at(&self, now: u64, drinker: Drinker) -> u32 {
        let worn = drinker.worn_off(now.saturating_sub(self.at));
        u32::try_from(u64::from(self.milli).saturating_sub(worn)).unwrap_or(0)
    }

    /// The tick the last of it wears off.
    pub fn sober_at(&self, drinker: Drinker) -> u64 {
        self.at + drinker.ticks_to_wear(self.milli.into())
    }

    /// The first tick it is below `milli` thousandths of a unit, if it is
    /// at or above it at `at`.
    pub(super) fn below_at(&self, milli: u32, drinker: Drinker) -> Option<u64> {
        (self.milli >= milli)
            .then(|| self.at + drinker.ticks_to_wear(u64::from(self.milli - milli) + 1))
    }

    /// Drinks `milli` more thousandths of a unit at `now`. Becoming very
    /// drunk or poisoned collapses it for `COLLAPSE_TICKS`. Below tipsy, what
    /// it held before no longer counts towards a hangover.
    pub fn drink(&mut self, milli: u32, now: u64, drinker: Drinker) {
        let before = self.milli_at(now, drinker);
        if before < drinker.threshold(TIPSY_MILLI) {
            self.peak = 0;
        }
        let after = before.saturating_add(milli);
        let crossed = |threshold| {
            let threshold = drinker.threshold(threshold);
            before < threshold && after >= threshold
        };
        if crossed(VERY_DRUNK_MILLI) || crossed(POISONED_MILLI) {
            self.collapsed_until = self.collapsed_until.max(now + COLLAPSE_TICKS);
        }
        self.milli = after;
        self.at = now;
        self.peak = self.peak.max(after);
    }

    /// Whether it is collapsed at `now`.
    pub fn collapsed(&self, now: u64) -> bool {
        now < self.collapsed_until
    }

    /// Its stage by units alone at `now` (no collapse or hangover).
    pub fn by_units(&self, now: u64, drinker: Drinker) -> Option<Stage> {
        Stage::by_units(self.milli_at(now, drinker), drinker)
    }
}
