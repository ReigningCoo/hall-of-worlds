//! Jelly fungi (wood ear, witch's butter, snow fungus): glossy wavy lobes
//! heaped together.

use bevy::prelude::*;

use super::ball::ball_profile;
use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};

pub(super) fn jelly(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let v = build.variety;
    let profile = ball_profile(0.6);
    let rows = profile.len() as f32;
    (0..4)
        .map(|k| {
            let phase = v.wobble(n * 4 + k, 1) * 3.0;
            let shade = 0.85 + 0.2 * v.share(n * 4 + k, 2);
            let mesh = lathe(&profile, false, |row, i| {
                let up = row as f32 / rows;
                Round {
                    out: 1.0 + 0.2 * (5.0 * angle(i) + phase).sin() * up,
                    colour: c.body * shade * (0.8 + 0.25 * up),
                }
            });
            let a = k as f32 * 2.1 + v.wobble(n, 3);
            let out = if k == 0 { 0.0 } else { 0.3 };
            let at = Vec3::new(a.cos() * out, 0.0, a.sin() * out);
            let size = if k == 0 { 1.0 } else { 0.7 };
            let place = Transform::from_translation(at)
                .with_rotation(Quat::from_rotation_y(a))
                .with_scale(Vec3::splat(size));
            FungusPiece::new(mesh).at(place).shiny()
        })
        .collect()
}
