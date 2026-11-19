//! A felled tree's stump: a short piece of its trunk in its bark, flared
//! roots, and the cut on top in its wood's heartwood colour.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, between};
use super::{around, out};

/// A stump `thick` across (half its trunk's width), in `bark`, cut showing
/// `wood`.
pub(super) fn stump(pieces: &mut Pieces, thick: f32, (bark, wood): (Vec3, Vec3), rng: &mut Rng) {
    let thick = stump_thick(thick);
    let high = (thick * 1.8).max(0.3 * METRE) * between(rng, 0.85, 1.15);
    let top = Vec3::Y * high;
    pieces.stem(Vec3::ZERO, top, (thick * 1.15, thick), bark);
    pieces.stem(
        top,
        top + Vec3::Y * 0.03 * METRE,
        (thick * 0.88, thick * 0.88),
        wood,
    );
    let roots = 3;
    for i in 0..roots {
        let angle = around(i, roots, rng);
        let foot = out(angle, 0.0) * thick * 2.2;
        pieces.stem(top * 0.45, foot, (thick * 0.45, thick * 0.2), bark);
    }
}

/// How thick a stump left by a trunk `thick` across (half its width) is.
fn stump_thick(thick: f32) -> f32 {
    thick.max(0.08 * METRE) * 1.1
}

/// How tall a stump left by a trunk `thick` across is at most, and how far
/// its roots reach from its middle.
pub(super) fn stump_size(thick: f32) -> (f32, f32) {
    let thick = stump_thick(thick);
    let high = (thick * 1.8).max(0.3 * METRE) * 1.15 + 0.03 * METRE;
    (high * 1.05, thick * 3.0)
}
