//! Upright stems carrying flat umbrellas of tiny flowers.

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, around, between, bud, out, stem_radius};

/// Feathery leaves at the foot, one to three stems, and on each an umbrella
/// of rays under a flat head of flowers.
pub(super) fn umbel(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let basal = plan.count(6);
    for i in 0..basal {
        let long = plan.leaf * between(rng, 0.9, 1.2);
        let way = out(around(i, basal, rng), between(rng, 0.3, 0.7));
        pieces.leaf(
            Vec3::Y * 0.3,
            way,
            (long, long * 0.5, 0.2),
            plan.leaf_colour,
        );
    }
    let stems = 1 + (rng.next_u64() % 3) as usize;
    let thick = stem_radius(plan.height);
    for _ in 0..stems {
        let off = Vec3::new(between(rng, -1.0, 1.0), 0.0, between(rng, -1.0, 1.0));
        let tall = plan.height * between(rng, 0.8, 1.0);
        let tip = plan.leaned(tall) + off * plan.radius * 0.3;
        pieces.stem(Vec3::ZERO, tip, (thick, thick * 0.6), plan.stem_colour());
        let at = tip * 0.5;
        let long = plan.leaf * 0.6;
        let way = out(between(rng, 0.0, std::f32::consts::TAU), 0.6);
        pieces.leaf(at, way, (long, long * 0.5, 0.2), plan.leaf_colour);
        if plan.growth.flowers {
            head(pieces, plan, tip, rng);
        } else if plan.growth.size >= 1.0 {
            bud(pieces, plan, tip);
        }
    }
}

/// Rays out from the stem's top, and the flat head over them.
fn head(pieces: &mut Pieces, plan: &Plan, tip: Vec3, rng: &mut Rng) {
    let Some(colour) = plan.flower else {
        return;
    };
    let wide = (plan.flower_size * 3.0).max(plan.radius * 0.35);
    let rays = 6;
    let top = tip + Vec3::Y * wide * 0.45;
    for i in 0..rays {
        let end = top + out(around(i, rays, rng), 0.0) * wide * 0.75;
        pieces.stem(tip, end, (0.15, 0.12), plan.stem_colour());
    }
    let turn = between(rng, 0.0, 1.6);
    pieces.blob_turned(top, Vec3::new(wide, wide * 0.22, wide), turn, colour);
}
