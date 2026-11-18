//! A rosette (the pineapple): stiff, narrow leaves fanning up and out from
//! the foot, and in the middle one stalk bearing a head of flowers, then one
//! fruit sitting on it with a tuft of leaves on top.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::{Dress, Look, Show, around, out};

/// A rosette.
pub(super) fn rosette(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let leaves = 16;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let rise = between(rng, 0.5, 1.8);
        let length = look.radius.max(look.height) * between(rng, 0.7, 0.9);
        let width = (look.leaf_size * 0.25).max(0.3);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(
            Vec3::ZERO,
            out(angle, rise),
            (length, width, width * 0.4),
            shade,
        );
    }
    let top = Vec3::Y * look.height * 0.55;
    if dress.bare() {
        return;
    }
    let thick = (look.fruit_size * 0.2).max(0.15);
    pieces.stem(Vec3::ZERO, top, (thick, thick), look.leaf * 0.8);
    dress.sits = true;
    match look.show {
        Show::Flowers => {
            let s = look.fruit_size * 0.8;
            let head = top + Vec3::Y * s;
            pieces.ball(head, Vec3::new(s * 0.8, s, s * 0.8), 0.0, look.flower);
        }
        Show::Fruit(colour) => {
            dress.fruit(pieces, top, colour);
            // The fruit's crown of leaves, above where it sits.
            let s = look.fruit_size;
            let crown = top + Vec3::Y * s * 2.3;
            for i in 0..5 {
                let angle = around(i, 5, rng);
                let size = (s * 0.9, s * 0.25, s * 0.1);
                pieces.leaf(crown - Vec3::Y * s * 0.3, out(angle, 2.0), size, look.leaf);
            }
        }
        Show::Leaves => {}
    }
}
