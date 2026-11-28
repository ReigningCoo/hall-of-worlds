//! What each `Desire` is for (mob behaviour plan PR 9): which kinds of item on
//! the ground a mob that has it goes for and takes.

use super::Desire;
use crate::item::{ItemCategory, ItemKind};

impl Desire {
    /// Every desire, in order.
    pub const ALL: [Self; 3] = [Self::Food, Self::Coins, Self::Weapons];

    /// Whether one with this desire wants this kind of item: food is anything
    /// in the Food category, coins any coin, weapons anything in the Weapons
    /// category.
    pub fn wants(self, kind: ItemKind) -> bool {
        match self {
            Self::Food => kind.category() == ItemCategory::Food,
            Self::Coins => kind.coin_value().is_some(),
            Self::Weapons => kind.category() == ItemCategory::Weapons,
        }
    }

    /// Whether any of `desires` wants this kind of item.
    pub fn any_wants(desires: &[Self], kind: ItemKind) -> bool {
        desires.iter().any(|desire| desire.wants(kind))
    }
}

#[cfg(test)]
mod tests;
