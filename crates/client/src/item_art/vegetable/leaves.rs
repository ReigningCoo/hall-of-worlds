//! Bunches of leaves: a handful fanned out from their stalks, which meet at
//! -x (kale, spinach, sorrel, dandelion, chicory); and sprigs with leaves
//! along a stem (lamb's quarters, purslane).

use bevy::prelude::*;

use super::Build;
use super::colours::colour;
use super::group::Leaves;
use super::leaf::{Blade, Leaf, leaf};

/// Where the stalks meet.
const BASE: Vec3 = Vec3::new(-0.55, 0.0, 0.0);

pub(super) fn leaves(build: &Build, leaves: Leaves, kit: &mut super::Kit) {
    match leaves {
        Leaves::Diamond | Leaves::Sprig => sprigs(build, leaves, kit),
        _ => fan(build, leaves, kit),
    }
}

/// Leaves fanned out, each on its stalk.
fn fan(build: &Build, leaves: Leaves, kit: &mut super::Kit) {
    let (blade, width, stalk, ruffle) = match leaves {
        Leaves::Curly => (Blade::Oval, 0.42, 0.18, 0.9),
        Leaves::Kidney => (Blade::Kidney, 0.45, 0.45, 0.0),
        Leaves::Toothed => (Blade::Toothed, 0.34, 0.08, 0.0),
        Leaves::Long => (Blade::Long, 0.3, 0.08, 0.0),
        _ => (Blade::Oval, 0.55, 0.25, 0.15),
    };
    let v = build.variety;
    let c = build.colours;
    let rib = c.accent_or(c.skin.lerp(Vec3::ONE, 0.45));
    let stem = kit.material(colour(rib), false);
    let count = build.count.max(1);
    for n in 0..count {
        let spread = if count == 1 {
            0.0
        } else {
            n as f32 / (count - 1) as f32 - 0.5
        };
        let yaw = spread * 1.1 + v.wobble(n, 1) * 0.08;
        let length = (1.0 - stalk) * (0.85 + v.share(n, 2) * 0.15);
        let out = Quat::from_rotation_y(yaw) * Vec3::X;
        let lift = Vec3::Y * (n as f32 * 0.025);
        let start = BASE + lift + out * stalk;
        kit.rod(&stem, BASE + lift, start, 0.014);
        let shade = c.skin * (1.0 + v.wobble(n, 3) * 0.08);
        let one = Leaf {
            ruffle,
            arch: 0.03 + v.share(n, 4) * 0.05,
            ..Leaf::new(blade, start, yaw, length, width).coloured(shade, rib)
        };
        leaf(kit, &one);
    }
}

/// Stems with leaves along them, smaller towards the tip.
fn sprigs(build: &Build, leaves: Leaves, kit: &mut super::Kit) {
    let v = build.variety;
    let c = build.colours;
    let stem_colour = c.accent_or(c.leaf);
    let stem = kit.material(colour(stem_colour), false);
    let fleshy = kit.material(colour(c.skin), true);
    for n in 0..3 {
        let yaw = (n as f32 - 1.0) * 0.35 + v.wobble(n, 5) * 0.06;
        let out = Quat::from_rotation_y(yaw) * Vec3::X;
        let lift = Vec3::Y * (n as f32 * 0.04);
        let tip = BASE + lift + out * 1.1;
        kit.rod(&stem, BASE + lift, tip, 0.02);
        for k in 0..7 {
            let t = 0.2 + k as f32 * 0.12;
            let at = BASE + lift + out * (t * 1.1) + Vec3::Y * 0.01;
            let side = if k % 2 == 0 { 1.0 } else { -1.0 };
            let size = 0.3 * (1.15 - t * 0.6);
            let a = yaw + side * 0.9;
            match leaves {
                Leaves::Sprig => {
                    let off = Quat::from_rotation_y(a) * Vec3::X * size * 0.4;
                    let half = Vec3::new(size * 0.4, size * 0.14, size * 0.24);
                    kit.blob(&fleshy, at + off, half, (a, 0.0));
                }
                _ => {
                    let rib = c.skin.lerp(stem_colour, 0.4);
                    let one = Leaf::new(Blade::Diamond, at, a, size, size * 0.75);
                    leaf(kit, &one.coloured(c.skin, rib));
                }
            }
        }
    }
}
