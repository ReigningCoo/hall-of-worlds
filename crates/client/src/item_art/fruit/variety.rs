//! `Variety`: how one fruit item differs from others of its fruit, from its
//! variant (its look seed): size, colour within its ripe range, blush, stalk
//! and leaf, speckles, and which way it lies.

use super::super::herb::kit::wobble;

/// What a variant changes.
pub(super) struct Variety {
    pub(super) variant: u8,
    /// Its size against the largest, 0.86 to 1.
    pub(super) size: f32,
    /// Towards its unripe colour (above 0) or deeper (below), -1 to 1.
    pub(super) tint: f32,
    /// How much of it is blushed, for fruits that blush: 0 to 1.
    pub(super) blush: f32,
    /// Which way the blush faces, in radians about the up axis.
    pub(super) blush_angle: f32,
    pub(super) stalk: bool,
    pub(super) leaf: bool,
    /// How many speckles a speckled skin has.
    pub(super) speckles: usize,
    /// The whole turned about the up axis.
    pub(super) turn: f32,
}

impl Variety {
    pub(super) fn of(variant: u8) -> Self {
        let roll = |k| wobble(0, variant, k);
        // Half 0 to 1, from a roll.
        let share = |k| (roll(k) + 1.0) / 2.0;
        Self {
            variant,
            size: 0.86 + share(1) * 0.14,
            tint: roll(2),
            blush: share(3),
            blush_angle: roll(4) * std::f32::consts::PI,
            stalk: roll(5) > -0.5,
            leaf: roll(6) > 0.2,
            speckles: 6 + (share(7) * 14.0) as usize,
            turn: roll(8) * 0.25,
        }
    }

    /// A small fixed wobble, -1 to 1, for the `n`th piece's `k`th measure.
    pub(super) fn wobble(&self, n: usize, k: u32) -> f32 {
        wobble(n + 1, self.variant, k)
    }
}
