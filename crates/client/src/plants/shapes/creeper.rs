//! Low mats over the ground.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, between, flower_head, out};

/// Small leaves lying nearly flat all over its width, a little higher towards
/// the middle; flowering, flowers dotted over the top.
pub(super) fn creeper(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let leaves = plan.count(18);
    let low = plan.height.max(1.0);
    let mut tops = Vec::new();
    for _ in 0..leaves {
        let angle = between(rng, 0.0, TAU);
        let far = between(rng, 0.0, 1.0).sqrt() * plan.radius * 0.75;
        let up = low * between(rng, 0.25, 0.6) * (1.0 - far / plan.radius * 0.6);
        let at = out(angle, 0.0) * far + Vec3::Y * up;
        let way = out(between(rng, 0.0, TAU), between(rng, -0.1, 0.25));
        let long = plan.leaf * between(rng, 0.8, 1.1);
        pieces.leaf(at, way, (long, long * 0.55, 0.2), plan.leaf_colour);
        tops.push(at);
    }
    if plan.growth.flowers {
        for at in tops.iter().step_by(3) {
            flower_head(pieces, plan, *at + Vec3::Y * 0.3, rng);
        }
    }
}
