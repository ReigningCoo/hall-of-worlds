//! The crafts benches are made for: each one's length and the tools it takes.
//! Hearths (cooking plan PR 8) are benches too: the Stove, Hearth and Range.

use serde::{Deserialize, Serialize};

use crate::alchemy::Apparatus;
use crate::item::ItemKind;

/// A craft a bench is for. New ones go at the end (saved and sent by index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Craft {
    Alchemy,
    /// A hearth 1 × 1: a stove (with its oven) and nothing else.
    Stove,
    /// A hearth 1 × 2: a stove and a place for each kitchen tool.
    Hearth,
    /// A hearth 1 × 3: a Hearth with drawers, a storage space for leaving
    /// food to rest or prove (`STORAGE_PLACES`).
    Range,
}

/// The kitchen tools a Hearth or Range holds, one place each, in order
/// (cooking's Pan, Cauldron, Bowl, Sieve, Knife and Mortar; the Oven is the
/// hearth's own).
pub const KITCHEN_TOOLS: [ItemKind; 6] = [
    ItemKind::Pan,
    ItemKind::Cauldron,
    ItemKind::Bowl,
    ItemKind::Sieve,
    ItemKind::Knife,
    ItemKind::MortarAndPestle,
];

impl Craft {
    pub const ALL: [Self; 4] = [Self::Alchemy, Self::Stove, Self::Hearth, Self::Range];

    /// Its place in `ALL`, for hashes.
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|&craft| craft == self)
            .unwrap_or(0)
    }

    /// "Alchemy Bench", "Stove", "Hearth", "Range".
    pub fn name(self) -> &'static str {
        match self {
            Self::Alchemy => "Alchemy Bench",
            Self::Stove => "Stove",
            Self::Hearth => "Hearth",
            Self::Range => "Range",
        }
    }

    /// Its bench's length along the wall, in cells (1 to 3); every bench is
    /// 1 deep. Alchemy's is 2 (owner's); hearths 1, 2 and 3 (owner's).
    pub fn length(self) -> u8 {
        match self {
            Self::Stove => 1,
            Self::Alchemy | Self::Hearth => 2,
            Self::Range => 3,
        }
    }

    /// Whether it is a hearth for cooking: proper heat (`cooking::Heat::
    /// Hearth`) while lit, made of stone.
    pub fn is_hearth(self) -> bool {
        matches!(self, Self::Stove | Self::Hearth | Self::Range)
    }

    /// Whether it has a storage space (`STORAGE_PLACES`): only the Range.
    pub fn has_storage(self) -> bool {
        self == Self::Range
    }

    /// The kinds of tool its bench holds, one place each, in order: Alchemy's
    /// apparatus, the kitchen tools, nothing on a Stove.
    pub fn tools(self) -> Vec<ItemKind> {
        match self {
            Self::Alchemy => Apparatus::ALL.iter().filter_map(|a| a.item()).collect(),
            Self::Stove => Vec::new(),
            Self::Hearth | Self::Range => KITCHEN_TOOLS.to_vec(),
        }
    }

    /// Whether its bench takes a tool of `kind`.
    pub fn takes(self, kind: ItemKind) -> bool {
        self.tools().contains(&kind)
    }
}
