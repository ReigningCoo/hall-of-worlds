//! Lumps: truffles (warty when dark, smooth when pale, a glossy cave pearl),
//! sclerotia (bigger, lumpier, crusted) and chaga's cracked black mass; and a
//! crust: dry rot spreading as a flat pancake with a pale edge.

use bevy::prelude::*;
use shared::fungus::Group;

use super::ball::ball_profile;
use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};
use super::paint::speckle;

pub(super) fn lump(build: &Build, n: usize) -> Vec<FungusPiece> {
    let def = build.def;
    let c = build.colours;
    let dark = c.body.dot(Vec3::new(0.3, 0.6, 0.1)) < 0.5;
    let (lumpy, warty) = match def.group {
        Group::Sclerotium => (0.16, 0.0),
        Group::Bracket => (0.14, 0.03),
        _ if dark => (0.08, 0.07),
        _ => (0.08, 0.0),
    };
    let crusted = def.group != Group::Truffle;
    let phase = build.variety.wobble(n, 1) * 3.0;
    let profile = ball_profile(0.78);
    let rows = profile.len() as f32;
    let mesh = lathe(&profile, false, |row, i| {
        let (a, up) = (angle(i), row as f32 / rows * std::f32::consts::PI);
        let swell = (2.0 * a + phase).sin() * (3.0 * up).sin() + (3.0 * a - phase).cos() * 0.6;
        let wart = ((9.0 * a).sin() * (9.0 * up).sin()).abs();
        let spot = speckle(row, i, n as u32);
        let colour = match crusted && spot < 0.25 {
            true => c.body.lerp(c.under, 0.6),
            false => c.body * (0.9 + 0.12 * spot),
        };
        Round {
            out: 1.0 + lumpy * swell + warty * wart,
            colour,
        }
    });
    let piece = FungusPiece::new(mesh);
    let shiny = def.look.colours.len() == 1 && def.group == Group::Truffle;
    vec![if shiny { piece.shiny() } else { piece }]
}

pub(super) fn crust(build: &Build, n: usize) -> Vec<FungusPiece> {
    if build.def.group != Group::Crust {
        return lump(build, n);
    }
    let c = build.colours;
    let profile = [
        Vec2::new(0.0, 0.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.5, 0.03),
        Vec2::new(0.45, 0.07),
        Vec2::new(0.25, 0.09),
        Vec2::new(0.0, 0.1),
    ];
    let phase = build.variety.wobble(n, 1) * 3.0;
    let mesh = lathe(&profile, false, |row, i| {
        let a = angle(i);
        let out = 1.0 + 0.18 * (3.0 * a + phase).sin() + 0.08 * (7.0 * a).cos();
        let edge = row <= 3;
        let colour = match edge {
            true => c.under,
            false => c.body * (0.85 + 0.2 * speckle(row, i, 2)),
        };
        Round { out, colour }
    });
    vec![FungusPiece::new(mesh)]
}
