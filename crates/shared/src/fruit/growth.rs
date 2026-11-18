//! How fast each fruit plant grows, by its family (owner, 2026-11-17: low
//! herbs, mats and canes fast; shrubs and vines medium; trees slow; the
//! giants very slow), and which plants have a trunk in the way.

use super::{FruitDef, PlantFamily};
use crate::herb::Growth;

/// The trees that grow very slowly (owner, 2026-11-17), by key.
pub const GIANTS: [&str; 6] = [
    "baobab",
    "brazil_nut",
    "durian",
    "mango",
    "siberian_pine_nut",
    "stone_pine_nut",
];

impl PlantFamily {
    /// How fast plants of this family grow (`Growth::days` is one round of
    /// leaves, flowers and fruit).
    pub const fn growth(self) -> Growth {
        match self {
            Self::LowHerb | Self::LowMat | Self::Canes => Growth::Fast,
            Self::Shrub
            | Self::Vine
            | Self::GroundVine
            | Self::Rosette
            | Self::PadCactus
            | Self::GiantHerb => Growth::Medium,
            Self::BroadleafTree | Self::Conifer | Self::Palm | Self::ColumnCactus => Growth::Slow,
        }
    }

    /// Whether it stands on a trunk that is in the way, as a wild tree's is
    /// (`clearance`): trees, palms, giant herbs and column cacti.
    pub const fn has_trunk(self) -> bool {
        matches!(
            self,
            Self::BroadleafTree | Self::Conifer | Self::Palm | Self::GiantHerb | Self::ColumnCactus
        )
    }
}

impl FruitDef {
    /// How fast its plant grows: its family's speed, or very slow for the
    /// `GIANTS`.
    pub fn growth(&self) -> Growth {
        if GIANTS.contains(&self.key.as_str()) {
            Growth::VerySlow
        } else {
            self.plant.growth()
        }
    }
}
