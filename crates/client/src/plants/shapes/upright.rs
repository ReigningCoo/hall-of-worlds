//! Upright herbs: a spike of flowers up one stem, and leafy clumps of stems.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, around, between, out, stem_radius, top};

/// One stem with leaves at its foot and up it, flowers up its top part.
pub(super) fn spike(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let thick = stem_radius(plan.height);
    let tip = plan.leaned(plan.height);
    pieces.stem(Vec3::ZERO, tip, (thick, thick * 0.5), plan.stem_colour());
    let basal = plan.count(5);
    for i in 0..basal {
        let long = plan.leaf * between(rng, 1.0, 1.3);
        let way = out(around(i, basal, rng), between(rng, 0.2, 0.5));
        pieces.leaf(
            Vec3::Y * 0.3,
            way,
            (long, long * 0.35, 0.25),
            plan.leaf_colour,
        );
    }
    let up_it = plan.count(4);
    for i in 0..up_it {
        let at = plan.leaned(plan.height * (0.15 + 0.4 * i as f32 / up_it as f32));
        let long = plan.leaf * 0.7;
        let way = out(around(i, up_it, rng), 0.5);
        pieces.leaf(at, way, (long, long * 0.35, 0.2), plan.leaf_colour);
    }
    if !plan.growth.flowers {
        if plan.growth.size >= 1.0 {
            let middle = plan.leaned(plan.height * 0.85);
            let size = Vec3::new(0.5, plan.height * 0.12, 0.5);
            pieces.blob(middle, size, plan.leaf_colour * 0.85);
        }
        return;
    }
    let Some(colour) = plan.flower else {
        return;
    };
    // Flowers round the top two fifths, in a climbing spiral.
    let flowers = 9;
    let size = plan.flower_size;
    let start = between(rng, 0.0, TAU);
    for i in 0..flowers {
        let share = i as f32 / flowers as f32;
        let angle = start + share * TAU * 2.4;
        let at = plan.leaned(plan.height * (0.6 + 0.4 * share));
        let side = Vec3::new(angle.cos(), 0.0, angle.sin()) * (thick + size * 0.6);
        let shrink = 1.0 - share * 0.4;
        pieces.blob(at + side, Vec3::splat(size * shrink), colour);
    }
}

/// Several stems, leafy all the way up, each with its flowers at the top.
pub(super) fn clump(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let stems = plan.count(5);
    let thick = stem_radius(plan.height) * 0.8;
    for i in 0..stems {
        let angle = around(i, stems, rng);
        let spread = between(rng, 0.15, 0.5) * plan.radius;
        let tall = plan.height * between(rng, 0.7, 1.0);
        let foot = out(angle, 0.0) * spread * 0.2;
        let tip = plan.leaned(tall) + out(angle, 0.0) * spread;
        pieces.stem(foot, tip, (thick, thick * 0.6), plan.stem_colour());
        let leaves = plan.count(3);
        for j in 0..leaves {
            let share = (j as f32 + 0.5) / (leaves as f32 + 0.5);
            let at = foot.lerp(tip, share * 0.85);
            let side = angle + if j % 2 == 0 { 1.4 } else { -1.4 };
            let long = plan.leaf * (1.0 - share * 0.4);
            let way = out(side, between(rng, 0.0, 0.4));
            pieces.leaf(at, way, (long, long * 0.45, 0.25), plan.leaf_colour);
        }
        top(pieces, plan, tip, rng);
    }
}
