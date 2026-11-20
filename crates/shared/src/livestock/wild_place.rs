//! Where within its biomes a wild form or feral herd lives (`Place`), and how
//! warm it likes it (`Warmth`): what `mob::wild_spawns` fits it to a block by.

use serde::{Deserialize, Serialize};

/// Where within its biomes it is found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Place {
    /// Anywhere in them.
    #[default]
    Anywhere,
    /// Mostly in mountains: a few in the lowlands, more the higher the land.
    Mountains,
    /// Only high up: none at the sea, more the higher the land.
    High,
    /// Only by water (rivers, lakes, the sea): water birds.
    ByWater,
    /// Only near people: by roads and towns.
    NearPeople,
    /// On cliffs and by people: by the sea, high up or by roads (rock doves).
    Cliffs,
}

/// How warm it must be where it lives.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Warmth {
    #[default]
    Any,
    /// No warmer than `COLD_UP_TO` (the Bactrian camel's cold deserts).
    Cold,
    /// At least `HOT_FROM` (the dromedary's hot deserts).
    Hot,
}

/// The warmest a `Cold` place is, in whole °C.
pub const COLD_UP_TO: i32 = 14;

/// The coolest a `Hot` place is, in whole °C.
pub const HOT_FROM: i32 = 15;

impl Warmth {
    /// Whether a place this warm suits it.
    pub fn suits(self, celsius: i32) -> bool {
        match self {
            Self::Any => true,
            Self::Cold => celsius <= COLD_UP_TO,
            Self::Hot => celsius >= HOT_FROM,
        }
    }
}
