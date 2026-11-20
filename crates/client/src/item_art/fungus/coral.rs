//! Branches and frills: cauliflower fungus as a mound of curly lobes, hen of
//! the woods (pored underneath) as a rosette of small overlapping fans.

use bevy::prelude::*;
use shared::fungus::Under;

use super::ball::ball_profile;
use super::bracket::shelf;
use super::lathe::{Round, lathe};
use super::model::{Build, FungusPiece};
use super::paint::coloured;

pub(super) fn coral(build: &Build, n: usize) -> Vec<FungusPiece> {
    match build.def.look.under {
        Under::Pores => rosette(build, n),
        _ => lobes(build, n),
    }
}

/// Small fans spiralling up and out from a lump in the middle.
fn rosette(build: &Build, n: usize) -> Vec<FungusPiece> {
    let heart = lathe(&ball_profile(0.6), false, |_, _| Round {
        out: 1.0,
        colour: build.colours.body * 0.85,
    });
    let heart = FungusPiece::new(heart).at(Transform::from_scale(Vec3::new(0.7, 0.6, 0.7)));
    let mut pieces = vec![heart];
    for k in 0..14 {
        let a = k as f32 * 2.399_963 + build.variety.wobble(n, 1);
        let out = 0.34 * (k as f32 / 14.0).sqrt();
        let at = Vec3::new(a.cos() * out, 0.3 - out * 0.6, a.sin() * out);
        // Each fan's front (+z) turned to face out from the middle.
        let face = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2 - a);
        let place =
            Transform::from_translation(at).with_rotation(face * Quat::from_rotation_x(-0.25));
        let fans = shelf(build, n * 16 + k, 0.4 + out * 0.3);
        pieces.extend(fans.into_iter().map(|piece| piece.moved(place)));
    }
    pieces
}

/// Curly lobes heaped in a dome.
fn lobes(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let v = build.variety;
    (0..40)
        .map(|k| {
            let a = k as f32 * 2.399_963;
            let out = (k as f32 / 40.0).sqrt() * 0.42;
            let up = (0.45 * (1.0 - (out / 0.45).powi(2))).max(0.0).sqrt() * 0.5;
            let at = Vec3::new(a.cos() * out, up, a.sin() * out);
            let shade = 0.82 + 0.25 * v.share(n * 50 + k, 2);
            let mesh = coloured(Sphere::new(1.0).mesh().uv(10, 6), c.body * shade);
            let turn = Quat::from_euler(
                EulerRot::YXZ,
                v.wobble(k, 3) * 3.0,
                v.wobble(k, 4) * 1.2,
                v.wobble(k, 5) * 1.2,
            );
            let size = Vec3::new(0.13, 0.045, 0.09) * (0.8 + 0.4 * v.share(k, 6));
            FungusPiece::new(mesh).at(Transform::from_translation(at)
                .with_rotation(turn)
                .with_scale(size))
        })
        .collect()
}
