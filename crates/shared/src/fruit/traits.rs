//! A fruit plant's plainer properties: how common it is, how much picking
//! gives and what sort of plant it is. Data for where they grow and their models
//! (PRs 3 and 4 of the fruit plan).

use serde::{Deserialize, Serialize};

/// How common a fruit plant is (owner, 2026-11-17: wild ones are very rare).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Abundance {
    Common,
    Uncommon,
    Rare,
    /// Never wild: only in orchards.
    OrchardOnly,
}

/// How many items picking a ripe plant gives untrained (owner, 2026-11-17);
/// Orchards and Foraging will add.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HarvestTier {
    One,
    UpToTwo,
    UpToThree,
    TwoToFour,
    ThreeToSix,
}

impl HarvestTier {
    /// The fewest and most items, untrained.
    pub const fn range(self) -> (u32, u32) {
        match self {
            Self::One => (1, 1),
            Self::UpToTwo => (1, 2),
            Self::UpToThree => (1, 3),
            Self::TwoToFour => (2, 4),
            Self::ThreeToSix => (3, 6),
        }
    }
}

/// The sort of plant a fruit grows on, which its model is built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PlantFamily {
    BroadleafTree,
    Conifer,
    Palm,
    Shrub,
    /// Bramble canes (raspberry, blackberry).
    Canes,
    /// A climbing vine (grape, kiwi, passion fruit).
    Vine,
    /// A vine along the ground (melons).
    GroundVine,
    /// A low mat of creeping stems (cranberry, crowberry).
    LowMat,
    LowHerb,
    /// A giant herb, tree-sized (banana, papaya).
    GiantHerb,
    /// A rosette of leaves (pineapple).
    Rosette,
    PadCactus,
    ColumnCactus,
}
