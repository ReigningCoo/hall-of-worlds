//! A sheaf of grain: stalks of straw tied together, fanning out to their
//! heads at +x (`ears.rs`); maize is a cob in its husk instead.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::ears::{Frame, head};
use super::group::Ear;
use super::leaf::{Blade, Leaf, leaf};
use super::tube::{Round, cap, rings, tube};

/// The stalks in a sheaf.
const STALKS: usize = 6;

pub(super) fn sheaf(build: &Build, ear: Ear, kit: &mut super::Kit) {
    if ear == Ear::Cob {
        return cob(build, kit);
    }
    let c = build.colours;
    let straw = match ear {
        Ear::Buckwheat => c.accent_or(c.leaf),
        _ => c.straw,
    };
    let stalk = kit.material(colour(straw), false);
    let base = Vec3::new(-0.75, 0.04, 0.0);
    for n in 0..STALKS {
        let spread = n as f32 / (STALKS - 1) as f32 - 0.5;
        let yaw = spread * 0.45 + build.variety.wobble(n, 1) * 0.04;
        let out = Quat::from_rotation_y(yaw) * Vec3::X;
        let from = base + Vec3::new(0.0, 0.0, spread * 0.1);
        let lift = Vec3::Y * (n % 2) as f32 * 0.04;
        let top = from + out * (0.75 + build.variety.wobble(n, 2) * 0.05) + lift;
        kit.rod(&stalk, from, top, 0.014);
        let frame = Frame { at: top, out, n };
        head(build, ear, &frame, kit);
    }
    let twine = kit.material(colour(c.straw * 0.7), false);
    kit.blob(
        &twine,
        Vec3::new(-0.42, 0.05, 0.0),
        Vec3::new(0.035, 0.05, 0.09),
        (0.0, 0.0),
    );
}

/// A cob of kernels in rows, its husk peeled back, silk at its tip.
fn cob(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let r = 0.17;
    let core = c.skin * 0.8;
    let at = |t: f32| Vec3::new(t - 0.45, r, 0.0);
    let half =
        |t: f32| Vec2::splat(r * 0.85 * (1.0 - 0.3 * t) * cap(t, 0.05).min(cap(1.0 - t, 0.1)));
    tube(kit, &rings(16, at, half, |_| core), Round::default(), false);
    let kernel = kit.material(colour(c.skin), true);
    let pale = kit.material(colour(c.skin.lerp(Vec3::ONE, 0.2)), true);
    let sphere = Sphere::new(1.0).mesh().uv(8, 6);
    for k in 0..15 {
        let t = 0.06 + k as f32 * 0.06;
        let radius = half(t).x;
        for row in 0..12 {
            let a = (row as f32 + 0.5 * (k % 2) as f32) / 12.0 * PI * 2.0;
            let normal = Vec3::new(0.0, a.cos(), a.sin());
            let spot = at(t) + normal * radius;
            let rotation = Quat::from_rotation_arc(Vec3::Y, normal);
            let size = radius * 0.24;
            let transform = Transform::from_translation(spot)
                .with_rotation(rotation)
                .with_scale(Vec3::new(size * 1.1, size * 0.6, size));
            let material = if (k + row) % 5 == 0 { &pale } else { &kernel };
            kit.mesh(sphere.clone(), material, transform);
        }
    }
    let husk = c.accent_or(c.straw);
    for k in 0..3 {
        let yaw = PI + (k as f32 - 1.0) * 0.5 + build.variety.wobble(k, 3) * 0.1;
        let one = Leaf {
            cup: 0.6,
            arch: 0.05,
            ..Leaf::new(Blade::Long, Vec3::new(-0.42, r, 0.0), yaw, 0.55, 0.2).coloured(husk, husk)
        };
        leaf(kit, &one);
    }
    let silk = kit.material(colour(Vec3::new(0.62, 0.45, 0.25)), false);
    let tip = at(0.98);
    for k in 0..5 {
        let a = (k as f32 - 2.0) * 0.25;
        let to = tip + Vec3::new(a.cos() * 0.12, 0.02, a.sin() * 0.12);
        kit.rod(&silk, tip, to, 0.005);
    }
}
