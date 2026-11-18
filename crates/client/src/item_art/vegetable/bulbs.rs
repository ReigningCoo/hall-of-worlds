//! Bulbs (onion, camas, desert onion): a papery globe streaked along its
//! length, a dry neck at +x and a tuft of roots on its plate at -x; small
//! ones by the handful, heaped.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::heap::{Place, heaped};
use super::tube::{Round, rings, tube};

pub(super) fn bulbs(build: &Build, kit: &mut super::Kit) {
    let places = match build.count {
        1 => vec![Place::ONE],
        n => heaped(n, 0.55, build.variety),
    };
    let egg = build.width_share().clamp(0.5, 1.0);
    for (n, place) in places.iter().enumerate() {
        bulb(build, place, n, egg, kit);
    }
}

/// One bulb about 1 long, `egg` as wide.
fn bulb(build: &Build, place: &Place, n: usize, egg: f32, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let r = egg / 2.0;
    let at = |t: f32| place.p(Vec3::new(t - 0.55, r * 0.9, 0.0));
    let half = |t: f32| {
        // A globe that draws in to a neck past 0.8.
        let body = (PI * (t / 0.85).min(1.0)).sin().max(0.0).powf(0.6);
        let neck = 0.12 * (1.0 - t).max(0.0) / 0.15;
        Vec2::new(r, r * 0.92) * body.max(neck.min(0.12)) * place.size
    };
    let colour_at = move |t: f32| skin.lerp(skin * 0.75, ((t - 0.8) / 0.2).clamp(0.0, 1.0));
    let round = Round {
        ribs: 9,
        depth: 0.015,
        stripes: Some((skin * 0.8, 0.15)),
    };
    tube(kit, &rings(26, at, half, colour_at), round, true);
    let roots = kit.material(colour(build.colours.straw * 0.85), false);
    let plate = at(0.0);
    for k in 0..5 {
        let a = PI + (k as f32 - 2.0) * 0.35 + build.variety.wobble(n * 5 + k, 1) * 0.1;
        let out = place.p(Vec3::new(a.cos(), -0.02, a.sin()) * 0.16) - place.at;
        kit.rod(&roots, plate, plate + out, 0.008 * place.size);
    }
}
