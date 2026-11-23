//! Barrels standing as tables, stacks of crates and sacks (tavern furniture
//! plan PR 6): wooden floor clutter near kitchens and in cellars. Each is
//! 1 × 1. A barrel takes seats round it and candles on it as a table does;
//! crates and sacks are only in the way.

use serde::{Deserialize, Serialize};

/// A barrel standing on end, used as a table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Barrel {
    /// Its own head as the top.
    Bare,
    /// A round board laid over its head, wider than the barrel.
    Topped,
}

impl Barrel {
    pub const ALL: [Self; 2] = [Self::Bare, Self::Topped];
}

/// Crates, stacked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Crates {
    /// One crate.
    One,
    /// Two, one on the other.
    Two,
    /// Two side by side with one across them on top.
    Three,
    /// Two stacks of two.
    Four,
}

impl Crates {
    pub const ALL: [Self; 4] = [Self::One, Self::Two, Self::Three, Self::Four];

    pub fn count(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

/// Sacks of grain or flour on the floor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Sacks {
    /// One sack standing.
    One,
    /// Two leaning together.
    Two,
    /// Three in a heap, one lying on two.
    Heap,
}

impl Sacks {
    pub const ALL: [Self; 3] = [Self::One, Self::Two, Self::Heap];

    pub fn count(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Heap => 3,
        }
    }
}
