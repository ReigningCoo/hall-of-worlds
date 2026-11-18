//! Low plants. A low mat (crowberry, lingonberry, cranberry) is a cushion
//! of small leaves with berries over it. A low herb (strawberry, cloudberry)
//! is a ring of leaves on stalks, its flowers and fruit on short stems of
//! their own near the ground.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::{Dress, Look, Show, around, out};

/// A low mat.
pub(super) fn mat(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let lumps = 6;
    let size = Vec3::new(look.radius * 0.45, look.height * 0.55, look.radius * 0.45);
    let count = if look.show == Show::Flowers { 4 } else { 3 };
    for i in 0..lumps {
        let (at, own) = if i == 0 {
            (Vec3::Y * look.height * 0.45, size * 1.1)
        } else {
            let angle = around(i - 1, lumps - 1, rng);
            let at = out(angle, 0.0) * look.radius * between(rng, 0.45, 0.6);
            (
                at + Vec3::Y * look.height * 0.35,
                size * between(rng, 0.75, 1.0),
            )
        };
        pieces.ball(
            at,
            own,
            between(rng, 0.0, 1.6),
            look.leaf * between(rng, 0.85, 1.1),
        );
        dress.over(pieces, at, own, count);
    }
}

/// A low herb.
pub(super) fn herb(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let leaves = 6;
    let stalk = look.leaf * 0.8;
    let thick = 0.1;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let tip = out(angle, 0.0) * look.radius * between(rng, 0.45, 0.65)
            + Vec3::Y * look.height * between(rng, 0.6, 0.9);
        pieces.stem(Vec3::ZERO, tip, (thick, thick), stalk);
        let towards = out(angle, between(rng, -0.1, 0.25));
        let length = look.leaf_size.clamp(look.radius * 0.35, look.radius * 0.6);
        let size = (length, length * 0.8, length * 0.12);
        pieces.leaf(tip, towards, size, look.leaf * between(rng, 0.85, 1.1));
    }
    if dress.bare() {
        return;
    }
    // Flowers and fruit on stems of their own, between the leaves.
    let stems = 3;
    for i in 0..stems {
        let angle = around(i, stems, &mut dress.rng) + 0.5;
        let tip = out(angle, 0.0) * look.radius * dress.between(0.25, 0.45)
            + Vec3::Y * look.height * dress.between(0.7, 1.0);
        pieces.stem(Vec3::ZERO, tip, (thick * 0.7, thick * 0.7), stalk);
        dress.at(pieces, tip);
    }
}
