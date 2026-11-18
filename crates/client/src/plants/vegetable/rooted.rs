//! Plants over something swollen in the ground. Leaves over a root (turnip,
//! carrot, beetroot) stand up and out from the foot, the root's shoulder
//! showing at the ground when it is full grown. A bulb clump (onion, camas)
//! is a fan of narrow upright leaves over its bulb.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::{Crop, Look, around, out};

/// Leaves over a root.
pub(super) fn root(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let leaves = ((8.0 * look.grown).round() as usize).max(3);
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let rise = between(rng, 1.2, 3.0);
        let long = look.height * between(rng, 0.8, 1.05);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        let towards = out(angle, rise);
        // `leaf` takes a length along its way: as tall as the plant at most.
        let length = long * towards.length() / rise;
        let width = (length * 0.32).min(look.radius * 0.7).max(0.6);
        pieces.leaf(Vec3::ZERO, towards, (length, width, 0.2), shade);
    }
    crop.shoulder(pieces);
}

/// A bulb clump.
pub(super) fn bulbs(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let leaves = ((7.0 * look.grown).round() as usize).max(3);
    let thick = (look.radius * 0.08).clamp(0.2, 0.5);
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let tip = out(angle, 0.0) * look.radius * between(rng, 0.3, 0.8)
            + Vec3::Y * look.height * between(rng, 0.75, 1.0);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.stem(Vec3::Y * 0.2, tip, (thick, thick * 0.3), shade);
    }
    crop.shoulder(pieces);
}
