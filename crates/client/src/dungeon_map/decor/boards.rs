//! Flat things on walls: a tapestry on its rod, a hanging sign on a bracket,
//! a notice board with notes pinned up, and a candle sconce. In a hanging's
//! frame, as `trophies`. Sizes are first guesses.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::colours::{dark_wood, iron, light_wood, painted, paper, wax};
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;

/// A woven tapestry on a rod: a field, a border and a diamond in the middle.
pub(super) fn tapestry(model: &mut Model, variant: u8) {
    let (field, border) = painted(variant);
    let along = Quat::from_rotation_z(FRAC_PI_2);
    let rod = Vec3::new(0.0, 0.92 * C, 0.025 * C);
    model.cylinder(rod, (0.012 * C, 0.92 * C), along, dark_wood());
    for side in [-1.0, 1.0] {
        model.ball(
            rod + Vec3::X * side * 0.46 * C,
            Vec3::splat(0.03 * C),
            dark_wood(),
        );
    }
    let (low, high, half) = (0.22 * C, 0.9 * C, 0.4 * C);
    model.block(
        Vec3::new(-half, low, 0.005 * C),
        Vec3::new(half, high, 0.015 * C),
        field,
    );
    let band = 0.04 * C;
    let front = (0.015 * C, 0.02 * C);
    for (from, to) in [
        (Vec3::new(-half, low, 0.0), Vec3::new(half, low + band, 0.0)),
        (
            Vec3::new(-half, high - band, 0.0),
            Vec3::new(half, high, 0.0),
        ),
        (
            Vec3::new(-half, low, 0.0),
            Vec3::new(-half + band, high, 0.0),
        ),
        (Vec3::new(half - band, low, 0.0), Vec3::new(half, high, 0.0)),
    ] {
        model.block(from.with_z(front.0), to.with_z(front.1), border);
    }
    let diamond = Quat::from_rotation_z(FRAC_PI_2 / 2.0);
    let middle = Vec3::new(0.0, (low + high) / 2.0, 0.0175 * C);
    model.cuboid(middle, Vec3::new(0.2, 0.2, 0.006) * C, diamond, border);
}

/// A painted sign hanging from an iron bracket, standing out from the wall.
pub(super) fn sign(model: &mut Model, variant: u8) {
    let (paint, emblem) = painted(variant);
    let arm = 0.8 * C;
    model.block(
        Vec3::new(-0.01 * C, arm - 0.015 * C, 0.0),
        Vec3::new(0.01 * C, arm + 0.015 * C, 0.32 * C),
        iron(),
    );
    model.stick(
        (
            Vec3::new(0.0, arm - 0.14 * C, 0.0),
            Vec3::new(0.0, arm, 0.2 * C),
        ),
        0.012 * C,
        iron(),
    );
    let (top, bottom) = (arm - 0.05 * C, arm - 0.3 * C);
    for z in [0.1 * C, 0.27 * C] {
        model.stick(
            (Vec3::new(0.0, arm, z), Vec3::new(0.0, top, z)),
            0.006 * C,
            iron(),
        );
    }
    model.block(
        Vec3::new(-0.012 * C, bottom, 0.07 * C),
        Vec3::new(0.012 * C, top, 0.3 * C),
        paint,
    );
    // An emblem on each face: a tankard's shape.
    let middle = Vec3::new(0.0, (top + bottom) / 2.0, 0.185 * C);
    for side in [-1.0, 1.0] {
        model.cylinder(
            middle + Vec3::X * side * 0.014 * C,
            (0.045 * C, 0.008 * C),
            Quat::from_rotation_z(FRAC_PI_2),
            emblem,
        );
    }
}

/// A notice board in a frame, with notes pinned up by the variant.
pub(super) fn notice_board(model: &mut Model, variant: u8) {
    let (low, high, half) = (0.36 * C, 0.8 * C, 0.3 * C);
    model.block(
        Vec3::new(-half, low, 0.0),
        Vec3::new(half, high, 0.02 * C),
        dark_wood(),
    );
    let edge = 0.03 * C;
    model.block(
        Vec3::new(-half + edge, low + edge, 0.0),
        Vec3::new(half - edge, high - edge, 0.026 * C),
        light_wood(),
    );
    let notes = 3 + usize::from(variant % 3);
    let pin = srgb(0.7, 0.1, 0.1);
    for i in 0..notes {
        // Spread over the board, the same for each variant.
        let hash = (u32::from(variant) * 7 + i as u32 * 13) % 17;
        let x = -half + 0.08 * C + (i as f32 / notes as f32) * (2.0 * half - 0.16 * C);
        let y = low + 0.12 * C + (hash as f32 / 17.0) * (high - low - 0.24 * C);
        let size = Vec3::new(0.08 + (hash % 3) as f32 * 0.015, 0.1, 0.004) * C;
        let at = Vec3::new(x, y, 0.03 * C);
        model.cuboid(
            at,
            size,
            Quat::from_rotation_z((hash as f32 - 8.0) * 0.02),
            paper(),
        );
        model.ball(
            at + Vec3::new(0.0, 0.035 * C, 0.004 * C),
            Vec3::splat(0.012 * C),
            pin,
        );
    }
}

/// A candle sconce: an iron back plate, an arm out to a drip pan, a candle
/// and its flame.
pub(super) fn sconce(model: &mut Model) {
    let base = 0.58 * C;
    model.block(
        Vec3::new(-0.035 * C, base - 0.05 * C, 0.0),
        Vec3::new(0.035 * C, base + 0.09 * C, 0.012 * C),
        iron(),
    );
    let pan = Vec3::new(0.0, base, 0.11 * C);
    model.stick(
        (Vec3::new(0.0, base - 0.02 * C, 0.01 * C), pan),
        0.012 * C,
        iron(),
    );
    model.cylinder(pan, (0.035 * C, 0.01 * C), Quat::IDENTITY, iron());
    let height = 0.09 * C;
    let candle = pan + Vec3::Y * (0.005 * C + height / 2.0);
    model.cylinder(candle, (0.014 * C, height), Quat::IDENTITY, wax());
    model.flame(candle + Vec3::Y * height / 2.0, 0.045 * C);
}
