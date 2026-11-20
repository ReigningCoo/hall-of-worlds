//! A fungus's colours from its look (its body, underneath, its stem), a
//! little lighter or darker by variant, and the small helpers that put colours
//! on meshes.

use bevy::prelude::*;
use shared::fungus::{FungusLook, Group};
use shared::herb::Rgb;

/// Cream: the flesh inside most fungi.
const FLESH: Vec3 = Vec3::new(0.95, 0.92, 0.85);

#[derive(Clone, Copy)]
pub(super) struct Colours {
    /// A cap, a bracket's top, a ball, a lichen.
    pub(super) body: Vec3,
    /// Gills, pores, the inside of a cup.
    pub(super) under: Vec3,
    pub(super) stem: Vec3,
    /// What a cut shows.
    pub(super) inside: Vec3,
}

impl Colours {
    pub(super) fn of(look: &FungusLook, group: Group, shade: f32) -> Self {
        let colour = |k: usize| look.colours.get(k).copied().map(rgb);
        let body = colour(0).unwrap_or(Vec3::splat(0.6)) * (1.0 + shade * 0.06);
        let under = colour(1).unwrap_or(body * 0.85);
        let stem = colour(2).unwrap_or_else(|| body.lerp(FLESH, 0.5));
        let inside = match group {
            Group::Truffle | Group::Sclerotium => under,
            _ => body.lerp(FLESH, 0.7),
        };
        Self {
            body,
            under,
            stem,
            inside,
        }
    }
}

/// A table colour as sRGB parts, 0 to 1.
pub(super) fn rgb(Rgb(red, green, blue): Rgb) -> Vec3 {
    Vec3::new(red.into(), green.into(), blue.into()) / 255.0
}

/// sRGB parts as a linear vertex colour.
pub(super) fn linear(colour: Vec3) -> [f32; 4] {
    let c = colour.clamp(Vec3::ZERO, Vec3::ONE);
    Color::srgb(c.x, c.y, c.z).to_linear().to_f32_array()
}

/// A mesh with every point this colour, so it can share the white material.
pub(super) fn coloured(mesh: impl Into<Mesh>, colour: Vec3) -> Mesh {
    let mut mesh = mesh.into();
    let count = mesh.count_vertices();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![linear(colour); count]);
    mesh
}

/// A fixed speckle, 0 to 1, for point `i` of row `row` with salt `k`.
pub(super) fn speckle(row: usize, i: usize, k: u32) -> f32 {
    let mut x = (row as u32).wrapping_mul(0x9E37_79B9) ^ (i as u32).wrapping_mul(0x85EB_CA6B);
    x ^= k.wrapping_mul(0xC2B2_AE35);
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x % 1001) as f32 / 1000.0
}
