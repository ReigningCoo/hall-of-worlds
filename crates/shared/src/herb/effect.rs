//! What eating a herb does: `Effect`s on `Stat`s.

use serde::{Deserialize, Serialize};

use crate::sheet::Attribute;

/// What a herb's effect changes: one of the nine attributes, or Health or Mana.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Stat {
    Strength,
    Agility,
    Toughness,
    Intellect,
    Perception,
    Will,
    Presence,
    Empathy,
    Aura,
    Health,
    Mana,
}

impl Stat {
    pub const ALL: [Stat; 11] = [
        Self::Strength,
        Self::Agility,
        Self::Toughness,
        Self::Intellect,
        Self::Perception,
        Self::Will,
        Self::Presence,
        Self::Empathy,
        Self::Aura,
        Self::Health,
        Self::Mana,
    ];

    /// The attribute it is, if it is one.
    pub const fn attribute(self) -> Option<Attribute> {
        Some(match self {
            Self::Strength => Attribute::Strength,
            Self::Agility => Attribute::Agility,
            Self::Toughness => Attribute::Toughness,
            Self::Intellect => Attribute::Intellect,
            Self::Perception => Attribute::Perception,
            Self::Will => Attribute::Will,
            Self::Presence => Attribute::Presence,
            Self::Empathy => Attribute::Empathy,
            Self::Aura => Attribute::Aura,
            Self::Health | Self::Mana => return None,
        })
    }
}

/// One effect: a stat raised or lowered, weakly (1), moderately (2) or strongly
/// (3). In potions (to come), effects add up and opposites cancel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Effect {
    pub stat: Stat,
    /// -3 to 3, never 0.
    pub change: i8,
}

impl Effect {
    /// How strong it is, 1 to 3, whichever way.
    pub const fn strength(self) -> u8 {
        self.change.unsigned_abs()
    }

    /// Whether it raises its stat.
    pub const fn raises(self) -> bool {
        self.change > 0
    }
}
