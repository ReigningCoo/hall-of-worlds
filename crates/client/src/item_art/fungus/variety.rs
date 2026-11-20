//! `Variety`: how one fungus item differs from others of its kind, from its
//! variant (its look seed): size, shade, a lean, and fixed wobbles for its
//! pieces.

use super::super::herb::kit::wobble;

pub(super) struct Variety {
    pub(super) variant: u8,
    /// Its size against the largest, 0.88 to 1.
    pub(super) size: f32,
    /// Lighter (above 0) or darker, -1 to 1.
    pub(super) shade: f32,
    /// A turn for the whole, -1 to 1.
    pub(super) turn: f32,
}

impl Variety {
    pub(super) fn of(variant: u8) -> Self {
        let roll = |k| wobble(0, variant, k);
        Self {
            variant,
            size: 0.88 + (roll(1) + 1.0) / 2.0 * 0.12,
            shade: roll(2),
            turn: roll(3),
        }
    }

    /// A fixed wobble, -1 to 1, for the `n`th piece's `k`th measure.
    pub(super) fn wobble(&self, n: usize, k: u32) -> f32 {
        wobble(n + 1, self.variant, k)
    }

    /// 0 to 1, for the `n`th piece's `k`th measure.
    pub(super) fn share(&self, n: usize, k: u32) -> f32 {
        (self.wobble(n, k) + 1.0) / 2.0
    }
}
