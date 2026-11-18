//! Heads: a cabbage (a firm ball veined from the stalk, in a ring of outer
//! leaves), a lettuce (a loose rosette round a pale heart) and a globe
//! artichoke (overlapping scales on a stem stub), seen from above.

use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use super::Build;
use super::colours::colour;
use super::group::Head;
use super::leaf::{Blade, Leaf, leaf};
use super::tube::{Round, rings, tube};

pub(super) fn head(build: &Build, head: Head, kit: &mut super::Kit) {
    match head {
        Head::Cabbage => cabbage(build, kit),
        Head::Lettuce => lettuce(build, kit),
        Head::Artichoke => artichoke(build, kit),
    }
}

fn cabbage(build: &Build, kit: &mut super::Kit) {
    let v = build.variety;
    let c = build.colours;
    let rib = c.accent_or(c.skin.lerp(Vec3::ONE, 0.5));
    for n in 0..6 {
        let yaw = n as f32 / 6.0 * TAU + v.wobble(n, 1) * 0.2;
        let base = Quat::from_rotation_y(yaw) * Vec3::X * 0.12;
        let outer = c.skin * 0.85;
        let one = Leaf {
            cup: 0.5,
            arch: 0.12,
            ruffle: 0.1,
            ..Leaf::new(Blade::Kidney, base, yaw, 0.75, 0.85).coloured(outer, rib)
        };
        leaf(kit, &one);
    }
    // The ball, its veins running up from the stalk below.
    let skin = c.skin;
    let at = |t: f32| Vec3::new(0.0, 0.05 + t * 0.85, 0.0);
    let half = |t: f32| Vec2::splat((PI * t).sin().max(0.0).powf(0.8) * 0.5);
    let round = Round {
        ribs: 7,
        depth: 0.02,
        stripes: Some((rib, 0.06)),
    };
    tube(kit, &rings(24, at, half, |_| skin), round, true);
}

fn lettuce(build: &Build, kit: &mut super::Kit) {
    let v = build.variety;
    let c = build.colours;
    let heart = c.accent_or(c.skin.lerp(Vec3::ONE, 0.5));
    for (ring, (count, length, rise)) in [(8, 0.78, 0.0), (6, 0.55, 0.12), (4, 0.36, 0.24)]
        .into_iter()
        .enumerate()
    {
        for n in 0..count {
            let k = ring * 8 + n;
            let yaw = (n as f32 + ring as f32 * 0.5) / count as f32 * TAU + v.wobble(k, 2) * 0.15;
            let shade = c.skin.lerp(heart, ring as f32 * 0.25);
            let one = Leaf {
                cup: 0.6,
                arch: 0.1,
                ruffle: 0.7,
                ..Leaf::new(Blade::Oval, Vec3::Y * rise, yaw, length, length * 0.9)
                    .coloured(shade, heart)
            };
            leaf(kit, &one);
        }
    }
    let middle = kit.material(colour(heart), false);
    kit.blob(
        &middle,
        Vec3::Y * 0.3,
        Vec3::new(0.14, 0.1, 0.14),
        (0.0, 0.0),
    );
}

/// Rows of scales round an egg lying along x, its tip at +x, purple-tipped.
fn artichoke(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let stem = kit.material(colour(c.skin * 0.85), false);
    kit.rod(
        &stem,
        Vec3::new(-0.75, 0.42, 0.0),
        Vec3::new(-0.4, 0.42, 0.0),
        0.08,
    );
    let sphere = Sphere::new(1.0).mesh().uv(10, 6);
    let rows = 7;
    for row in 0..rows {
        let t = row as f32 / (rows - 1) as f32;
        // Along the egg from its base (-x) to its tip.
        let x = -0.42 + t * 0.78;
        let r = 0.42 * (PI * (0.12 + t * 0.8)).sin().max(0.05);
        let tip = c.skin.lerp(c.accent_or(c.skin), (t * 1.2).min(1.0) * 0.6);
        let scale = kit.material(colour(tip), false);
        let around = 9;
        for k in 0..around {
            let a = (k as f32 + 0.5 * (row % 2) as f32) / around as f32 * TAU;
            let normal = Vec3::new(0.25, a.cos(), a.sin()).normalize();
            let at = Vec3::new(x, 0.42, 0.0) + Vec3::new(0.0, a.cos(), a.sin()) * r;
            let forward = Vec3::X.reject_from(normal).normalize_or(Vec3::X);
            let side = forward.cross(normal);
            let rotation = Quat::from_mat3(&Mat3::from_cols(forward, normal, side));
            let size = 0.13 + 0.06 * (1.0 - t);
            let transform = Transform::from_translation(at + forward * size * 0.4)
                .with_rotation(rotation)
                .with_scale(Vec3::new(size, size * 0.22, size * 0.8));
            kit.mesh(sphere.clone(), &scale, transform);
        }
    }
}
