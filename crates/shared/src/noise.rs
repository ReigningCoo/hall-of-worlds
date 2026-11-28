//! Noise (mob behaviour plan PR 6): a sound made at a place with a loudness,
//! muffled by walls and closed doors between, and heard by a listener by its
//! hearing and how far away it is, with a roll from a hash. Both sides use it:
//! the server for whether mobs hear, the client for the sneaking eye.

mod footsteps;
mod hear;
mod muffle;

pub use footsteps::{
    ARMOUR_GRAMS_PER_CELL, Footsteps, SNEAK_BEST_PER_MILLE, SNEAK_WORST_PER_MILLE, WALK_CELLS,
};
pub use hear::{Hearer, chance, heard, hearing_per_mille, reach, roll};
pub use muffle::{DOOR_PER_MILLE, WALL_PER_MILLE, muffled};

use crate::spot::{CELL_SPAN, Spot};

/// What made a noise.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    /// Someone walking (`Footsteps`).
    Footsteps,
    /// A blow being wound up or landing.
    Blow,
    /// A spell being cast.
    Spell,
    /// A door opened.
    Door,
    /// A door kicked.
    DoorKicked,
    /// An alarm call (`behaviour::Alarm`: its loudness by race).
    Call,
}

impl Source {
    /// How far each is heard in the open by normal hearing, in cells (first
    /// guesses, Claude's, to check in play; a fight's 8 as before).
    pub const fn cells(self) -> u32 {
        match self {
            Self::Footsteps => WALK_CELLS,
            Self::Blow => 8,
            Self::Spell => 6,
            Self::Door => 5,
            Self::DoorKicked => 12,
            Self::Call => 12,
        }
    }
}

/// A noise made somewhere on a level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Noise {
    pub source: Source,
    pub at: Spot,
    /// How far it is heard in the open by normal hearing, in units (240 a
    /// cell).
    pub loudness: u32,
}

impl Noise {
    /// A noise of `source` at `at`, at its usual loudness.
    pub const fn new(source: Source, at: Spot) -> Self {
        Self {
            source,
            at,
            loudness: source.cells() * CELL_SPAN as u32,
        }
    }

    /// Footsteps at `at`, as loud as `steps` makes them.
    pub fn footsteps(at: Spot, steps: Footsteps) -> Self {
        Self {
            source: Source::Footsteps,
            at,
            loudness: steps.loudness(),
        }
    }

    /// An alarm call carrying `cells` (`behaviour::Alarm::loudness`).
    pub const fn call(at: Spot, cells: u16) -> Self {
        Self {
            source: Source::Call,
            at,
            loudness: cells as u32 * CELL_SPAN as u32,
        }
    }
}

#[cfg(test)]
mod tests;
