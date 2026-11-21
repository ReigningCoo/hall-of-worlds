//! `Craft`: which craft a recipe is for.

use serde::{Deserialize, Serialize};

/// The craft a recipe is for: Cooking now, Alchemy later (its scrolls' art
/// and emblem to tell them apart).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Craft {
    #[default]
    Cooking,
    Alchemy,
}

impl Craft {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cooking => "Cooking",
            Self::Alchemy => "Alchemy",
        }
    }
}
