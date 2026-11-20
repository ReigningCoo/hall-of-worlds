//! A cup (elf cups, cellar cups): a short foot opening into a bowl with a
//! wavy rim, its outside one colour and its inside another.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};

const ROWS: usize = 10;

pub(super) fn cup(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let mut profile = vec![Vec2::ZERO, Vec2::new(0.05, 0.0), Vec2::new(0.05, 0.06)];
    for k in 1..=ROWS {
        let t = k as f32 / ROWS as f32;
        profile.push(Vec2::new(
            0.06 + 0.44 * (t * FRAC_PI_2).sin(),
            0.06 + 0.4 * t.powf(1.5),
        ));
    }
    let rim = profile.len();
    for k in (0..=ROWS).rev() {
        let t = k as f32 / ROWS as f32;
        profile.push(Vec2::new(
            0.46 * (t * FRAC_PI_2).sin(),
            0.18 + 0.3 * t.powf(1.5),
        ));
    }
    let rows = profile.len();
    let phase = build.variety.wobble(n, 1) * 3.0;
    let mesh = lathe(&profile, false, |row, i| {
        let near = match row < rim {
            true => row as f32 / rim as f32,
            false => (rows - row) as f32 / (rows - rim) as f32,
        };
        let out = 1.0 + 0.06 * (4.0 * angle(i) + phase).sin() * near.powi(3);
        let colour = match row < rim {
            true => c.body,
            false => c.under * (0.8 + 0.2 * near),
        };
        Round { out, colour }
    });
    vec![FungusPiece::new(mesh)]
}
