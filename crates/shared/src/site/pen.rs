//! Pens for kept animals (livestock plan PR 8): a sty for pigs and a yard for
//! poultry, each a small fenced field with a shelter against its back fence,
//! and in yards perhaps a dovecote on a post. Farms have them by the house,
//! towns a few small yards on bare ground.
//!
//! Placed last, from their own seeds, on ground with walkable ground all
//! round, so nothing else moves, no generator version changes and everything
//! stays as reachable as before. Sent with the fields (`FieldLook::pen`), so
//! they reach the client and the wilderness as fields do.

use serde::{Deserialize, Serialize};

use super::town::Rect;

// Split into files (`cargo xtask lint` keeps files small).
mod place;

pub(super) use place::{PenSize, add_pen};

/// What a pen is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenKind {
    /// Pigs, with a low roofed shelter.
    Sty,
    /// Poultry, with a coop.
    Yard,
}

/// A pen's own parts, on top of its field (`FieldLook::rect`, inside the fence).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PenLook {
    pub kind: PenKind,
    /// The cells under the shelter or coop (walkable: it is open in front).
    pub shelter: Rect,
    /// The fence the shelter backs onto: 0 north, 1 east, 2 south, 3 west.
    pub back: u8,
    /// The cell a dovecote's post stands on, in a yard that has one: pigeons.
    pub dovecote: Option<(i64, i64)>,
}

#[cfg(test)]
mod tests;
