//! `SetPiece`: which set an item belongs to, and which of its pieces it is.

use serde::{Deserialize, Serialize};

use super::{ItemSet, POOL, roll};
use crate::id::Id;
use crate::item::{ItemKind, Quality};

/// One piece of one set: what an item keeps of its set (`Item::set`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SetPiece {
    /// The set's number in the pool, below `POOL`.
    pub set: u16,
    /// Which of its pieces, counting from 0.
    pub piece: u8,
}

impl SetPiece {
    /// The whole set it belongs to.
    pub fn whole(self) -> ItemSet {
        ItemSet::of(self.set)
    }

    /// What kind of item this piece is, if the set has such a piece.
    pub fn kind(self) -> Option<ItemKind> {
        let (_, pieces) = roll::pieces(self.set);
        pieces.get(usize::from(self.piece)).copied()
    }

    /// The quality every piece of its set has.
    pub fn quality(self) -> Quality {
        roll::pieces(self.set).0
    }

    /// A piece of some set for an item of this kind and quality, chosen from
    /// its id: for Give Item and for set items saved before sets had pieces.
    /// The search starts at a set picked from the id and takes the first set
    /// with a piece of this kind, one of this quality too if any has it.
    /// `None` if no set has a piece of this kind.
    pub fn fitting(kind: ItemKind, quality: Option<Quality>, id: Id) -> Option<Self> {
        let start = id.seed().derive("item set").value() % u64::from(POOL);
        let order = (0..POOL).map(|step| ((start + u64::from(step)) % u64::from(POOL)) as u16);
        let mut kind_only = None;
        for set in order {
            let (set_quality, pieces) = roll::pieces(set);
            let Some(piece) = pieces.iter().position(|&each| each == kind) else {
                continue;
            };
            let found = Self {
                set,
                piece: piece as u8,
            };
            if quality.is_none_or(|quality| quality == set_quality) {
                return Some(found);
            }
            kind_only = kind_only.or(Some(found));
        }
        kind_only
    }
}
