//! `ClothingPiece`: the kinds of item that are clothes with forms (hats on the Outer layer).

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
    Hood,
    Cap,
    Headcloth,
    Headdress,
    Neckwear,
    Shawl,
    /// Brimmed and stiff hats, on the Outer layer.
    Hat,
}

impl ClothingPiece {
    pub const ALL: [Self; 16] = [
        Self::Tunic,
        Self::Robe,
        Self::Doublet,
        Self::Bodice,
        Self::Vest,
        Self::Trousers,
        Self::Skirt,
        Self::Belt,
        Self::Gloves,
        Self::Hood,
        Self::Cap,
        Self::Headcloth,
        Self::Headdress,
        Self::Neckwear,
        Self::Shawl,
        Self::Hat,
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
