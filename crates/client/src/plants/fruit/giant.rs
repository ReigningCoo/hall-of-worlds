//! Giant herbs, as tall as trees. A banana (a long fruit) is a stout green
//! stem of huge arching leaves, its bunch hanging from the top on a curving
//! stalk ending in a purple heart. A papaya is a slender trunk with leaves on
//! long stalks at the top, its fruit crowded round the trunk just below them.

use bevy::prelude::*;
use shared::fruit::FruitShape;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Dress, Look, Show, around, out};

/// A giant herb.
pub(super) fn giant(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    if look.fruit_shape == FruitShape::Long {
        banana(pieces, look, rng, dress);
    } else {
        papaya(pieces, look, rng, dress);
    }
}

fn banana(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let thick = (look.height * 0.06).clamp(0.4, 1.2);
    let top = Vec3::Y * look.height * 0.6;
    pieces.stem(
        Vec3::ZERO,
        top,
        (thick, thick * 0.8),
        look.leaf * Vec3::new(1.2, 0.9, 0.6),
    );
    let leaves = 7;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let first = out(angle, between(rng, 1.0, 2.0)).normalize() * look.height * 0.25;
        let second = out(angle, -0.5);
        let width = look.radius * 0.35;
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(top, first, (look.height * 0.27, width, width * 0.1), shade);
        pieces.leaf(
            top + first,
            second,
            (look.radius * 0.6, width, width * 0.1),
            shade,
        );
    }
    if dress.bare() {
        return;
    }
    // The bunch's stalk, out over the side and down.
    let side = out(dress.between(0.0, std::f32::consts::TAU), 0.0);
    let bend = top + side * thick * 2.0 + Vec3::Y * thick;
    let end = bend + side * thick - Vec3::Y * look.height * 0.3;
    pieces.stem(top, bend, (thick * 0.2, thick * 0.2), look.leaf * 0.7);
    pieces.stem(bend, end, (thick * 0.2, thick * 0.15), look.leaf * 0.7);
    let heart = Vec3::new(thick * 0.5, thick * 0.9, thick * 0.5);
    pieces.ball(end - Vec3::Y * heart.y, heart, 0.0, look.flower);
    if let Show::Fruit(colour) = look.show {
        // Hands of fruit round the stalk, from the bend down.
        for hand in 0..3 {
            let at = bend.lerp(end, 0.15 + hand as f32 * 0.25);
            for i in 0..5 {
                let angle = around(i, 5, &mut dress.rng);
                dress.fruit(pieces, at + out(angle, 0.0) * thick * 0.3, colour);
            }
        }
    }
}

fn papaya(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let thick = (look.height * 0.04).clamp(0.3, 0.9);
    let top = Vec3::Y * look.height * 0.85;
    pieces.stem(Vec3::ZERO, top, (thick, thick * 0.7), WOOD * 1.4);
    let leaves = 8;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let tip = top + out(angle, between(rng, 0.0, 0.6)) * look.radius * 0.6;
        pieces.stem(top, tip, (thick * 0.15, thick * 0.12), look.leaf * 0.85);
        let size = (look.leaf_size, look.leaf_size * 0.9, look.leaf_size * 0.12);
        pieces.leaf(
            tip,
            out(angle, -0.2),
            size,
            look.leaf * between(rng, 0.85, 1.1),
        );
    }
    // Round the trunk, just below the leaves.
    let count = if dress.bare() { 0 } else { 4 };
    for i in 0..count {
        let angle = around(i, count, &mut dress.rng);
        let up = top.y - look.fruit_size * dress.between(0.0, 2.0);
        let at = Vec3::Y * up + out(angle, 0.0) * (thick + look.fruit_size * 0.8);
        dress.at(pieces, at);
    }
}
