//! A funnel or trumpet (chanterelle, black trumpet): a stem widening into a
//! wavy rim, ridges or gills running down its outside, hollow inside: shallow,
//! or deep for a trumpet with a smooth outside.

use bevy::prelude::*;
use shared::fungus::Under;

use super::cap::underside;
use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};

const ROWS: usize = 12;

pub(super) fn funnel(build: &Build, n: usize) -> Vec<FungusPiece> {
    let look = &build.def.look;
    let c = build.colours;
    let depth = match look.under {
        Under::Smooth => 0.7,
        _ => 0.25,
    };
    let height = 0.95 * (1.0 + build.variety.wobble(n, 1) * 0.1);
    let mut profile = vec![Vec2::ZERO];
    for k in 0..=ROWS {
        let t = k as f32 / ROWS as f32;
        profile.push(Vec2::new(0.07 + 0.43 * t.powf(2.2), t * height));
    }
    let rim = profile.len();
    for k in (0..=ROWS).rev() {
        let t = k as f32 / ROWS as f32;
        let y = height + 0.02 - depth * (1.0 - t).powf(0.7);
        profile.push(Vec2::new(0.47 * t.powf(1.4), y));
    }
    let rows = profile.len();
    let phase = build.variety.wobble(n, 2) * 3.0;
    let mesh = lathe(&profile, false, |row, i| {
        // How near the rim: the waves grow towards it.
        let near = match row < rim {
            true => row as f32 / rim as f32,
            false => (rows - row) as f32 / (rows - rim) as f32,
        };
        let out = 1.0 + 0.08 * (5.0 * angle(i) + phase).sin() * near.powi(3);
        let colour = match row < rim {
            true => underside(look.under, c.under, row, i),
            false => c.body * (0.85 + 0.15 * near),
        };
        Round { out, colour }
    });
    vec![FungusPiece::new(mesh)]
}
