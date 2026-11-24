//! `Totals`: every bonus added up, stat by stat, and applied to a base.

use std::collections::BTreeMap;

use super::MILLI;
use super::stat::{Amount, Bonus, PER_CENT, Stat};
use crate::sheet::{Attribute, Attributes};
use crate::vitals::Vital;

/// What one stat's bonuses come to: the flat ones in thousandths of its unit,
/// the percentages in thousandths of a percent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sum {
    pub flat: i64,
    pub percent: i64,
}

/// Everything a creature's bonuses come to now. Always built from scratch
/// (`of`), never changed a piece at a time.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Totals(BTreeMap<Stat, Sum>);

impl Totals {
    /// All these bonuses added up.
    pub fn of<'a>(bonuses: impl IntoIterator<Item = &'a Bonus>) -> Self {
        let mut totals = BTreeMap::<Stat, Sum>::new();
        for bonus in bonuses {
            let sum = totals.entry(bonus.stat).or_default();
            match bonus.amount {
                Amount::Flat(milli) => sum.flat += i64::from(milli),
                Amount::Percent(milli) => sum.percent += i64::from(milli),
            }
        }
        Self(totals)
    }

    /// What this stat's bonuses come to (nothing if none).
    pub fn sum(&self, stat: Stat) -> Sum {
        self.0.get(&stat).copied().unwrap_or_default()
    }

    /// What the bonuses add to a base of `base` thousandths: the flat ones, then
    /// the percentages of the base. Not rounded.
    pub fn added_milli(&self, stat: Stat, base: i64) -> i64 {
        let sum = self.sum(stat);
        sum.flat + base * sum.percent / (100 * i64::from(PER_CENT))
    }

    /// A whole-numbered base with its bonuses, rounded once to the nearest.
    pub fn whole(&self, stat: Stat, base: i64) -> i64 {
        let base = base * MILLI;
        round_milli(base + self.added_milli(stat, base))
    }

    /// The nine with their bonuses, each at least 1.
    pub fn attributes(&self, base: &Attributes) -> Attributes {
        let mut with = *base;
        for which in Attribute::ALL {
            let value = self.whole(Stat::Attribute(which), i64::from(base.get(which)));
            with.set(which, value.clamp(1, i64::from(u16::MAX)) as u16);
        }
        with
    }

    /// Health or Mana given back over time, in thousandths of a point a second
    /// (never below 0).
    pub fn restoring(&self, vital: Vital) -> u32 {
        self.sum(Stat::Restore(vital))
            .flat
            .clamp(0, i64::from(u32::MAX)) as u32
    }

    /// Health or Mana taken away over time (poison), in thousandths of a point
    /// a second (never below 0).
    pub fn draining(&self, vital: Vital) -> u32 {
        let flat = self.sum(Stat::Drain(vital)).flat + self.sum(Stat::DeadlyDrain(vital)).flat;
        flat.clamp(0, i64::from(u32::MAX)) as u32
    }

    /// Whether any draining of this one is deadly (`Stat::DeadlyDrain`), so
    /// draining can take it to 0.
    pub fn deadly(&self, vital: Vital) -> bool {
        self.sum(Stat::DeadlyDrain(vital)).flat > 0
    }

    /// How far in cells one sees in the dark (Darkvision): 0 without.
    pub fn darkvision(&self) -> u32 {
        (self.sum(Stat::Darkvision).flat / MILLI).clamp(0, i64::from(u32::MAX)) as u32
    }

    /// How far in cells its own light reaches (a spell's Light): 0 without.
    pub fn light(&self) -> u32 {
        (self.sum(Stat::Light).flat / MILLI).clamp(0, i64::from(u32::MAX)) as u32
    }

    /// A stat kept only as a percentage of whatever it changes (`Defence`,
    /// `BlowHarm`, `HarmTaken`, `Resistance`), per mille: 1000 with none,
    /// never below 0.
    pub fn per_mille(&self, stat: Stat) -> u64 {
        let percent = self.sum(stat).percent;
        (1000 + percent / 100).max(0) as u64
    }

    /// Whether there are no bonuses at all.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Thousandths to the nearest whole, halves up.
pub fn round_milli(milli: i64) -> i64 {
    (milli + MILLI / 2).div_euclid(MILLI)
}
