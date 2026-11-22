//! Things hanging from a peg rail: bundles of herbs drying, hams, strings of
//! onions, and coat pegs (perhaps with a cloak on one). In a hanging's frame,
//! as `trophies`. Sizes are first guesses.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::colours::{dark_wood, painted};
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;
/// How high a peg rail runs.
const RAIL: f32 = C * 0.82;

/// A rail along the wall with `pegs` pegs, evenly spaced: where each peg's
/// end is.
fn rail(model: &mut Model, pegs: usize, high: f32) -> Vec<Vec3> {
    let wood = dark_wood();
    let half = 0.36 * C;
    model.block(
        Vec3::new(-half, high - 0.025 * C, 0.0),
        Vec3::new(half, high + 0.025 * C, 0.02 * C),
        wood,
    );
    let out = Quat::from_rotation_x(FRAC_PI_2);
    (0..pegs)
        .map(|i| {
            let x = -half + 2.0 * half * (i as f32 + 0.5) / pegs as f32;
            let peg = Vec3::new(x, high, 0.045 * C);
            model.cylinder(peg, (0.01 * C, 0.05 * C), out, wood);
            peg + Vec3::Z * 0.02 * C
        })
        .collect()
}

/// Bundles of herbs tied at their stems, hanging head down.
pub(super) fn herb_bundles(model: &mut Model, variant: u8) {
    let greens = [
        srgb(0.42, 0.5, 0.36),
        srgb(0.22, 0.38, 0.18),
        srgb(0.5, 0.42, 0.6),
        srgb(0.6, 0.58, 0.3),
    ];
    let twine = srgb(0.62, 0.52, 0.34);
    for (i, peg) in rail(model, 4, RAIL).into_iter().enumerate() {
        let colour = greens[(i + usize::from(variant)) % greens.len()];
        let length = 0.16 * C + (i % 2) as f32 * 0.04 * C;
        model.cone(
            peg - Vec3::Y * (0.02 * C + length / 2.0),
            (0.05 * C, length),
            colour,
        );
        model.cylinder(
            peg - Vec3::Y * 0.02 * C,
            (0.015 * C, 0.025 * C),
            Quat::IDENTITY,
            twine,
        );
    }
}

/// Hams on strings, from two or three pegs.
pub(super) fn hams(model: &mut Model, variant: u8) {
    let pegs = 2 + usize::from(variant % 2);
    let (meat, skin) = (srgb(0.55, 0.25, 0.18), srgb(0.82, 0.72, 0.55));
    let string = srgb(0.7, 0.62, 0.45);
    for peg in rail(model, pegs, RAIL) {
        let top = peg - Vec3::Y * 0.12 * C;
        model.stick((peg, top), 0.006 * C, string);
        model.cylinder(
            top - Vec3::Y * 0.02 * C,
            (0.015 * C, 0.04 * C),
            Quat::IDENTITY,
            skin,
        );
        model.ball(
            top - Vec3::Y * 0.12 * C,
            Vec3::new(0.1, 0.17, 0.08) * C,
            meat,
        );
    }
}

/// Strings of onions plaited on straw, from three pegs.
pub(super) fn onions(model: &mut Model, variant: u8) {
    let colours = [srgb(0.72, 0.48, 0.2), srgb(0.5, 0.18, 0.28)];
    let straw = srgb(0.78, 0.68, 0.38);
    for (s, peg) in rail(model, 3, RAIL).into_iter().enumerate() {
        let colour = colours[(s + usize::from(variant)) % 2];
        let bottom = peg - Vec3::Y * 0.4 * C;
        model.stick((peg, bottom), 0.012 * C, straw);
        for i in 0..6 {
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            let at = peg + Vec3::new(side * 0.025 * C, -0.06 * C * (i as f32 + 1.0), 0.02 * C);
            model.ball(at, Vec3::splat(0.05 * C), colour);
        }
    }
}

/// A board of four coat pegs, a cloak on one of them in some variants.
pub(super) fn coat_pegs(model: &mut Model, variant: u8) {
    let pegs = rail(model, 4, 0.68 * C);
    if variant.is_multiple_of(2) {
        let (cloth, _) = painted(variant / 2);
        let peg = pegs[usize::from(variant / 2) % pegs.len()];
        model.block(
            peg + Vec3::new(-0.06 * C, -0.42 * C, -0.04 * C),
            peg + Vec3::new(0.06 * C, 0.01 * C, 0.0),
            cloth,
        );
    }
}
