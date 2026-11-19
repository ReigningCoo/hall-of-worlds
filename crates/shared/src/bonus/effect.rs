//! `Effect`: bonuses laid on a creature for a while or for good, by eating,
//! drinking or dev tools, and what ends them.

use serde::{Deserialize, Serialize};

use super::stat::{Bonus, Stat};
use crate::item::ItemKind;
use crate::vitals::Vital;

/// Where an effect came from. A creature has at most one from each, and one
/// eaten at all: a new one takes its place, so eating a second apple starts it
/// again, and bread after a fruit ends the fruit's (`replaced_by`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Source {
    /// Eating or drinking one of these.
    Eaten(ItemKind),
    /// The debug menu's Bonus (`protocol::DevVitals::Bonus`), and the changes
    /// older saves kept on `Vitals`.
    Dev,
    /// Poison, from eating what doesn't suit its kind (`bonus::poison`): its
    /// own source, so it runs alongside any healing, and poisoning again
    /// starts it again.
    Poison,
    /// A herb eaten raw (`herb::raw_dose`): its own source, so it runs
    /// alongside food, and only the latest herb's counts.
    RawHerb,
    /// Eating a fruit (`fruit::eating`): its healing, food like any eaten.
    EatenFruit(crate::fruit::FruitTag),
    /// A fruit's effects (`fruit::eating`): their own source, alongside food
    /// and herbs, and only the latest fruit's count.
    FruitEffects(crate::fruit::FruitTag),
    /// Eating a vegetable raw (`vegetable::eating`): its healing, food like
    /// any eaten.
    EatenVegetable(crate::vegetable::VegetableTag),
    /// A vegetable's effects (`vegetable::eating`): their own source, as a
    /// fruit's, and only the latest vegetable's count.
    VegetableEffects(crate::vegetable::VegetableTag),
    /// Ointment rubbed on oneself (`brewing::skin_effect`): its own source,
    /// alongside food and herbs, and only the latest ointment's counts.
    Ointment,
    /// A potion's dose on one stat (`brewing::drunk`): its own source per
    /// stat, so a new potion's dose there replaces the old one's while the
    /// old potion's other stats carry on (brewing decision 19).
    Potion(crate::herb::Stat),
    /// Juice drunk (`juice::drink_effect`): Mana restored; the latest drink's
    /// replaces any earlier juice or cordial.
    Juice(crate::fruit::FruitTag),
    /// Cordial drunk, as juice.
    Cordial(crate::fruit::FruitTag),
    /// A fermented drink drunk (`juice::drink_effect`): Mana restored, twice
    /// its juice's; replaces any earlier juice, cordial or drink.
    Drink,
    /// The stage of drinking it is at (`alcohol::Stage`): one at a time,
    /// swapped as the stage changes (`Sheet::keep_alcohol`).
    Alcohol(crate::alcohol::Stage),
    // New sources go at the end: messages carry one by its place in this list.
}

impl Source {
    /// Whether a new effect from `new` takes the place of one from this: the
    /// same source, or both eaten, as only the latest food counts, or both a
    /// fruit's or both a vegetable's effects, or both a juice or cordial drunk.
    pub fn replaced_by(self, new: Source) -> bool {
        match (self, new) {
            (Source::FruitEffects(_), Source::FruitEffects(_)) => true,
            (Source::VegetableEffects(_), Source::VegetableEffects(_)) => true,
            (
                Source::Juice(_) | Source::Cordial(_) | Source::Drink,
                Source::Juice(_) | Source::Cordial(_) | Source::Drink,
            ) => true,
            (Source::Alcohol(_), Source::Alcohol(_)) => true,
            _ => self.is_eaten() && new.is_eaten() || self == new,
        }
    }

    /// Whether it is eaten food's healing: any food, a fruit or a vegetable.
    pub fn is_eaten(self) -> bool {
        matches!(
            self,
            Source::Eaten(_) | Source::EatenFruit(_) | Source::EatenVegetable(_)
        )
    }
}

/// Something that ends an effect early (`EndsOn`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cancel {
    /// Taking a step or moving at all.
    Moving,
    /// Being harmed.
    Hurt,
}

/// What ends an effect before its end tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EndsOn {
    pub moving: bool,
    pub hurt: bool,
}

impl EndsOn {
    pub const NOTHING: Self = Self {
        moving: false,
        hurt: false,
    };

    /// Whether this ends it.
    pub const fn by(self, cancel: Cancel) -> bool {
        match cancel {
            Cancel::Moving => self.moving,
            Cancel::Hurt => self.hurt,
        }
    }
}

/// Bonuses that last from `start` until `end` (an action tick, so they wear
/// off in real time, offline too), or for good with no end, unless something
/// in `ends_on` happens first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    pub source: Source,
    pub bonuses: Vec<Bonus>,
    /// The action tick it began at.
    pub start: u64,
    /// The action tick it ends at, or `None` for good.
    pub end: Option<u64>,
    pub ends_on: EndsOn,
}

impl Effect {
    /// Whether it still counts at this tick.
    pub fn lasts(&self, now: u64) -> bool {
        self.end.is_none_or(|end| now < end)
    }

    /// The debug menu's Bonus: `amount` more of the most of both, from `now`
    /// until `end`.
    pub fn dev_bonus(amount: i32, now: u64, end: u64) -> Self {
        Self {
            source: Source::Dev,
            bonuses: Vital::ALL
                .into_iter()
                .map(|vital| Bonus::flat(Stat::Most(vital), amount.saturating_mul(1000)))
                .collect(),
            start: now,
            end: Some(end),
            ends_on: EndsOn::NOTHING,
        }
    }
}
