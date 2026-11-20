//! Clubs or fingers standing up: one club rising from a caterpillar's body for
//! the kinds that grow on insects, a few fingers from one foot for the rest,
//! more for a branching coral.

use bevy::prelude::*;
use shared::fungus::Niche;

use super::lathe::{Round, lathe};
use super::model::{Build, FungusPiece};
use super::paint::speckle;

const ROWS: usize = 12;

/// One club `tall` high: a thin stalk swelling into a rounded head.
fn club(colour: Vec3, tip: Vec3, tall: f32) -> FungusPiece {
    let mut profile = vec![Vec2::ZERO];
    for k in 0..=ROWS {
        let t = k as f32 / ROWS as f32;
        let head = ((t - 0.35) / 0.65).clamp(0.0, 1.0);
        let round = (1.0 - ((t - 0.88) / 0.12).clamp(0.0, 1.0).powi(2)).sqrt();
        profile.push(Vec2::new((0.04 + 0.07 * head.powf(0.5)) * round, t * tall));
    }
    profile.push(Vec2::new(0.0, tall));
    let rows = profile.len() as f32;
    let mesh = lathe(&profile, false, |row, i| Round {
        out: 1.0,
        colour: colour.lerp(tip, (row as f32 / rows).powi(3)) * (0.94 + 0.08 * speckle(row, i, 4)),
    });
    FungusPiece::new(mesh)
}

pub(super) fn clubs(build: &Build, n: usize) -> Vec<FungusPiece> {
    let def = build.def;
    let c = build.colours;
    let insect = def.niches.contains(&Niche::Insects);
    let fingers = match (insect, def.look.colours.len()) {
        (true, _) => 1,
        (false, 1) => 5,
        _ => 3,
    };
    let tip = match insect {
        true => c.body * 1.15,
        false => c.under.lerp(c.body, 0.6),
    };
    let mut pieces = Vec::new();
    for k in 0..fingers {
        let spread = k as f32 - (fingers as f32 - 1.0) / 2.0;
        let lean = spread * 0.28 + build.variety.wobble(n * 8 + k, 1) * 0.1;
        let tall = 0.9 - spread.abs() * 0.12 + build.variety.wobble(n * 8 + k, 2) * 0.08;
        let turn =
            Quat::from_rotation_z(-lean) * Quat::from_rotation_x(build.variety.wobble(k, 3) * 0.2);
        let at = Vec3::new(spread * 0.06, 0.0, 0.0);
        let piece = club(c.body, tip, tall).at(Transform::from_translation(at).with_rotation(turn));
        pieces.push(piece);
    }
    if insect {
        pieces.push(caterpillar(c.under));
    }
    pieces
}

/// A caterpillar's segmented body lying along x, its head where the club rises.
fn caterpillar(colour: Vec3) -> FungusPiece {
    let segments = 9.0;
    let profile: Vec<Vec2> = (0..=36)
        .map(|k| {
            let t = k as f32 / 36.0;
            let end = (1.0 - (2.0 * t - 1.0).powi(8)).sqrt();
            let bulge = 1.0 + 0.15 * (t * segments * std::f32::consts::PI).sin().abs();
            Vec2::new(0.075 * end * bulge, t * 0.7)
        })
        .collect();
    let mesh = lathe(&profile, false, |row, _| Round {
        out: 1.0,
        colour: colour * (0.85 + 0.15 * ((row / 4) % 2) as f32),
    });
    let lie = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2);
    FungusPiece::new(mesh).at(Transform::from_xyz(-0.05, 0.05, 0.0).with_rotation(lie))
}
