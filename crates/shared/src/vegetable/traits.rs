//! A vegetable's plainer properties: how often it is eaten raw, how common it
//! is wild, and what its plant and its item are like (data for where they grow,
//! their pictures and models: later PRs of the vegetable plan).

use serde::{Deserialize, Serialize};

/// How often people eat it raw (owner, 2026-11-17): only `Commonly` and
/// `Sometimes` heal raw, at a quarter of the cooked figure (`raw_ticks`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Raw {
    Commonly,
    Sometimes,
    Rarely,
    Never,
}

impl Raw {
    /// Whether eating it raw heals at all.
    pub const fn heals(self) -> bool {
        matches!(self, Self::Commonly | Self::Sometimes)
    }
}

/// How common it is in the wild (owner, 2026-11-17): crops only farmed are
/// very rare there (escapes); wild-only plants common to uncommon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Abundance {
    Common,
    Uncommon,
    Rare,
    VeryRare,
}

/// The sort of plant it grows on, which its model will be built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PlantShape {
    /// A rosette of leaves over a swollen root (turnip, carrot).
    LeavesOverARoot,
    UprightLeafyHerb,
    LeafyRosette,
    Bush,
    GroundVine,
    ClimberOnAPole,
    ClimberOnSticks,
    BulbClump,
    WaterPlant,
    GrainGrass,
    /// Tall grass (maize, sorghum).
    TallGrass,
    /// Grass standing in water (wild rice).
    WaterGrass,
    /// Grass in a flooded field (rice).
    PaddyGrass,
    Fern,
    Succulent,
    AgaveRosette,
    BambooGrove,
    /// A thorny tree (mesquite).
    ThornyTree,
}

/// The part that is picked, which its item's picture will show.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Part {
    Root,
    Tuber,
    Bulb,
    LeafBunch,
    StalkBunch,
    Shoots,
    FlowerHead,
    /// A botanical fruit eaten as a vegetable (tomato, pumpkin).
    Fruit,
    Pods,
    Seeds,
    EarOfGrain,
}
