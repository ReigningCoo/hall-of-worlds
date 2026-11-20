//! The crafts benches are made for: each one's length and the tools it takes.

use serde::{Deserialize, Serialize};

use crate::alchemy::Apparatus;
use crate::item::ItemKind;

/// A craft a bench is for. New ones go at the end (saved and sent by index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Craft {
    Alchemy,
}

impl Craft {
    pub const ALL: [Self; 1] = [Self::Alchemy];

    /// Its place in `ALL`, for hashes.
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|&craft| craft == self)
            .unwrap_or(0)
    }

    /// Its bench's length along the wall, in cells (1 to 3); every bench is
    /// 1 deep. Alchemy's is 2 (owner's).
    pub fn length(self) -> u8 {
        match self {
            Self::Alchemy => 2,
        }
    }

    /// The kinds of tool its bench holds, one place each, in order: Alchemy's
    /// apparatus.
    pub fn tools(self) -> Vec<ItemKind> {
        match self {
            Self::Alchemy => Apparatus::ALL.iter().filter_map(|a| a.item()).collect(),
        }
    }

    /// Whether its bench takes a tool of `kind`.
    pub fn takes(self, kind: ItemKind) -> bool {
        self.tools().contains(&kind)
    }
}
