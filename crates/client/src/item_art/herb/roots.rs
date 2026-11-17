//! Herbs used for what grows underground (a root, a rhizome, a tuber, a bulb),
//! or for their bark or resin.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Palette;
use super::kit::{Kit, along, wobble};

/// A taproot, its cut top pale, with fine rootlets.
pub(in super::super) fn root(palette: &Palette, variant: u8, kit: &mut Kit) {
    let skin = kit.material(palette.earth, false);
    let fine = kit.material(palette.earth_dark, false);
    let cut = kit.material(palette.pale, false);
    let top = Vec3::new(-0.1, 0.0, -0.7);
    let tip = Vec3::new(0.15 + wobble(0, variant, 1) * 0.1, 0.0, 0.8);
    kit.cone(&skin, top, tip, 0.2);
    kit.blob(
        &cut,
        top + Vec3::Y * 0.05,
        Vec3::new(0.17, 0.03, 0.17),
        (0.0, 0.0),
    );
    for n in 0..5 {
        let at = top.lerp(tip, 0.25 + n as f32 * 0.13);
        let side = if n % 2 == 0 { 1.0 } else { -1.0 };
        let out = Vec3::new(side * (0.25 + wobble(n, variant, 2) * 0.08), 0.0, 0.15);
        kit.rod(&fine, at, at + out, 0.012);
    }
}

/// A knobbly creeping rhizome, in a few joints.
pub(in super::super) fn rhizome(palette: &Palette, variant: u8, kit: &mut Kit) {
    let skin = kit.material(palette.earth, false);
    let fine = kit.material(palette.earth_dark, false);
    let mut at = Vec3::new(-0.65, 0.0, 0.1);
    for n in 0..5 {
        let next = at + Vec3::new(0.3, 0.0, wobble(n, variant, 1) * 0.25);
        let (_, yaw) = along(FRAC_PI_2 + wobble(n, variant, 2) * 0.4);
        kit.blob(&skin, at, Vec3::new(0.2, 0.12, 0.13), (yaw, 0.0));
        let rootlet = at + Vec3::new(wobble(n, variant, 3) * 0.1, -0.02, 0.3);
        kit.rod(&fine, at, rootlet, 0.012);
        if n == 1 || n == 3 {
            // A bud on top, where a shoot would grow.
            let bud = at + Vec3::new(0.0, 0.1, -0.08);
            kit.blob(&fine, bud, Vec3::splat(0.06), (0.0, 0.0));
        }
        at = next;
    }
}

/// Two or three lumpy tubers.
pub(in super::super) fn tuber(palette: &Palette, variant: u8, kit: &mut Kit) {
    let skin = kit.material(palette.earth, false);
    let eye = kit.material(palette.earth_dark, false);
    let lumps = [(-0.3, 0.1, 0.35), (0.35, -0.25, 0.3), (0.2, 0.45, 0.24)];
    let count = 2 + usize::from(variant % 2);
    for (n, &(x, z, size)) in lumps.iter().enumerate().take(count) {
        let at = Vec3::new(x, size * 0.5, z);
        let shape = Vec3::new(size, size * 0.8, size * 0.75);
        kit.blob(&skin, at, shape, (wobble(n, variant, 1), 0.0));
        let eye_at = at + Vec3::new(0.0, size * 0.75, -size * 0.2);
        kit.blob(&eye, eye_at, Vec3::splat(0.04), (0.0, 0.0));
    }
}

/// A bulb: pale and round, coming to a point, with a tuft of roots.
pub(in super::super) fn bulb(palette: &Palette, kit: &mut Kit) {
    let skin = kit.material(palette.pale, false);
    let papery = kit.material(palette.earth, false);
    kit.blob(
        &skin,
        Vec3::new(0.0, 0.2, 0.1),
        Vec3::new(0.48, 0.42, 0.45),
        (0.0, 0.0),
    );
    kit.cone(
        &papery,
        Vec3::new(0.0, 0.2, -0.3),
        Vec3::new(0.0, 0.2, -0.75),
        0.15,
    );
    for n in 0..5 {
        let x = (n as f32 - 2.0) * 0.08;
        let from = Vec3::new(x * 0.5, 0.1, 0.5);
        kit.rod(&papery, from, Vec3::new(x * 2.0, 0.0, 0.82), 0.012);
    }
}

/// Curled strips of bark, the paler inner side showing.
pub(in super::super) fn bark(palette: &Palette, variant: u8, kit: &mut Kit) {
    let outer = kit.material(palette.earth_dark, false);
    let inner = kit.material(palette.earth, false);
    for n in 0..3 {
        let a = FRAC_PI_2 + (n as f32 - 1.0) * 0.25 + wobble(n, variant, 1) * 0.1;
        let (_, yaw) = along(a);
        let at = Vec3::new(0.0, n as f32 * 0.06, (n as f32 - 1.0) * 0.42);
        kit.blob(&outer, at, Vec3::new(0.75, 0.07, 0.17), (yaw, 0.0));
        let inside = at + Vec3::Y * 0.03;
        kit.blob(&inner, inside, Vec3::new(0.66, 0.05, 0.12), (yaw, 0.0));
    }
}

/// Lumps of resin, shiny and amber.
pub(in super::super) fn resin(palette: &Palette, variant: u8, kit: &mut Kit) {
    let resin = kit.material(palette.fruit, true);
    let lumps = [
        (-0.3, -0.25),
        (0.3, -0.2),
        (0.0, 0.25),
        (-0.4, 0.4),
        (0.42, 0.42),
    ];
    for (n, &(x, z)) in lumps.iter().enumerate() {
        let size = 0.17 + wobble(n, variant, 1) * 0.05;
        let at = Vec3::new(x, size * 0.5, z);
        let shape = Vec3::new(size, size * 0.7, size * 0.85);
        kit.blob(&resin, at, shape, (wobble(n, variant, 2) * 3.0, 0.0));
    }
}
