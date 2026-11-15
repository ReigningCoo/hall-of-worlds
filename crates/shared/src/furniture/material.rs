//! What a piece of furniture is made of: a wood or a stone.

use serde::{Deserialize, Serialize};

/// What a piece is made of (only for its model).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Material {
    Wood(Wood),
    Stone(Stone),
}

impl Material {
    pub fn is_stone(self) -> bool {
        matches!(self, Self::Stone(_))
    }
}

/// Woods furniture is made of. Ebony is rare.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Wood {
    Oak,
    Walnut,
    Ash,
    Pine,
    Elm,
    Beech,
    Cherry,
    Ebony,
}

impl Wood {
    /// Every wood with how often it is chosen, out of the total.
    pub const WEIGHTS: [(Self, u64); 8] = [
        (Self::Oak, 24),
        (Self::Pine, 20),
        (Self::Ash, 14),
        (Self::Elm, 12),
        (Self::Beech, 12),
        (Self::Walnut, 8),
        (Self::Cherry, 8),
        (Self::Ebony, 2),
    ];
}

/// Stones furniture is made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Stone {
    Granite,
    Sandstone,
    Limestone,
    Marble,
    Slate,
}

impl Stone {
    /// Every stone with how often it is chosen, out of the total.
    pub const WEIGHTS: [(Self, u64); 5] = [
        (Self::Limestone, 30),
        (Self::Granite, 25),
        (Self::Sandstone, 20),
        (Self::Slate, 15),
        (Self::Marble, 10),
    ];
}

/// One of `weights`, by `roll`, drawing nothing from any sequence.
pub(super) fn weighted<T: Copy>(roll: u64, weights: &[(T, u64)]) -> T {
    let total: u64 = weights.iter().map(|&(_, weight)| weight).sum();
    let mut left = roll % total;
    for &(choice, weight) in weights {
        if left < weight {
            return choice;
        }
        left -= weight;
    }
    weights[0].0
}
