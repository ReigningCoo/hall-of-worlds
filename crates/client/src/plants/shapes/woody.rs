//! Woody plants: bushes of many branches, and small trees on a trunk. Their
//! leaves are drawn as masses, too many to draw one by one.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::plan::WOOD;
use super::{Pieces, Plan, around, between, flower_head, out};

/// Branches from the foot, each ending in a mass of leaves.
pub(super) fn bush(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let branches = plan.count(5).max(2);
    let thick = (plan.height * 0.03).clamp(0.25, 0.7);
    let mut crowns = Vec::new();
    for i in 0..branches {
        let angle = around(i, branches, rng);
        let tip = plan.leaned(plan.height * between(rng, 0.6, 0.8))
            + out(angle, 0.0) * plan.radius * between(rng, 0.4, 0.6);
        pieces.stem(Vec3::ZERO, tip, (thick, thick * 0.4), WOOD);
        crowns.push(tip);
    }
    crowns.push(plan.leaned(plan.height * 0.75));
    let size = Vec3::new(plan.radius * 0.5, plan.height * 0.28, plan.radius * 0.5);
    foliage(pieces, plan, &crowns, size, rng);
}

/// A trunk to half its height, then a crown of leaves round the top.
pub(super) fn tree(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let thick = (plan.height * 0.045).clamp(0.3, 1.2);
    let fork = plan.leaned(plan.height * 0.5);
    pieces.stem(Vec3::ZERO, fork, (thick, thick * 0.7), WOOD);
    let middle = plan.leaned(plan.height * 0.72);
    let branches = plan.count(4).max(2);
    let mut crowns = vec![middle + Vec3::Y * plan.height * 0.08];
    for i in 0..branches {
        let angle = around(i, branches, rng);
        let tip = middle + out(angle, 0.0) * plan.radius * 0.55;
        pieces.stem(fork, tip, (thick * 0.6, thick * 0.3), WOOD);
        crowns.push(tip);
    }
    let size = Vec3::new(plan.radius * 0.5, plan.height * 0.2, plan.radius * 0.5);
    foliage(pieces, plan, &crowns, size, rng);
}

/// A mass of leaves of about `size` at each of `crowns`, each a shade of its
/// own; flowering, flowers (or fruit) on their tops and sides.
fn foliage(pieces: &mut Pieces, plan: &Plan, crowns: &[Vec3], size: Vec3, rng: &mut Rng) {
    for &crown in crowns {
        let shade = between(rng, 0.85, 1.1);
        let own = size * between(rng, 0.8, 1.15);
        let turn = between(rng, 0.0, 1.6);
        pieces.blob_turned(crown, own, turn, plan.leaf_colour * shade);
        if plan.growth.flowers {
            for _ in 0..3 {
                let angle = between(rng, 0.0, std::f32::consts::TAU);
                let up = between(rng, 0.2, 0.8);
                let side = out(angle, 0.0) * (1.0 - up * 0.6);
                let at = crown + side * own.x * 0.8 + Vec3::Y * up * own.y;
                flower_head(pieces, plan, at, rng);
            }
        }
    }
}
