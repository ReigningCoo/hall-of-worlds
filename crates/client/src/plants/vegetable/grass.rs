//! Grasses. Grain grass (wheat, barley, millets) and the water grasses (rice,
//! wild rice) are tufts of narrow blades, sending up stalks with ears once
//! grown: green, then ripe and nodding in water grasses, the leaves turning
//! to straw. Tall grass (maize, sorghum) is one stout stalk with broad leaves
//! arching from it, topped by its head, or by a tassel over a cob (maize).
//! Tall grasses with narrow ears (oats, rye) grow as tufts.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;
use shared::vegetable::PlantShape;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, between};
use super::look::{STRAW, Show};
use super::{Crop, Look, around, out};

/// Ears narrower than this (half their width, in metres) grow on tufts.
const NARROW_EAR: f32 = 0.03;

/// A tuft of grass.
pub(super) fn tuft(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let blades = ((5.0 * look.grown).round() as usize).max(2);
    let wide = (look.radius * 0.3).max(0.35);
    for i in 0..blades {
        let angle = around(i, blades, rng);
        let foot = out(angle, 0.0) * look.radius * 0.15;
        let rise = between(rng, 2.0, 5.0);
        let long = look.height * between(rng, 0.55, 0.8);
        let shade = look.leaf * between(rng, 0.9, 1.1);
        pieces.leaf(foot, out(angle, rise), (long, wide, 0.15), shade);
    }
    if look.show == Show::Leaves {
        return;
    }
    let stalk = stalk_colour(look);
    let nods = matches!(look.shape, PlantShape::WaterGrass | PlantShape::PaddyGrass);
    let stalks = 3;
    for i in 0..stalks {
        let angle = around(i, stalks, rng);
        let tip = Vec3::Y * look.height * between(rng, 0.85, 1.0)
            + out(angle, 0.0) * look.radius * between(rng, 0.3, 0.6);
        pieces.stem(Vec3::ZERO, tip, (0.15, 0.12), stalk);
        let way = if nods && look.show == Show::Ripe {
            out(angle, -0.8)
        } else {
            out(angle, 4.0)
        };
        crop.head(pieces, tip, way);
    }
}

/// Tall grass.
pub(super) fn tall(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    if look.part_size.y < NARROW_EAR * METRE {
        tuft(pieces, look, rng, crop);
        return;
    }
    let top = Vec3::Y * look.height;
    let thick = (look.height * 0.025).clamp(0.25, 0.5);
    pieces.stem(Vec3::ZERO, top, (thick, thick * 0.5), stalk_colour(look));
    let leaves = ((6.0 * look.grown).round() as usize).max(2);
    let start = between(rng, 0.0, TAU);
    for i in 0..leaves {
        let at = top * (i as f32 + 0.5) / (leaves as f32 + 0.5);
        let angle = start + i as f32 * TAU / 2.0 + between(rng, -0.4, 0.4);
        let length = (look.radius * between(rng, 1.1, 1.4)).max(1.0);
        let shade = look.leaf * between(rng, 0.9, 1.1);
        let towards = out(angle, between(rng, 0.3, 0.8));
        pieces.leaf(at, towards, (length, length * 0.14, 0.15), shade);
    }
    if !look.cob {
        crop.head(pieces, top, Vec3::Y);
        return;
    }
    // Maize: a straw tassel on top, cobs in their husks on the side.
    let tassel = look.flower_size.max(0.4);
    if crop.show().is_some() {
        for i in 0..3 {
            let way = out(i as f32 * TAU / 3.0, 1.5);
            pieces.stem(top, top + way * tassel * 4.0, (0.1, 0.06), look.flower);
        }
    }
    for i in 0..2 {
        let at = top * (0.45 + 0.12 * i as f32);
        let way = out(start + 1.6 + i as f32 * 3.0, 2.0);
        let base = at + way.normalize() * thick;
        if let Some(show) = crop.show() {
            let husk = look.accent.unwrap_or(look.part);
            let colour = if show == Show::Ripe {
                husk
            } else {
                crop.colour()
            };
            let size = look.part_size;
            let end = base + way.normalize() * size.x * 2.0;
            pieces.stem(base, end, (size.y, size.y * 0.6), colour);
        }
    }
}

/// Stalks' colour: straw once the ears are ripe.
fn stalk_colour(look: &Look) -> Vec3 {
    if look.show == Show::Ripe {
        STRAW * 1.3
    } else {
        look.stem()
    }
}
