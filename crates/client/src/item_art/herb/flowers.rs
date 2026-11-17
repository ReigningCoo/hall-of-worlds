//! Herbs used for their flowering tops, flowers, buds or stigmas.

use bevy::prelude::*;

use super::Palette;
use super::kit::{Kit, along, wobble};

/// A few leafy stems, each crowned with a cluster of small flowers.
pub(in super::super) fn tops(palette: &Palette, variant: u8, kit: &mut Kit) {
    let stem = kit.material(palette.leaf_dark, false);
    let leaf = kit.material(palette.leaf, false);
    let flower = kit.material(palette.flower, false);
    let base = Vec3::new(0.0, 0.0, 0.8);
    for n in 0..3 {
        let a = (n as f32 - 1.0) * 0.45 + wobble(n, variant, 1) * 0.06;
        let (out, yaw) = along(a);
        let across = Vec3::new(out.z, 0.0, -out.x);
        let top = base + out * 1.0;
        kit.rod(&stem, base, top, 0.025);
        for side in [-1.0, 1.0] {
            let at = base + out * 0.45 + across * side * 0.12;
            kit.blob(
                &leaf,
                at,
                Vec3::new(0.14, 0.03, 0.06),
                (yaw + side * 0.8, 0.0),
            );
        }
        for k in 0..7 {
            let up = out * (0.1 + k as f32 * 0.05);
            let side = across * wobble(n * 7 + k, variant, 2) * 0.09;
            let at = top - out * 0.35 + up + side + Vec3::Y * 0.05;
            kit.blob(&flower, at, Vec3::splat(0.06), (0.0, 0.0));
        }
    }
}

/// A handful of open flower heads: a ring of petals round a darker middle.
pub(in super::super) fn flowers(palette: &Palette, variant: u8, kit: &mut Kit) {
    let petal = kit.material(palette.flower, false);
    let middle = kit.material(palette.earth, false);
    let heads = [
        (-0.42, -0.3),
        (0.38, -0.38),
        (0.0, 0.1),
        (-0.35, 0.5),
        (0.4, 0.45),
    ];
    let count = 4 + usize::from(variant % 2);
    for (n, &(x, z)) in heads.iter().enumerate().take(count) {
        let at = Vec3::new(x + wobble(n, variant, 1) * 0.05, n as f32 * 0.02, z);
        let size = 0.3 + wobble(n, variant, 2) * 0.03;
        for p in 0..7 {
            let a = p as f32 / 7.0 * std::f32::consts::TAU + wobble(n, variant, 3);
            let (out, yaw) = along(a);
            let petal_at = at + out * size * 0.55;
            kit.blob(
                &petal,
                petal_at,
                Vec3::new(size * 0.5, 0.03, size * 0.2),
                (yaw, 0.0),
            );
        }
        kit.blob(
            &middle,
            at + Vec3::Y * 0.04,
            Vec3::new(0.08, 0.05, 0.08),
            (0.0, 0.0),
        );
    }
}

/// A heap of closed buds, each on a short green base.
pub(in super::super) fn buds(palette: &Palette, variant: u8, kit: &mut Kit) {
    let bud = kit.material(palette.flower, false);
    let base = kit.material(palette.leaf_dark, false);
    for n in 0..9 {
        let ring = if n == 0 {
            0.0
        } else {
            0.4 + (n % 2) as f32 * 0.15
        };
        let a = n as f32 * 0.8 + wobble(n, variant, 1) * 0.3;
        let (out, yaw) = along(a);
        let at = out * ring + Vec3::Y * (n % 3) as f32 * 0.04;
        kit.blob(&bud, at, Vec3::new(0.16, 0.09, 0.09), (yaw, 0.0));
        kit.blob(
            &base,
            at - out * 0.14,
            Vec3::new(0.05, 0.06, 0.06),
            (yaw, 0.0),
        );
    }
}

/// A small pile of fine threads.
pub(in super::super) fn stigmas(palette: &Palette, variant: u8, kit: &mut Kit) {
    let thread = kit.material(palette.flower, false);
    for n in 0..16 {
        let a = n as f32 * 0.9 + wobble(n, variant, 1) * 0.4;
        let (out, _) = along(a);
        let near = out * (0.05 + wobble(n, variant, 2).abs() * 0.15);
        let far = out * (0.6 + wobble(n, variant, 3) * 0.15);
        let lift = Vec3::Y * (n % 4) as f32 * 0.02;
        kit.rod(&thread, near + lift, far + lift, 0.022);
    }
}
