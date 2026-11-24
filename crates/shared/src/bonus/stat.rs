//! `Stat`, `Amount` and `Bonus`: what a bonus changes, and by how much.

use serde::{Deserialize, Serialize};

use crate::sheet::Attribute;
use crate::skill::SkillId;
use crate::vitals::Vital;

/// Thousandths of a percent in one percent: `Amount::Percent(10 * PER_CENT)`
/// is 10%.
pub const PER_CENT: i32 = 1000;

/// What a bonus changes. Each has a whole unit its amounts are thousandths of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Stat {
    /// One of the nine, in points.
    Attribute(Attribute),
    /// The most Health or Mana, in points.
    Most(Vital),
    /// A skill's own value, in its hundredths (`skill::VALUE_SCALE`).
    Skill(SkillId),
    /// Only from bonuses: Health or Mana given back over time, in points a
    /// second, on top of refilling and by the same formula (`vitals::Limits`).
    Restore(Vital),
    /// Only from effects: Health or Mana taken away over time (poison), in
    /// points a second, waiting to refill or not, never below 1 point
    /// (`vitals::course`).
    Drain(Vital),
    /// Only from effects: seeing in the dark this many cells round oneself
    /// (`dungeon::darkvision_reach`); only a potion's at a time
    /// (`Source::Potion` of `herb::Stat::Darkvision`).
    Darkvision,
    /// Only from effects: as `Drain`, but it can take Health to 0 and kill
    /// (a deadly fungus's poison, `fungus::poisoning`): while any lasts, all
    /// draining of that one has no floor.
    DeadlyDrain(Vital),
    /// Only from effects, as a percentage: how well it defends against blows
    /// (`combat::defence_power`; a spell's Illusion).
    Defence,
    /// Only from effects, as a percentage: the harm its own blows do (a
    /// spell's Curse, less).
    BlowHarm,
    /// Only from effects, as a percentage: the harm blows do to it (a spell's
    /// Stoneskin, less).
    HarmTaken,
    /// Only from effects, as a percentage: what it resists harmful magic with
    /// (`spell::resistance`; a spell's Warding).
    Resistance,
    /// Only from effects, as a percentage: how fast it acts (a spell's Haste,
    /// more; Slow, less: `Sheet::spell_speed_percent`).
    Speed,
    /// Only from effects: its own light reaching this many cells round it (a
    /// spell's Light: `Sheet::light_reach`).
    Light,
    // New stats go at the end: messages carry one by its place in this list.
}

/// How much: a flat amount in thousandths of the stat's unit, or a share of the
/// stat's base (what it is without bonuses) in thousandths of a percent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Amount {
    Flat(i32),
    Percent(i32),
}

/// One thing a worn item or an effect gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bonus {
    pub stat: Stat,
    pub amount: Amount,
}

impl Bonus {
    /// This many thousandths of the stat's unit.
    pub const fn flat(stat: Stat, milli: i32) -> Self {
        Self {
            stat,
            amount: Amount::Flat(milli),
        }
    }

    /// This many thousandths of a percent of the stat's base.
    pub const fn percent(stat: Stat, milli_percent: i32) -> Self {
        Self {
            stat,
            amount: Amount::Percent(milli_percent),
        }
    }
}
