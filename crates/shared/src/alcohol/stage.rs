//! The stages of drinking (owner, 2026-11-18; thresholds and penalties are
//! Claude's first guesses, to tune): what each does, and which a creature is
//! at any tick.

use serde::{Deserialize, Serialize};

use super::{Alcohol, Drinker};
use crate::bonus::{Bonus, PER_CENT, Stat};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::sheet::{Attribute, Row};
use crate::vitals::{PER_POINT, Vital};

/// Thousandths of a unit to be tipsy at 70 kg with no Tolerance: about a
/// pint of ale.
pub const TIPSY_MILLI: u32 = 3_000;
/// To be drunk: about three pints.
pub const DRUNK_MILLI: u32 = 7_000;
/// To be very drunk, collapsing on getting there.
pub const VERY_DRUNK_MILLI: u32 = 12_000;
/// For alcohol poisoning, which harms.
pub const POISONED_MILLI: u32 = 20_000;

/// How long a collapse lasts, unable to act: a minute.
pub const COLLAPSE_TICKS: u64 = 60 * ACTION_TICKS_PER_SECOND as u64;
/// How long a hangover lasts after the last unit wears off: half an hour.
/// It starts once the drinker is no longer tipsy, so it runs about an hour.
pub const HANGOVER_TICKS: u64 = 30 * 60 * ACTION_TICKS_PER_SECOND as u64;

/// One stage of drinking. New ones go at the end: saved effects and messages
/// carry one by its place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    /// A third slower.
    Tipsy,
    /// Two thirds slower, the Mind row lower, the odd stumble.
    Drunk,
    /// As drunk, worse, stumbling more.
    VeryDrunk,
    /// Fallen down on becoming very drunk or poisoned: unable to act for
    /// `COLLAPSE_TICKS`.
    Collapsed,
    /// Alcohol poisoning: as very drunk, losing Health (never below 1, as
    /// poison).
    Poisoned,
    /// Sobering up after being drunk: the Mind row a little lower.
    Hangover,
}

impl Stage {
    /// How much of its speed a creature keeps, as a percentage; stacking fully
    /// with what it carries (`sheet::Load`).
    pub fn speed_percent(self) -> u32 {
        match self {
            Self::Tipsy => 67,
            Self::Hangover => 100,
            Self::Drunk | Self::VeryDrunk | Self::Collapsed | Self::Poisoned => 34,
        }
    }

    /// One move in how many stumbles, if it stumbles at all.
    pub fn stumble_one_in(self) -> Option<u64> {
        match self {
            Self::Drunk => Some(10),
            Self::VeryDrunk | Self::Poisoned => Some(5),
            Self::Tipsy | Self::Collapsed | Self::Hangover => None,
        }
    }

    /// Its penalties: the Mind row lower (`Row::Mind`), Agility too when very
    /// drunk, and Health drained when poisoned.
    pub fn bonuses(self) -> Vec<Bonus> {
        let (mind, agility) = match self {
            Self::Tipsy => (0, 0),
            Self::Hangover => (10, 0),
            Self::Drunk => (20, 0),
            Self::VeryDrunk | Self::Collapsed | Self::Poisoned => (40, 20),
        };
        let mut bonuses: Vec<Bonus> = Row::Mind
            .attributes()
            .into_iter()
            .chain([Attribute::Agility])
            .map(|attribute| {
                let less = if attribute == Attribute::Agility {
                    agility
                } else {
                    mind
                };
                Bonus::percent(Stat::Attribute(attribute), -less * PER_CENT)
            })
            .filter(|bonus| bonus.amount != crate::bonus::Amount::Percent(0))
            .collect();
        if self == Self::Poisoned {
            bonuses.push(Bonus::flat(Stat::Drain(Vital::Health), PER_POINT as i32));
        }
        bonuses
    }

    /// The stage `milli` thousandths of a unit bring this drinker, if any:
    /// the highest threshold reached.
    pub fn by_units(milli: u32, drinker: Drinker) -> Option<Self> {
        [
            (POISONED_MILLI, Self::Poisoned),
            (VERY_DRUNK_MILLI, Self::VeryDrunk),
            (DRUNK_MILLI, Self::Drunk),
            (TIPSY_MILLI, Self::Tipsy),
        ]
        .into_iter()
        .find(|&(threshold, _)| milli >= drinker.threshold(threshold))
        .map(|(_, stage)| stage)
    }
}

impl Alcohol {
    /// Its stage at `now`, if any: collapsed for a while after becoming very
    /// drunk or poisoned, then by the units left, then hungover until
    /// `HANGOVER_TICKS` after the last wears off if it was drunk.
    pub fn stage(&self, now: u64, drinker: Drinker) -> Option<Stage> {
        if now < self.collapsed_until {
            return Some(Stage::Collapsed);
        }
        if let Some(stage) = Stage::by_units(self.milli_at(now, drinker), drinker) {
            return Some(stage);
        }
        let was_drunk = self.peak >= drinker.threshold(DRUNK_MILLI);
        (was_drunk && now < self.sober_at(drinker) + HANGOVER_TICKS).then_some(Stage::Hangover)
    }

    /// The first tick after `now` at which its stage differs, if it ever
    /// does: the end of a collapse, units falling below a threshold, or a
    /// hangover ending.
    pub fn next_change(&self, now: u64, drinker: Drinker) -> Option<u64> {
        let mut ticks: Vec<u64> = [TIPSY_MILLI, DRUNK_MILLI, VERY_DRUNK_MILLI, POISONED_MILLI]
            .into_iter()
            .filter_map(|threshold| self.below_at(drinker.threshold(threshold), drinker))
            .chain([
                self.collapsed_until,
                self.sober_at(drinker).saturating_add(HANGOVER_TICKS),
            ])
            .filter(|&tick| tick > now)
            .collect();
        ticks.sort_unstable();
        let stage = self.stage(now, drinker);
        ticks
            .into_iter()
            .find(|&tick| self.stage(tick, drinker) != stage)
    }
}
