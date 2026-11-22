//! `Fireplace`: how long a fireplace is, its name, and how many places its
//! log basket has.

use serde::{Deserialize, Serialize};

/// How many pieces of kindling one place in a log basket holds, as one stack
/// (Claude's first guess: 10 kg).
pub const KINDLING_PER_PLACE: u32 = 10;

/// A stone fireplace `length` cells long (1 to 3), 1 deep, against a wall.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fireplace {
    pub length: u8,
}

impl Fireplace {
    /// The lengths fireplaces come in.
    pub const LENGTHS: [u8; 3] = [1, 2, 3];

    /// Every fireplace there is.
    pub fn all() -> Vec<Self> {
        Self::LENGTHS.map(|length| Self { length }).to_vec()
    }

    /// Whether it is one there is.
    pub fn is_well_made(self) -> bool {
        Self::LENGTHS.contains(&self.length)
    }

    /// "Fireplace", "Wide Fireplace" or "Inglenook".
    pub fn name(self) -> &'static str {
        match self.length {
            1 => "Fireplace",
            2 => "Wide Fireplace",
            _ => "Inglenook",
        }
    }

    /// Whether it is an inglenook (1 × 3): the fire in its middle cell, a
    /// settle either side of it inside.
    pub fn is_inglenook(self) -> bool {
        self.length >= 3
    }

    /// Which of its cells along the wall the fire burns in, from its start:
    /// the first of a fireplace 1 or 2 long (the log basket beside it in the
    /// second), the middle of an inglenook.
    pub fn fire_cell(self) -> u8 {
        if self.is_inglenook() { 1 } else { 0 }
    }

    /// How many places its log basket has, each a stack of kindling (Claude's
    /// first guesses): 2, 3, and 4 in an inglenook.
    pub fn basket_places(self) -> usize {
        usize::from(self.length.clamp(1, 3)) + 1
    }
}
