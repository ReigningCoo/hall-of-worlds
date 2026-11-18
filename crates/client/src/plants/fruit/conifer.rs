//! Conifers: tiers of needles narrowing up a trunk, or, for one that grows
//! nearly as wide as it is tall (the stone pine), an umbrella crown on a bare
//! trunk. Cones (or berries) over the outside, near the top when they grow
//! there.

use bevy::prelude::*;
use shared::fruit::FruitPosition;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Dress, Look};

/// A conifer.
pub(super) fn conifer(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let thick = (look.height * 0.035).clamp(0.25, 1.2);
    if look.wide > 0.55 {
        umbrella(pieces, look, thick, rng, dress);
        return;
    }
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * look.height,
        (thick, thick * 0.2),
        WOOD,
    );
    let tiers = 4;
    let foot = look.height * 0.2;
    let step = (look.height - foot) / tiers as f32;
    for tier in 0..tiers {
        let share = 1.0 - tier as f32 / tiers as f32 * 0.8;
        let at = Vec3::Y * (foot + step * (tier as f32 + 0.6));
        let size = Vec3::new(look.radius * share, step * 0.75, look.radius * share);
        let shade = between(rng, 0.85, 1.1);
        pieces.ball(at, size, between(rng, 0.0, 1.6), look.leaf * shade);
        let top = tier + 2 >= tiers;
        if look.fruits != FruitPosition::Tips || top {
            dress.over(pieces, at, size, 3);
        }
    }
}

/// A bare trunk under a wide, flat crown.
fn umbrella(pieces: &mut Pieces, look: &Look, thick: f32, rng: &mut Rng, dress: &mut Dress) {
    let half = look.height * 0.17;
    let middle = Vec3::Y * (look.height - half);
    pieces.stem(Vec3::ZERO, middle, (thick, thick * 0.6), WOOD);
    let size = Vec3::new(look.radius, half, look.radius);
    pieces.ball(middle, size, between(rng, 0.0, 1.6), look.leaf);
    for side in [-1.0, 1.0] {
        let at = middle + Vec3::new(side * look.radius * 0.45, -half * 0.3, 0.0);
        pieces.ball(at, size * 0.6, between(rng, 0.0, 1.6), look.leaf * 0.9);
    }
    dress.over(pieces, middle, size, 7);
}
