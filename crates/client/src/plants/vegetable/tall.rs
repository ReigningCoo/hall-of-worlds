//! The tallest vegetable plants, drawn smaller than life as fruit trees are.
//! A bamboo grove is a few culms with tufts of leaves up their upper half,
//! shoots coming up round their foot once full grown. A thorny tree
//! (mesquite) is a trunk forking under a wide, flat crown, its pods hanging
//! below it.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::look::Show;
use super::{Crop, Look, around, out};

/// A bamboo grove.
pub(super) fn bamboo(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let culms = 4;
    let cane = look.leaf * Vec3::new(1.3, 1.1, 0.6);
    for i in 0..culms {
        let angle = around(i, culms, rng);
        let foot = out(angle, 0.0) * look.radius * 0.25;
        let tall = look.height * between(rng, 0.75, 1.0);
        let top = foot + out(angle, 0.0) * look.radius * 0.2 + Vec3::Y * tall;
        pieces.stem(foot, top, (0.35, 0.25), cane);
        for t in [0.55, 0.75, 0.95] {
            let at = foot + (top - foot) * t + out(angle + t * 3.0, 0.0) * look.radius * 0.2;
            let s = look.radius * 0.28;
            let shade = look.leaf * between(rng, 0.85, 1.1);
            pieces.ball(at, Vec3::new(s, s * 0.5, s), between(rng, 0.0, 1.6), shade);
        }
    }
    if crop.show() != Some(Show::Ripe) {
        return;
    }
    for i in 0..3 {
        let angle = i as f32 / 3.0 * TAU + crop.between(0.2, 0.8);
        let foot = out(angle, 0.0) * look.radius * 0.5;
        let size = look.part_size;
        pieces.stem(
            foot,
            foot + Vec3::Y * size.x * 2.0,
            (size.y, 0.1),
            crop.colour(),
        );
    }
}

/// A thorny tree.
pub(super) fn thorny_tree(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let fork = Vec3::Y * look.height * 0.4;
    let thick = (look.height * 0.04).clamp(0.3, 1.0);
    pieces.stem(Vec3::ZERO, fork, (thick, thick * 0.7), WOOD);
    let branches = 4;
    let crown = look.height * 0.8;
    let size = Vec3::new(look.radius * 0.45, look.height * 0.14, look.radius * 0.45);
    let mut masses = vec![Vec3::Y * crown];
    for i in 0..branches {
        let angle = around(i, branches, rng);
        let tip = out(angle, 0.0) * look.radius * between(rng, 0.5, 0.65)
            + Vec3::Y * crown * between(rng, 0.85, 1.0);
        pieces.stem(fork, tip, (thick * 0.6, thick * 0.3), WOOD);
        masses.push(tip);
    }
    for at in masses {
        let own = size * between(rng, 0.85, 1.15);
        pieces.ball(
            at,
            own,
            between(rng, 0.0, 1.6),
            look.leaf * between(rng, 0.85, 1.1),
        );
        for _ in 0..2 {
            let angle = crop.between(0.0, TAU);
            // At the crown's edge, so they show from above.
            let below = at + out(angle, 0.0) * own.x * 0.95 - Vec3::Y * own.y * 0.2;
            let on_top = at + out(angle, 0.0) * own.x * 0.5 + Vec3::Y * own.y * 0.8;
            if crop.show() == Some(Show::Flowers) {
                crop.flower(pieces, on_top);
            } else {
                crop.hang(pieces, below);
            }
        }
    }
}
