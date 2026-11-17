//! Herbs used for their berries, fruit, seeds or pods.

use bevy::prelude::*;

use super::Palette;
use super::kit::{Kit, along, wobble};

/// A cluster of shiny berries on thin stalks.
pub(in super::super) fn berries(palette: &Palette, variant: u8, kit: &mut Kit) {
    let berry = kit.material(palette.fruit, true);
    let stalk = kit.material(palette.leaf_dark, false);
    let top = Vec3::new(0.0, 0.05, -0.75);
    let count = 8 + usize::from(variant);
    for n in 0..count {
        let a = n as f32 * 2.4 + wobble(n, variant, 1) * 0.3;
        let ring = 0.12 + (n as f32 / count as f32) * 0.5;
        let y = (n % 3) as f32 * 0.06;
        let at = Vec3::new(a.sin() * ring, y, 0.1 + a.cos() * ring * 0.8);
        kit.rod(&stalk, top, at, 0.012);
        let size = 0.15 + wobble(n, variant, 2) * 0.02;
        kit.blob(&berry, at, Vec3::splat(size), (0.0, 0.0));
    }
}

/// Two fruits with a leaf.
pub(in super::super) fn fruit(palette: &Palette, variant: u8, kit: &mut Kit) {
    let skin = kit.material(palette.fruit, true);
    let leaf = kit.material(palette.leaf, false);
    let (_, yaw) = along(0.7 + wobble(0, variant, 1) * 0.2);
    kit.blob(
        &leaf,
        Vec3::new(0.25, 0.0, -0.45),
        Vec3::new(0.4, 0.03, 0.15),
        (yaw, 0.0),
    );
    kit.blob(
        &skin,
        Vec3::new(-0.3, 0.12, 0.15),
        Vec3::new(0.38, 0.33, 0.36),
        (0.0, 0.0),
    );
    kit.blob(
        &skin,
        Vec3::new(0.38, 0.1, 0.3),
        Vec3::new(0.3, 0.27, 0.29),
        (0.5, 0.0),
    );
}

/// A small heap of seeds.
pub(in super::super) fn seeds(palette: &Palette, variant: u8, kit: &mut Kit) {
    let seed = kit.material(palette.fruit, false);
    let darker = kit.material(palette.earth_dark, false);
    for n in 0..26 {
        let a = n as f32 * 2.4;
        let ring = (n as f32).sqrt() * 0.13;
        let at = Vec3::new(a.sin() * ring, (0.6 - ring).max(0.0) * 0.3, a.cos() * ring);
        let colour = if n % 4 == 0 { &darker } else { &seed };
        let yaw = wobble(n, variant, 1) * 3.0;
        kit.blob(colour, at, Vec3::new(0.09, 0.04, 0.055), (yaw, 0.0));
    }
}

/// A few long pods, the seeds inside showing as bumps.
pub(in super::super) fn pods(palette: &Palette, variant: u8, kit: &mut Kit) {
    let pod = kit.material(palette.fruit, false);
    for n in 0..3 {
        let a = (n as f32 - 1.0) * 0.5 + wobble(n, variant, 1) * 0.1;
        let (out, yaw) = along(a);
        let middle = Vec3::new((n as f32 - 1.0) * 0.3, n as f32 * 0.03, 0.0);
        kit.blob(&pod, middle, Vec3::new(0.75, 0.08, 0.11), (yaw, 0.0));
        for k in 0..5 {
            let at = middle + out * ((k as f32 - 2.0) * 0.22) + Vec3::Y * 0.05;
            kit.blob(&pod, at, Vec3::new(0.08, 0.06, 0.08), (yaw, 0.0));
        }
    }
}
