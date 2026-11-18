//! `Variety`: how one vegetable item differs from others of its kind, from its
//! variant (its look seed): which of its colours (most are the usual one, a few
//! other varieties), size, shade, and fixed wobbles for its pieces.

use super::super::herb::kit::wobble;

/// Variants below this are the usual colour; the rest share the others.
const USUAL: u8 = 10;

pub(super) struct Variety {
    pub(super) variant: u8,
    /// Which of its look's colours: 0 the usual.
    pub(super) colour: usize,
    /// Its size against the largest, 0.88 to 1.
    pub(super) size: f32,
    /// Lighter (above 0) or darker, -1 to 1.
    pub(super) shade: f32,
    /// A turn for the whole, -1 to 1.
    pub(super) turn: f32,
}

impl Variety {
    /// Variant `variant` of a vegetable with `colours` colours.
    pub(super) fn of(variant: u8, colours: usize) -> Self {
        let roll = |k| wobble(0, variant, k);
        let colour = match colours {
            0 | 1 => 0,
            _ if variant < USUAL => 0,
            _ => 1 + usize::from(variant - USUAL) % (colours - 1),
        };
        Self {
            variant,
            colour,
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
