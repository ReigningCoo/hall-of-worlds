//! Pods: okra (five ridges, pointed, a cap at the stalk), mesquite (flat,
//! narrow and bumpy over its seeds, streaked), winged beans (four frilly
//! wings) and peanuts in their netted shells. Built along +x.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::group::Pod;
use super::heap::{Place, heaped, side_by_side};
use super::tube::{Round, cap, rings, tube};

pub(super) fn pods(build: &Build, pod: Pod, kit: &mut super::Kit) {
    let places = match pod {
        Pod::Shell => heaped(build.count, 0.32, build.variety),
        Pod::Ridged => side_by_side(build.count, 0.24, build.variety),
        _ => side_by_side(build.count, 0.2, build.variety),
    };
    for (n, place) in places.iter().enumerate() {
        match pod {
            Pod::Ridged => okra(build, place, kit),
            Pod::Flat => mesquite(build, place, n, kit),
            Pod::Winged => winged(build, place, kit),
            Pod::Shell => shell(build, place, n, kit),
        }
    }
}

fn okra(build: &Build, place: &Place, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let r = 0.1;
    let at = |t: f32| place.p(Vec3::new(t - 0.5, r, 0.0));
    let half = |t: f32| Vec2::splat(r * (1.0 - t).powf(0.7) * cap(t, 0.06) * place.size);
    let round = Round {
        ribs: 5,
        depth: 0.14,
        stripes: Some((skin * 0.85, 0.1)),
    };
    tube(kit, &rings(16, at, half, |_| skin), round, false);
    let cap_colour = kit.material(colour(c.accent_or(c.leaf)), false);
    let top = at(0.0);
    kit.blob(
        &cap_colour,
        top,
        Vec3::new(0.04, r * 0.9, r * 0.9) * place.size,
        (place.yaw, 0.0),
    );
    let stalk = top + place.p(Vec3::new(-0.12, 0.0, 0.0)) - place.at;
    kit.rod(&cap_colour, top, stalk, 0.02 * place.size);
}

fn mesquite(build: &Build, place: &Place, n: usize, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let streak = c.accent_or(skin * 0.8);
    let bend = 0.06 + build.variety.wobble(n, 1) * 0.05;
    let at = |t: f32| place.p(Vec3::new(t * 1.2 - 0.6, 0.04, (PI * t).sin() * bend));
    let half = |t: f32| {
        let bumps = 1.0 + 0.18 * (t * 9.0 * PI).sin().abs();
        Vec2::new(0.05 * bumps, 0.025 * bumps) * cap(t, 0.04).min(cap(1.0 - t, 0.08)) * place.size
    };
    let round = Round {
        ribs: 3,
        depth: 0.0,
        stripes: Some((streak, 0.12)),
    };
    tube(kit, &rings(36, at, half, |_| skin), round, false);
}

fn winged(build: &Build, place: &Place, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let at = |t: f32| place.p(Vec3::new(t * 1.1 - 0.55, 0.08, 0.0));
    let half = |t: f32| {
        let frill = 1.0 + 0.08 * (t * 30.0).sin();
        Vec2::splat(0.065 * frill * cap(t, 0.05).min(cap(1.0 - t, 0.1)) * place.size)
    };
    let round = Round {
        ribs: 4,
        depth: 0.45,
        stripes: Some((skin * 1.15, 0.2)),
    };
    tube(kit, &rings(30, at, half, |_| skin), round, true);
}

/// Two lumps pinched in the middle, netted.
fn shell(build: &Build, place: &Place, n: usize, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let bend = build.variety.wobble(n, 2) * 0.05;
    let at = |t: f32| place.p(Vec3::new(t * 0.7 - 0.35, 0.12, (PI * t).sin() * bend));
    let half = |t: f32| {
        let lumps = 1.0 - 0.3 * (-((t - 0.5) / 0.1).powi(2)).exp();
        Vec2::splat(0.13 * lumps * cap(t, 0.15).min(cap(1.0 - t, 0.15)) * place.size)
    };
    let shade = move |t: f32| skin * (1.0 - 0.1 * f32::from((t * 10.0).fract() < 0.2));
    let round = Round {
        ribs: 8,
        depth: 0.03,
        stripes: Some((skin * 0.85, 0.15)),
    };
    tube(kit, &rings(24, at, half, shade), round, false);
}
