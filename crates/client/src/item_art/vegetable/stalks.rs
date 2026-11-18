//! Stalks: a leek (white shank, fanned blue-green leaves, roots), a bunch of
//! celery (ribbed stalks with leafy tops), a stalk of rhubarb (cut at the
//! top) and an agave heart (cut leaf bases all over). Built along +x.

use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use super::Build;
use super::colours::colour;
use super::group::Stalk;
use super::leaf::{Blade, Leaf, leaf};
use super::tube::{Round, cap, rings, tube};

pub(super) fn stalks(build: &Build, stalk: Stalk, kit: &mut super::Kit) {
    match stalk {
        Stalk::Leek => leek(build, kit),
        Stalk::Celery => celery(build, kit),
        Stalk::Rhubarb => rhubarb(build, kit),
        Stalk::Heart => heart(build, kit),
    }
}

fn leek(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let green = c.accent_or(c.leaf);
    let r = 0.07;
    let skin = c.skin;
    let at = |t: f32| Vec3::new(-0.6 + t * 0.6, r, 0.0);
    let half = |t: f32| Vec2::splat(r * cap(t, 0.05) * (1.0 - 0.15 * t));
    let shade = move |t: f32| skin.lerp(skin.lerp(green, 0.5), ((t - 0.6) / 0.4).clamp(0.0, 1.0));
    tube(kit, &rings(16, at, half, shade), Round::default(), false);
    for n in 0..4 {
        let yaw = (n as f32 - 1.5) * 0.16 + build.variety.wobble(n, 1) * 0.05;
        let start = Vec3::new(-0.05, r + n as f32 * 0.012, 0.0);
        let one = Leaf {
            cup: 0.5,
            arch: 0.02,
            ..Leaf::new(Blade::Long, start, yaw, 0.65, 0.16).coloured(green, green * 1.15)
        };
        leaf(kit, &one);
    }
    let roots = kit.material(colour(c.straw * 0.9), false);
    for k in 0..6 {
        let a = PI + (k as f32 - 2.5) * 0.22;
        let to = Vec3::new(-0.6 + a.cos() * 0.12, r * 0.6, a.sin() * 0.12);
        kit.rod(&roots, Vec3::new(-0.6, r, 0.0), to, 0.006);
    }
}

fn celery(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let v = build.variety;
    let tops = c.accent_or(c.leaf);
    let base = c.skin.lerp(Vec3::ONE, 0.35);
    let round = Round {
        ribs: 6,
        depth: 0.08,
        stripes: Some((c.skin.lerp(Vec3::ONE, 0.25), 0.2)),
    };
    for n in 0..6 {
        let spread = (n as f32 - 2.5) * 0.07;
        let r = 0.065;
        let skin = c.skin;
        let lift = (n % 2) as f32 * 0.05;
        let at = move |t: f32| Vec3::new(-0.6 + t * 1.05, r + lift, spread * (0.6 + t));
        let half = |t: f32| Vec2::new(r, r * 0.7) * cap(t, 0.03).min(cap(1.0 - t, 0.03));
        let shade = move |t: f32| base.lerp(skin, (t * 2.0).min(1.0));
        tube(kit, &rings(14, at, half, shade), round, true);
        for k in 0..3 {
            let yaw = spread * 3.0 + (k as f32 - 1.0) * 0.5 + v.wobble(n * 3 + k, 2) * 0.1;
            let start = at(0.95) + Vec3::Y * 0.02;
            let one = Leaf::new(Blade::Toothed, start, yaw, 0.25, 0.16).coloured(tops, tops);
            leaf(kit, &one);
        }
    }
}

fn rhubarb(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let r = 0.07;
    let skin = c.skin;
    let top = skin.lerp(Vec3::new(0.55, 0.65, 0.3), 0.35);
    let at = |t: f32| Vec3::new(t - 0.5, r, 0.0);
    let half = |t: f32| {
        let heel = 1.0 + 0.5 * (1.0 - t / 0.12).max(0.0);
        let cut = if t > 0.995 { 0.0 } else { 1.0 };
        Vec2::new(r, r * 0.75) * heel * cap(t, 0.02) * cut
    };
    let shade = move |t: f32| skin.lerp(top, t);
    let round = Round {
        ribs: 9,
        depth: 0.04,
        stripes: Some((skin * 0.85, 0.12)),
    };
    tube(kit, &rings(24, at, half, shade), round, true);
    let flesh = kit.material(colour(c.inside), false);
    let face = Vec3::new(0.49, r, 0.0);
    kit.blob(
        &flesh,
        face,
        Vec3::new(0.005, r * 0.7, r * 0.95),
        (0.0, 0.0),
    );
}

/// An egg covered in the flat stubs of cut leaves.
fn heart(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let body = kit.material(colour(c.skin), false);
    kit.blob(
        &body,
        Vec3::new(0.0, 0.4, 0.0),
        Vec3::new(0.55, 0.4, 0.45),
        (0.0, 0.0),
    );
    let stub = kit.material(colour(c.accent_or(c.skin * 0.85) * 0.85), false);
    let sphere = Sphere::new(1.0).mesh().uv(8, 6);
    for row in 0..6 {
        let t = row as f32 / 5.0;
        let x = -0.4 + t * 0.8;
        let r = 0.42 * (PI * (0.1 + t * 0.8)).sin();
        for k in 0..10 {
            let a = (k as f32 + 0.5 * (row % 2) as f32) / 10.0 * TAU;
            let out = Vec3::new(0.0, a.cos(), a.sin());
            let at = Vec3::new(x, 0.4, 0.0) + out * r;
            let normal = (out + Vec3::X * 0.3).normalize();
            let forward = Vec3::X.reject_from(normal).normalize_or(Vec3::X);
            let rotation =
                Quat::from_mat3(&Mat3::from_cols(forward, normal, forward.cross(normal)));
            let transform = Transform::from_translation(at)
                .with_rotation(rotation)
                .with_scale(Vec3::new(0.2, 0.07, 0.16));
            kit.mesh(sphere.clone(), &stub, transform);
        }
    }
}
