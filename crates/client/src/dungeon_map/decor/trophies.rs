//! Trophies and arms on walls: antlers and a boar's head on plaques, a
//! painted shield, and two swords crossed behind a small shield. In a
//! hanging's frame: `x` along the wall, `y` up from the floor, `z` out from
//! the wall's face. Sizes are first guesses.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::colours::{bone, dark_wood, iron, painted, steel};
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;
/// How high a trophy's middle hangs.
const MIDDLE: f32 = C * 0.6;

/// A plaque: a shield-shaped board with a point below.
fn plaque(model: &mut Model) {
    let wood = dark_wood();
    model.block(
        Vec3::new(-0.09 * C, MIDDLE - 0.08 * C, 0.0),
        Vec3::new(0.09 * C, MIDDLE + 0.1 * C, 0.025 * C),
        wood,
    );
    let point = Quat::from_rotation_z(FRAC_PI_2 / 2.0);
    let side = 0.125 * C;
    let at = Vec3::new(0.0, MIDDLE - 0.08 * C, 0.0125 * C);
    model.cuboid(at, Vec3::new(side, side, 0.024 * C), point, wood);
}

/// A stag's antlers on a plaque, larger with each variant.
pub(super) fn antlers(model: &mut Model, variant: u8) {
    plaque(model);
    let skull = Vec3::new(0.0, MIDDLE + 0.02 * C, 0.06 * C);
    model.cuboid(
        skull,
        Vec3::new(0.07, 0.08, 0.06) * C,
        Quat::IDENTITY,
        bone(),
    );
    let antler = srgb(0.55, 0.42, 0.28);
    let spread = 1.0 + f32::from(variant % 3) * 0.15;
    for side in [-1.0f32, 1.0] {
        let x = |v: f32| v * side * spread;
        let base = skull + Vec3::new(x(0.03 * C), 0.03 * C, 0.0);
        let bend = base + Vec3::new(x(0.1 * C), 0.1 * C, 0.02 * C);
        let tip = bend + Vec3::new(x(0.02 * C), 0.14 * C, -0.01 * C);
        model.stick((base, bend), 0.018 * C, antler);
        model.stick((bend, tip), 0.015 * C, antler);
        // Tines branching up and in from the beam.
        for (at, reach) in [(0.4, 0.07), (0.8, 0.06)] {
            let from = base.lerp(bend, at);
            let to = from + Vec3::new(x(-0.02 * C), reach * C, 0.03 * C);
            model.stick((from, to), 0.012 * C, antler);
        }
        let crown = bend.lerp(tip, 0.6);
        model.stick(
            (crown, crown + Vec3::new(x(0.06 * C), 0.05 * C, 0.0)),
            0.012 * C,
            antler,
        );
    }
}

/// A boar's head on a plaque, tusks up.
pub(super) fn boar_head(model: &mut Model, variant: u8) {
    plaque(model);
    let hide = if variant.is_multiple_of(2) {
        srgb(0.3, 0.24, 0.2)
    } else {
        srgb(0.42, 0.32, 0.24)
    };
    let head = Vec3::new(0.0, MIDDLE + 0.01 * C, 0.08 * C);
    model.cuboid(head, Vec3::new(0.13, 0.12, 0.1) * C, Quat::IDENTITY, hide);
    let snout = head + Vec3::new(0.0, -0.035 * C, 0.08 * C);
    model.cuboid(
        snout,
        Vec3::new(0.07, 0.055, 0.08) * C,
        Quat::IDENTITY,
        hide,
    );
    let nose = snout + Vec3::new(0.0, 0.0, 0.045 * C);
    model.cylinder(
        nose,
        (0.03 * C, 0.012 * C),
        Quat::from_rotation_x(FRAC_PI_2),
        srgb(0.55, 0.38, 0.34),
    );
    for side in [-1.0f32, 1.0] {
        let root = snout + Vec3::new(side * 0.035 * C, -0.01 * C, 0.02 * C);
        let tip = root + Vec3::new(side * 0.02 * C, 0.05 * C, 0.02 * C);
        model.stick((root, tip), 0.01 * C, bone());
        let ear = head + Vec3::new(side * 0.06 * C, 0.07 * C, -0.01 * C);
        let tilt = Quat::from_rotation_z(-side * 0.5);
        model.cuboid(ear, Vec3::new(0.035, 0.05, 0.015) * C, tilt, hide);
        let eye = head + Vec3::new(side * 0.04 * C, 0.02 * C, 0.05 * C);
        model.ball(eye, Vec3::splat(0.018 * C), srgb(0.05, 0.04, 0.04));
    }
}

/// A round painted shield with an iron rim and boss, a band across it.
pub(super) fn shield(model: &mut Model, variant: u8) {
    let (field, band) = painted(variant);
    let flat = Quat::from_rotation_x(FRAC_PI_2);
    let middle = Vec3::new(0.0, MIDDLE, 0.0);
    model.cylinder(
        middle + Vec3::Z * 0.01 * C,
        (0.18 * C, 0.02 * C),
        flat,
        iron(),
    );
    model.cylinder(
        middle + Vec3::Z * 0.02 * C,
        (0.165 * C, 0.02 * C),
        flat,
        field,
    );
    let across = if variant.is_multiple_of(2) {
        Vec3::new(0.3, 0.06, 0.01)
    } else {
        Vec3::new(0.06, 0.3, 0.01)
    } * C;
    model.cuboid(middle + Vec3::Z * 0.032 * C, across, Quat::IDENTITY, band);
    model.ball(
        middle + Vec3::Z * 0.035 * C,
        Vec3::new(0.06, 0.06, 0.04) * C,
        iron(),
    );
}

/// Two swords crossed behind a small round shield.
pub(super) fn crossed_weapons(model: &mut Model, variant: u8) {
    let grip = srgb(0.3, 0.18, 0.1);
    for side in [-1.0f32, 1.0] {
        // From the hilt, low, up across to the point.
        let hilt = Vec3::new(side * 0.17 * C, MIDDLE - 0.2 * C, 0.015 * C);
        let point = Vec3::new(-side * 0.17 * C, MIDDLE + 0.2 * C, 0.015 * C);
        let guard = hilt.lerp(point, 0.16);
        model.stick((guard, point), 0.025 * C, steel());
        model.stick((hilt, guard), 0.02 * C, grip);
        let across = (point - hilt).normalize().cross(Vec3::Z) * 0.05 * C;
        model.stick((guard - across, guard + across), 0.018 * C, iron());
        model.ball(hilt, Vec3::splat(0.03 * C), iron());
    }
    let (field, _) = painted(variant);
    let flat = Quat::from_rotation_x(FRAC_PI_2);
    let middle = Vec3::new(0.0, MIDDLE, 0.04 * C);
    model.cylinder(middle, (0.09 * C, 0.015 * C), flat, field);
    model.ball(
        middle + Vec3::Z * 0.01 * C,
        Vec3::new(0.035, 0.035, 0.025) * C,
        iron(),
    );
}
