//! Leaves from the ground: rosettes, and fleshy succulents.

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, around, between, out, stem_radius, top};

/// A ring of leaves on the ground, with flower stalks rising from it.
pub(super) fn rosette(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let length = plan.leaf.max(plan.radius * 0.9);
    ring(pieces, plan, rng, length, 0.12);
    stalks(pieces, plan, rng);
}

/// Thick, fleshy leaves, rising more steeply, with flat heads of flowers.
pub(super) fn succulent(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let length = plan.leaf.max(plan.radius * 0.8);
    ring(pieces, plan, rng, length, 0.45);
    stalks(pieces, plan, rng);
}

/// A ring of leaves, `thickness` deep for each unit across.
fn ring(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng, length: f32, thickness: f32) {
    let leaves = plan.count(11);
    let steep = 1.0 + thickness;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let rise = between(rng, 0.1, 0.45) * steep;
        let long = length * between(rng, 0.75, 1.1);
        let wide = long * 0.4;
        let base = Vec3::Y * 0.3;
        pieces.leaf(
            base,
            out(angle, rise),
            (long, wide, wide * thickness + 0.2),
            plan.leaf_colour,
        );
    }
}

/// One to three stalks, up to the plant's height, each with its flower or bud.
fn stalks(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    if plan.growth.size < 1.0 || plan.height < plan.leaf * 0.5 {
        return;
    }
    let count = 1 + (rng.next_u64() % 3) as usize;
    let thick = stem_radius(plan.height) * 0.7;
    for _ in 0..count {
        let tall = plan.height * between(rng, 0.8, 1.0);
        let off = Vec3::new(between(rng, -0.6, 0.6), 0.0, between(rng, -0.6, 0.6));
        let tip = plan.leaned(tall) + off * plan.radius * 0.4;
        pieces.stem(off * 0.5, tip, (thick, thick * 0.6), plan.stem_colour());
        top(pieces, plan, tip, rng);
    }
}
