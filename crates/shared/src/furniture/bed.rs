//! Beds (tavern furniture plan PR 7): a wooden bed, 2 long, its head against
//! a wall. The model is smaller than its cells (owner's), so there is room to
//! walk round it even in a corner. Nobody sleeps in one yet.

use serde::{Deserialize, Serialize};

/// A bed's width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Bed {
    /// 2 × 1.
    Single,
    /// 2 × 2.
    Double,
}

impl Bed {
    pub const ALL: [Self; 2] = [Self::Single, Self::Double];

    /// Cells head to foot.
    pub const LENGTH: u8 = 2;

    /// Cells across.
    pub fn width(self) -> u8 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
        }
    }
}
