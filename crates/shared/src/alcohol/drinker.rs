//! `Drinker`: what alcohol's rules go by for one creature: its body's mass and
//! its Tolerance. A heavier body wears units off faster and takes more to
//! reach each stage (owner, 2026-11-18); Tolerance raises the thresholds.

use crate::creature::CreatureKind;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::movement::FULL;
use crate::sheet::Sheet;
use crate::skill::SkillId;

/// The Tolerance skill's path: practised by drinking while tipsy or worse.
pub const TOLERANCE: &str = "survival/tolerance";

/// The body mass the rules are set for, in grams: an average human's 70 kg.
pub const AVERAGE_GRAMS: u64 = 70_000;

/// Action ticks to wear off one unit at `AVERAGE_GRAMS`: 10 real minutes
/// (owner, 2026-11-18, a compromise).
pub const UNIT_TICKS: u64 = 10 * 60 * ACTION_TICKS_PER_SECOND as u64;

/// One creature as alcohol sees it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Drinker {
    /// Its body's mass in grams (`CreatureDef::weight`), at least 1.
    pub grams: u64,
    /// Its Tolerance skill's value (`movement::FULL` is 100%), at most 100%.
    pub tolerance: u32,
}

impl Drinker {
    /// A creature of `kind` (the body it is in now) with this sheet's
    /// Tolerance.
    pub fn of(sheet: &Sheet, kind: CreatureKind) -> Self {
        let tolerance = SkillId::find(TOLERANCE).map_or(0, |skill| sheet.skills.value(skill));
        Self::new(u64::from(kind.def().weight), tolerance)
    }

    pub fn new(grams: u64, tolerance: u32) -> Self {
        Self {
            grams: grams.max(1),
            tolerance: tolerance.min(FULL),
        }
    }

    /// A stage's threshold for this drinker, in thousandths of a unit, from
    /// its threshold at `AVERAGE_GRAMS` with no Tolerance: in proportion to
    /// the mass, and up to twice as high at 100% Tolerance.
    pub fn threshold(self, average_milli: u32) -> u32 {
        let by_mass = u64::from(average_milli) * self.grams / AVERAGE_GRAMS;
        let raised = by_mass * u64::from(FULL + self.tolerance) / u64::from(FULL);
        u32::try_from(raised).unwrap_or(u32::MAX)
    }

    /// Thousandths of a unit worn off in `ticks`, rounded down.
    pub fn worn_off(self, ticks: u64) -> u64 {
        let per = UNIT_TICKS * AVERAGE_GRAMS / 1000;
        u64::try_from(u128::from(ticks) * u128::from(self.grams) / u128::from(per))
            .unwrap_or(u64::MAX)
    }

    /// The fewest ticks that wear off `milli` thousandths of a unit.
    pub fn ticks_to_wear(self, milli: u64) -> u64 {
        let per = UNIT_TICKS * AVERAGE_GRAMS / 1000;
        u64::try_from((u128::from(milli) * u128::from(per)).div_ceil(u128::from(self.grams)))
            .unwrap_or(u64::MAX)
    }
}
