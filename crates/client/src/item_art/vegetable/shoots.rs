//! Shoots: fiddleheads (coils lying flat), asparagus spears tied in a
//! bundle, blanched sea kale with crinkled tips, a bamboo shoot in its
//! sheaths, and water spinach's stems with arrow leaves.

use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use super::Build;
use super::colours::colour;
use super::group::Shoot;
use super::heap::{Place, heaped, side_by_side};
use super::leaf::{Blade, Leaf, leaf};
use super::tube::{Ring, Round, cap, rings, tube};

pub(super) fn shoots(build: &Build, shoot: Shoot, kit: &mut super::Kit) {
    match shoot {
        Shoot::Fiddlehead => {
            for (n, place) in heaped(build.count, 0.45, build.variety).iter().enumerate() {
                fiddlehead(build, place, n, kit);
            }
        }
        Shoot::Spear => spears(build, kit),
        Shoot::Blanched => blanched(build, kit),
        Shoot::Bamboo => bamboo(build, kit),
        Shoot::Vine => vines(build, kit),
    }
}

/// A coil seen from above, its stalk running off below.
fn fiddlehead(build: &Build, place: &Place, n: usize, kit: &mut super::Kit) {
    let c = build.colours;
    let mut path = Vec::new();
    for k in 0..5 {
        let s = k as f32 / 5.0;
        path.push((Vec3::new(0.32, 0.0, -0.55 + s * 0.55), 0.07));
    }
    for k in 0..=40 {
        let s = k as f32 / 40.0;
        let a = s * 2.1 * TAU;
        let r = 0.32 * (1.0 - 0.85 * s);
        let thick = 0.07 * (1.0 - 0.45 * s) * cap(1.0 - s, 0.05);
        path.push((Vec3::new(r * a.cos(), 0.0, r * a.sin()), thick));
    }
    let rings: Vec<Ring> = path
        .iter()
        .map(|&(at, r)| Ring {
            at: place.p(at + Vec3::Y * 0.08),
            half: Vec2::splat(r * place.size),
            colour: c.skin,
        })
        .collect();
    tube(kit, &rings, Round::default(), true);
    let scale = kit.material(colour(c.accent_or(c.earth)), false);
    for k in 0..3 {
        let a = build.variety.wobble(n * 3 + k, 1) * PI;
        let at = place.p(Vec3::new(a.cos() * 0.2, 0.16, a.sin() * 0.2));
        let half = Vec3::new(0.06, 0.012, 0.035) * place.size;
        kit.blob(&scale, at, half, (a, 0.0));
    }
}

/// Spears side by side, tied round the middle.
fn spears(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let tip = c.accent_or(c.skin * 0.8);
    let bract = kit.material(colour(tip), false);
    for (n, place) in side_by_side(build.count, 0.1, build.variety)
        .iter()
        .enumerate()
    {
        let r = 0.045;
        let at = |t: f32| place.p(Vec3::new(t - 0.5, r, 0.0));
        let half = |t: f32| Vec2::splat(r * cap(1.0 - t, 0.1) * place.size);
        let skin = c.skin;
        let shade = move |t: f32| skin.lerp(Vec3::ONE, 0.3 * (1.0 - t / 0.2).max(0.0));
        tube(kit, &rings(16, at, half, shade), Round::default(), true);
        for k in 0..4 {
            let t = 0.68 + k as f32 * 0.07;
            let side = if (k + n) % 2 == 0 { 1.0 } else { -1.0 };
            let at = at(t) + place.p(Vec3::new(0.0, r * 0.5, side * r * 0.8)) - place.at;
            kit.blob(&bract, at, Vec3::new(0.03, 0.01, 0.015), (place.yaw, 0.0));
        }
    }
    let band = kit.material(colour(Vec3::new(0.7, 0.25, 0.2)), false);
    let across = 0.1 * build.count as f32 * 0.55;
    kit.blob(
        &band,
        Vec3::new(-0.1, 0.06, 0.0),
        Vec3::new(0.035, 0.07, across),
        (0.0, 0.0),
    );
}

/// Slender pale stalks with small crinkled purple tips.
fn blanched(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let tips = c.accent_or(c.leaf);
    for n in 0..4 {
        let yaw = (n as f32 - 1.5) * 0.12 + build.variety.wobble(n, 2) * 0.04;
        let place = Place {
            at: Vec3::Y * n as f32 * 0.02,
            yaw,
            size: 1.0 - build.variety.share(n, 3) * 0.1,
        };
        let r = 0.04;
        let skin = c.skin;
        let at = |t: f32| place.p(Vec3::new(t * 0.8 - 0.55, r, 0.0));
        let half = |t: f32| Vec2::splat(r * cap(t, 0.03) * place.size);
        tube(kit, &rings(10, at, half, |_| skin), Round::default(), true);
        let one = Leaf {
            ruffle: 1.0,
            cup: 0.5,
            ..Leaf::new(Blade::Oval, at(1.0), yaw, 0.3, 0.22).coloured(tips, skin)
        };
        leaf(kit, &one);
    }
}

/// A curved cone in overlapping brown sheaths, cut across its base.
fn bamboo(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let tip = c.accent_or(skin);
    let at = |t: f32| Vec3::new(t - 0.5, 0.2, (PI * t * 0.5).sin() * 0.1);
    let half = |t: f32| {
        let sheath = 1.0 + 0.06 * (1.0 - (t * 7.0).fract());
        Vec2::splat(0.2 * (1.0 - t).powf(0.8) * sheath * cap(t, 0.02))
    };
    let shade = move |t: f32| {
        let edge = (t * 7.0).fract() < 0.12;
        let base = skin.lerp(tip, (t - 0.6).max(0.0) * 1.5);
        if edge { base * 1.25 } else { base }
    };
    tube(kit, &rings(42, at, half, shade), Round::default(), false);
}

/// Hollow stems with arrowhead leaves along them.
fn vines(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let stem = c.accent_or(c.leaf);
    let stalk = kit.material(colour(stem), true);
    for n in 0..build.count {
        let yaw = (n as f32 - 1.0) * 0.3 + build.variety.wobble(n, 4) * 0.06;
        let out = Quat::from_rotation_y(yaw) * Vec3::X;
        let from = Vec3::new(-0.6, 0.03 + n as f32 * 0.03, 0.0);
        kit.rod(&stalk, from, from + out * 1.15, 0.025);
        for k in 0..4 {
            let at = from + out * (0.3 + k as f32 * 0.25) + Vec3::Y * 0.01;
            let side = if (k + n) % 2 == 0 { 0.8 } else { -0.8 };
            let one = Leaf::new(Blade::Arrow, at, yaw + side, 0.32, 0.2);
            leaf(kit, &one.coloured(c.skin, stem));
        }
    }
}
