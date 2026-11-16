//! One board, leg or block of a piece of furniture, in the piece's own frame.

use bevy::prelude::*;

/// Which way a part's grain runs (wood), or its layers lie flat (stone:
/// `Up`, so bands run level round its sides).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Grain {
    /// Along the piece's length (its frame's `x`).
    Along,
    /// Across it (`z`).
    Across,
    /// Upwards (`y`).
    Up,
}

/// A box in the piece's frame (`x` along its length from its head, `y` up
/// from the ground, `z` across it), in world units. A leaning part's top is
/// moved by `lean` (`x`, `z`) from its foot, as a trestle's legs splay.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Part {
    pub(super) low: Vec3,
    pub(super) high: Vec3,
    pub(super) grain: Grain,
    pub(super) lean: Vec2,
    /// A little lighter or darker than the rest, as boards differ.
    pub(super) shade: f32,
    /// Its picture turned round, so boards side by side don't match.
    pub(super) flipped: bool,
}

impl Part {
    /// An upright box from `low` to `high`.
    pub(super) fn block(low: Vec3, high: Vec3, grain: Grain) -> Self {
        Self {
            low: low.min(high),
            high: low.max(high),
            grain,
            lean: Vec2::ZERO,
            shade: 1.0,
            flipped: false,
        }
    }

    /// The same, its top moved by `lean`.
    pub(super) fn leaning(self, lean: Vec2) -> Self {
        Self { lean, ..self }
    }

    /// The same, shaded and perhaps turned round.
    pub(super) fn varied(self, shade: f32, flipped: bool) -> Self {
        Self {
            shade,
            flipped,
            ..self
        }
    }

    /// Its middle, foot and top together.
    pub(super) fn middle(&self) -> Vec3 {
        let lean = Vec3::new(self.lean.x, 0.0, self.lean.y);
        (self.low + self.high + lean) / 2.0
    }
}
