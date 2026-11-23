//! What a chest or wardrobe is: its length (1 deep, against a wall), the
//! grid inside and what it takes.

use serde::{Deserialize, Serialize};

use crate::item::{ItemCategory, ItemKind};

/// A wooden chest with a lid, standing on the floor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Chest {
    /// 1 × 1: a small coffer at a bed's foot.
    Small,
    /// 1 × 2: a long blanket chest.
    Large,
}

impl Chest {
    pub const ALL: [Self; 2] = [Self::Small, Self::Large];

    /// Cells along its front.
    pub fn length(self) -> u8 {
        match self {
            Self::Small => 1,
            Self::Large => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Small => "Small Chest",
            Self::Large => "Chest",
        }
    }
}

/// A tall wooden wardrobe against a wall, its doors shut.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Wardrobe {
    /// 1 × 1, one door.
    Narrow,
    /// 1 × 2, two doors.
    Wide,
}

impl Wardrobe {
    pub const ALL: [Self; 2] = [Self::Narrow, Self::Wide];

    /// Cells along its front.
    pub fn length(self) -> u8 {
        match self {
            Self::Narrow => 1,
            Self::Wide => 2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Narrow => "Narrow Wardrobe",
            Self::Wide => "Wardrobe",
        }
    }
}

/// A piece storing things in a grid: a chest or a wardrobe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Storage {
    Chest(Chest),
    Wardrobe(Wardrobe),
}

impl Storage {
    /// Its grid inside, cells wide and tall (first guesses): a small chest
    /// 6 × 4 (a backpack's), a chest 8 × 5, a narrow wardrobe 4 × 8, a
    /// wardrobe 8 × 8. Anything up to `item::MAX_SHAPE_SIDE` fits one of
    /// the wardrobes' grids, some way round.
    pub fn grid(self) -> (u8, u8) {
        match self {
            Self::Chest(Chest::Small) => (6, 4),
            Self::Chest(Chest::Large) => (8, 5),
            Self::Wardrobe(Wardrobe::Narrow) => (4, 8),
            Self::Wardrobe(Wardrobe::Wide) => (8, 8),
        }
    }

    /// Whether it takes a `kind` (owner's: a chest anything, a wardrobe
    /// clothes only: what is worn and is clothing or armour, so never a
    /// ring, a weapon or a bag).
    pub fn takes(self, kind: ItemKind) -> bool {
        match self {
            Self::Chest(_) => true,
            Self::Wardrobe(_) => {
                kind.def().wear.is_some()
                    && matches!(
                        kind.category(),
                        ItemCategory::Clothing | ItemCategory::Armour
                    )
            }
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Chest(chest) => chest.name(),
            Self::Wardrobe(wardrobe) => wardrobe.name(),
        }
    }
}
