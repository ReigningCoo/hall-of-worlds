//! Grasses, reeds and sedges; and water plants' upright leaves on stalks.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, around, between, out, top};

/// Straw's colour, for seed heads of grasses with no flowers to see.
const STRAW: Vec3 = Vec3::new(0.55, 0.45, 0.2);

/// Narrow blades from the foot, bending out; flowering, a few taller stems
/// with seed heads.
pub(super) fn grass(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let blades = plan.count(14);
    let wide = (plan.radius * 0.1).max(0.5);
    for i in 0..blades {
        let angle = around(i, blades, rng);
        let foot = out(angle, 0.0) * plan.radius * 0.15;
        let lean = between(rng, 0.15, 0.5);
        let way = out(angle, 1.0 / lean);
        let long = plan.height * between(rng, 0.6, 1.0);
        pieces.leaf(foot, way, (long, wide, 0.2), plan.leaf_colour);
    }
    if !plan.growth.flowers {
        return;
    }
    let colour = plan.flower.unwrap_or(STRAW);
    for _ in 0..3 {
        let angle = between(rng, 0.0, TAU);
        let tip = plan.leaned(plan.height * 1.05) + out(angle, 0.0) * plan.radius * 0.3;
        pieces.stem(Vec3::ZERO, tip, (0.2, 0.15), plan.stem_colour());
        let size = Vec3::new(0.45, plan.height * 0.1, 0.45);
        pieces.blob(tip - Vec3::Y * size.y * 0.5, size, colour);
    }
}

/// Stalks rising to most of its height, each with a broad leaf held up at its
/// end, and flowers above them.
pub(super) fn aquatic(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let stalks = plan.count(6);
    for i in 0..stalks {
        let angle = around(i, stalks, rng);
        let tip =
            plan.leaned(plan.height * between(rng, 0.5, 0.8)) + out(angle, 0.0) * plan.radius * 0.5;
        pieces.stem(Vec3::ZERO, tip, (0.25, 0.18), plan.stem_colour());
        let long = plan.leaf * between(rng, 1.0, 1.3);
        pieces.leaf(
            tip,
            out(angle, 0.6),
            (long, long * 0.6, 0.25),
            plan.leaf_colour,
        );
    }
    let tip = plan.leaned(plan.height);
    pieces.stem(Vec3::ZERO, tip, (0.25, 0.18), plan.stem_colour());
    top(pieces, plan, tip, rng);
}
