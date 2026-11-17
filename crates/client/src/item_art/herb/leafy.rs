//! Herbs used for their leaves, fronds, stalks or gel.

use bevy::prelude::*;

use super::Palette;
use super::kit::{Kit, along, wobble};

/// A bunch of leaves fanned out from their stems, tied at the bottom.
pub(in super::super) fn leaves(palette: &Palette, variant: u8, kit: &mut Kit) {
    let light = kit.material(palette.leaf, false);
    let dark = kit.material(palette.leaf_dark, false);
    let tie = kit.material(palette.earth, false);
    let base = Vec3::new(0.0, 0.0, 0.75);
    let count = 4 + usize::from(variant % 2);
    for n in 0..count {
        let spread = n as f32 / (count - 1) as f32 - 0.5;
        let a = spread * 1.2 + wobble(n, variant, 1) * 0.08;
        let (out, yaw) = along(a);
        let length = 0.42 + wobble(n, variant, 2) * 0.06;
        let stem_end = base + out * 0.25;
        kit.rod(&dark, base, stem_end, 0.025);
        let colour = if n % 2 == 0 { &light } else { &dark };
        let middle = stem_end + out * length;
        kit.blob(
            colour,
            middle + Vec3::Y * n as f32 * 0.02,
            Vec3::new(length, 0.04, 0.17),
            (yaw, 0.0),
        );
    }
    kit.blob(
        &tie,
        base + Vec3::new(0.0, 0.05, -0.05),
        Vec3::new(0.09, 0.07, 0.05),
        (0.0, 0.0),
    );
}

/// A few long fronds: a midrib with small leaflets either side.
pub(in super::super) fn fronds(palette: &Palette, variant: u8, kit: &mut Kit) {
    let leaf = kit.material(palette.leaf, false);
    let rib = kit.material(palette.leaf_dark, false);
    for n in 0..3 {
        let a = (n as f32 - 1.0) * 0.35 + wobble(n, variant, 1) * 0.05;
        let (out, _) = along(a);
        let from = Vec3::new(0.0, n as f32 * 0.03, 0.8);
        let to = from + out * 1.5;
        kit.rod(&rib, from, to, 0.018);
        for step in 1..8 {
            let at = from + out * (step as f32 * 0.18);
            let size = 0.13 - step as f32 * 0.01;
            for side in [-1.0, 1.0] {
                let (_, leaflet) = along(a + side * 1.1);
                let off = Vec3::new(out.z, 0.0, -out.x) * side * size;
                kit.blob(&leaf, at + off, Vec3::new(size, 0.02, 0.04), (leaflet, 0.0));
            }
        }
    }
}

/// A bundle of stalks tied in the middle.
pub(in super::super) fn stalks(palette: &Palette, variant: u8, kit: &mut Kit) {
    let light = kit.material(palette.leaf, false);
    let dark = kit.material(palette.leaf_dark, false);
    let tie = kit.material(palette.earth, false);
    for n in 0..6 {
        let x = (n as f32 - 2.5) * 0.07;
        let lean = wobble(n, variant, 1) * 0.08;
        let colour = if n % 2 == 0 { &light } else { &dark };
        let y = (n % 2) as f32 * 0.04;
        kit.rod(
            colour,
            Vec3::new(x - lean, y, 0.8),
            Vec3::new(x + lean, y, -0.8),
            0.035,
        );
    }
    kit.blob(
        &tie,
        Vec3::new(0.0, 0.03, 0.0),
        Vec3::new(0.25, 0.09, 0.05),
        (0.0, 0.0),
    );
}

/// A thick fleshy leaf cut open, its clear gel showing.
pub(in super::super) fn gel(palette: &Palette, kit: &mut Kit) {
    let skin = kit.material(palette.leaf_dark, false);
    let gel = kit.material(Color::srgba(0.82, 0.92, 0.78, 0.9), true);
    let (_, yaw) = along(0.4);
    kit.blob(
        &skin,
        Vec3::new(-0.05, 0.0, 0.05),
        Vec3::new(0.85, 0.12, 0.26),
        (yaw, 0.0),
    );
    kit.blob(
        &gel,
        Vec3::new(-0.05, 0.1, 0.05),
        Vec3::new(0.7, 0.06, 0.19),
        (yaw, 0.0),
    );
}
