//! How a herb grows and what harvesting it does: growth bands, regrowing and
//! harvesting early.

use serde::{Deserialize, Serialize};

/// The stages a herb grows through: harvested in full at the last.
pub const STAGES: u8 = 4;

/// The stage it may be harvested at early (`Early`).
pub const EARLY_STAGE: u8 = 3;

const _: () = assert!(EARLY_STAGE < STAGES);

/// How quickly a herb grows through its `STAGES`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Growth {
    Fast,
    Medium,
    Slow,
    VerySlow,
}

impl Growth {
    /// Game days from nothing to fully grown.
    pub const fn days(self) -> u32 {
        match self {
            Self::Fast => 2,
            Self::Medium => 4,
            Self::Slow => 8,
            Self::VerySlow => 16,
        }
    }
}

/// What harvesting does to the plant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Regrow {
    /// Cut back: it grows again from stage 2.
    Cut,
    /// Picked: it grows again from stage 3.
    Pick,
    /// Dug up: it is gone.
    Dig,
    /// Coppiced: it grows again from stage 1.
    Coppice,
}

impl Regrow {
    /// The stage it grows again from, or `None` if it is gone.
    pub const fn back_to(self) -> Option<u8> {
        match self {
            Self::Cut => Some(2),
            Self::Pick => Some(3),
            Self::Dig => None,
            Self::Coppice => Some(1),
        }
    }
}

/// What harvesting at `EARLY_STAGE` gives.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Early {
    /// How much of a full harvest it gives, as a percentage: 0 for nothing.
    pub taken: u8,
    /// How strong what it gives is, as a percentage: kept for potions, as
    /// quality is now always the plant's rarity.
    pub strength: u8,
    /// The research's words for it.
    pub note: String,
}
