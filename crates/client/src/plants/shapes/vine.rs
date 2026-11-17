//! Climbing and trailing plants: stems winding up round each other.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, between, flower_head, out};

/// Points along each stem.
const JOINTS: usize = 6;

/// Two or three stems winding up to the plant's height, a leaf at every
/// joint, and flowers (or fruit) hanging from some.
pub(super) fn vine(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let stems = 2 + (rng.next_u64() % 2) as usize;
    let joints = plan.count(JOINTS).max(2);
    for _ in 0..stems {
        let start = between(rng, 0.0, TAU);
        let turn = between(rng, 0.9, 1.4);
        let wide = plan.radius * between(rng, 0.35, 0.6);
        let mut last = Vec3::ZERO;
        for joint in 1..=joints {
            let share = joint as f32 / JOINTS as f32;
            let angle = start + turn * joint as f32;
            let at = plan.leaned(plan.height * share) + out(angle, 0.0) * wide;
            pieces.stem(last, at, (0.22, 0.18), plan.stem_colour());
            let long = plan.leaf * between(rng, 0.8, 1.1);
            let way = out(angle + between(rng, -0.5, 0.5), -0.2);
            pieces.leaf(at, way, (long, long * 0.6, 0.2), plan.leaf_colour);
            if plan.growth.flowers && joint % 2 == 0 {
                let hanging = at + out(angle, 0.0) * plan.flower_size - Vec3::Y * plan.flower_size;
                flower_head(pieces, plan, hanging, rng);
            }
            last = at;
        }
    }
}
