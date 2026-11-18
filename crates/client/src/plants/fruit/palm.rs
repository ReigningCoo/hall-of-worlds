//! Palms: one tall trunk, bending a little, with a ring of drooping fronds,
//! and their flowers and fruit in bunches hanging under the crown (dates,
//! coconuts, açaí).

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, WOOD, between};
use super::{Dress, Look, around, out};

/// A palm.
pub(super) fn palm(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let thick = (look.height * 0.035).clamp(0.3, 1.0);
    let lean = out(between(rng, 0.0, TAU), 0.0) * look.height * 0.08;
    // The trunk in three pieces, curving over by `lean` at the top.
    let mut from = Vec3::ZERO;
    for piece in 1..=3 {
        let up = piece as f32 / 3.0;
        let to = Vec3::Y * look.height * up + lean * up * up;
        let r = thick * (1.0 - up * 0.25);
        pieces.stem(from, to, (r, r * 0.92), WOOD * 1.15);
        from = to;
    }
    let top = from;
    fronds(pieces, look, top, rng);
    if dress.bare() {
        return;
    }
    // Bunches hanging under the crown, round the trunk.
    let bunches = 3;
    for i in 0..bunches {
        let angle = around(i, bunches, &mut dress.rng);
        let at = top + out(angle, 0.0) * thick * 1.4 - Vec3::Y * thick * 0.5;
        bunch(pieces, dress, at);
    }
}

/// A ring of fronds from `top`: each rising, then drooping.
pub(super) fn fronds(pieces: &mut Pieces, look: &Look, top: Vec3, rng: &mut Rng) {
    let fronds = 9;
    let reach = look.radius;
    for i in 0..fronds {
        let angle = around(i, fronds, rng);
        let lift = between(rng, 0.25, 0.7);
        let first = out(angle, lift).normalize() * reach * 0.5;
        let second = out(angle, -0.6).normalize();
        let width = look.leaf_size * 0.8;
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(top, first, (reach * 0.55, width, width * 0.2), shade);
        pieces.leaf(
            top + first,
            second,
            (reach * 0.55, width * 0.8, width * 0.2),
            shade,
        );
    }
}

/// A bunch hanging from `at`: flowers or fruit in a cone narrowing down.
pub(super) fn bunch(pieces: &mut Pieces, dress: &mut Dress, at: Vec3) {
    let s = dress.look.fruit_size.max(dress.look.flower_size);
    // Big fruit (coconuts) hang a few to a bunch, small ones many.
    let rows = if s > 0.15 * METRE { 2 } else { 3 };
    for row in 0..rows {
        let across = s * (1.6 - row as f32 * 0.5);
        let count = rows + 1 - row;
        for i in 0..count {
            let angle = i as f32 / count as f32 * TAU + row as f32;
            let side = out(angle, 0.0) * across;
            dress.at(pieces, at + side - Vec3::Y * s * row as f32 * 1.5);
        }
    }
}
