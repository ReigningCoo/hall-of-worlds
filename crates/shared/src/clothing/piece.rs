//! `ClothingPiece`: the kinds of item that are Middle-layer clothes with forms.

use serde::{Deserialize, Serialize};

use crate::item::ItemKind;

/// A kind of clothing, one for each item kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClothingPiece {
    Tunic,
    Robe,
    Doublet,
    Bodice,
    Vest,
    Trousers,
    Skirt,
    Belt,
    Gloves,
}

impl ClothingPiece {
    pub const ALL: [Self; 9] = [
        Self::Tunic,
        Self::Robe,
        Self::Doublet,
        Self::Bodice,
        Self::Vest,
        Self::Trousers,
        Self::Skirt,
        Self::Belt,
        Self::Gloves,
    ];

    /// The piece an item kind is, or `None` for other kinds.
    pub fn of(kind: ItemKind) -> Option<Self> {
        ItemKind::ALL_CLOTHING
            .iter()
            .position(|&clothing| clothing == kind)
            .map(|index| Self::ALL[index])
    }

    /// The item kind this is.
    pub fn kind(self) -> ItemKind {
        ItemKind::ALL_CLOTHING[self as usize]
    }
}
