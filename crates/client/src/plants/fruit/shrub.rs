//! Shrubs and canes. A shrub is several stems from the foot, each ending in
//! a mass of leaves. Canes (brambles) arch up and over, leafy along their
//! length, flowering and fruiting towards their ends.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Dress, Look, Show, around, out};

/// A shrub.
pub(super) fn shrub(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let stems = 4 + (rng.next_u64() % 3) as usize;
    let thick = (look.height * 0.03).clamp(0.2, 0.6);
    let size = Vec3::new(look.radius * 0.55, look.height * 0.3, look.radius * 0.55);
    let mut masses = vec![Vec3::Y * look.height * 0.68];
    for i in 0..stems {
        let angle = around(i, stems, rng);
        let tip = Vec3::Y * look.height * between(rng, 0.5, 0.65)
            + out(angle, 0.0) * look.radius * between(rng, 0.4, 0.55);
        pieces.stem(Vec3::ZERO, tip, (thick, thick * 0.4), WOOD);
        masses.push(tip);
    }
    let count = if look.show == Show::Flowers { 4 } else { 2 };
    for at in masses {
        let own = size * between(rng, 0.85, 1.15);
        let turn = between(rng, 0.0, 1.6);
        pieces.ball(at, own, turn, look.leaf * between(rng, 0.85, 1.1));
        dress.over(pieces, at, own, count);
    }
}

/// Bramble canes.
pub(super) fn canes(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let canes = 5 + (rng.next_u64() % 3) as usize;
    let thick = (look.height * 0.015).clamp(0.12, 0.3);
    let cane = look.leaf * Vec3::new(1.6, 0.6, 0.6);
    for i in 0..canes {
        let angle = around(i, canes, rng);
        let tall = look.height * between(rng, 0.75, 1.0);
        let reach = look.radius * between(rng, 0.75, 1.0);
        // Up, over, and down again.
        let points = [
            Vec3::ZERO,
            out(angle, 0.0) * reach * 0.25 + Vec3::Y * tall * 0.8,
            out(angle, 0.0) * reach * 0.65 + Vec3::Y * tall,
            out(angle, 0.0) * reach + Vec3::Y * tall * 0.55,
        ];
        for pair in points.windows(2) {
            pieces.stem(pair[0], pair[1], (thick, thick * 0.8), cane);
            let middle = (pair[0] + pair[1]) / 2.0;
            let across = look.leaf_size.min(look.radius * 0.25);
            let leaves = Vec3::new(across, across * 0.6, across);
            let shade = look.leaf * between(rng, 0.85, 1.1);
            pieces.ball(middle, leaves, between(rng, 0.0, 1.6), shade);
        }
        for &at in &points[2..] {
            dress.at(pieces, at);
        }
    }
}
