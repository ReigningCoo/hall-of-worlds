//! Water plants and ferns. A tall water plant (cattail) is a fan of upright
//! strap leaves, with a brown spike in flower; a lower one (wapato, water
//! spinach) holds broad leaves up on stalks, its flowers among them, and its
//! shoots showing once full grown. A fern (fiddlehead) is a ring of fronds
//! arching out, with curled young fronds in its middle when full grown.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, between};
use super::look::Show;
use super::{Crop, Look, around, out};

/// Water plants taller than this, in metres, are reeds with strap leaves.
const REED: f32 = 1.05;

/// A water plant.
pub(super) fn water(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    if look.height / look.grown > REED * METRE {
        reed(pieces, look, rng, crop);
        return;
    }
    let stalks = ((6.0 * look.grown).round() as usize).max(3);
    for i in 0..stalks {
        let angle = around(i, stalks, rng);
        let tip =
            Vec3::Y * look.height * between(rng, 0.55, 0.85) + out(angle, 0.0) * look.radius * 0.5;
        pieces.stem(Vec3::ZERO, tip, (0.2, 0.15), look.stem());
        let long = look.leaf_size.min(look.radius * 0.8).max(1.0);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(tip, out(angle, 0.5), (long, long * 0.55, 0.2), shade);
        if i % 2 == 0 {
            match crop.show() {
                Some(Show::Flowers) => crop.flower(pieces, tip + Vec3::Y * 0.5),
                Some(Show::Ripe) => {
                    let size = look.part_size;
                    let way = out(angle + 0.5, 1.5).normalize();
                    let colour = crop.colour();
                    pieces.stem(tip, tip + way * size.x, (size.y, size.y * 0.6), colour);
                }
                _ => {}
            }
        }
    }
}

/// A reed: strap leaves, and a spike in flower or seed.
fn reed(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let leaves = ((7.0 * look.grown).round() as usize).max(3);
    let wide = (look.radius * 0.2).max(0.4);
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let long = look.height * between(rng, 0.7, 1.0);
        let shade = look.leaf * between(rng, 0.9, 1.1);
        pieces.leaf(Vec3::ZERO, out(angle, 6.0), (long, wide, 0.2), shade);
    }
    if crop.show().is_some() {
        let top = Vec3::Y * look.height * 1.02;
        pieces.stem(Vec3::ZERO, top, (0.15, 0.12), look.stem());
        let spike = (look.height * 0.12).max(1.0);
        let thick = (spike * 0.15).max(0.3);
        let from = top - Vec3::Y * spike * 1.4;
        pieces.stem(from, from + Vec3::Y * spike, (thick, thick), look.flower);
    }
}

/// A fern.
pub(super) fn fern(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let fronds = ((7.0 * look.grown).round() as usize).max(3);
    for i in 0..fronds {
        let angle = around(i, fronds, rng);
        let rise = between(rng, 1.0, 2.0);
        let middle = out(angle, rise) * look.radius * 0.5;
        let long = look.radius * between(rng, 0.55, 0.7);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(
            Vec3::ZERO,
            middle,
            (middle.length(), long * 0.35, 0.15),
            shade,
        );
        pieces.leaf(middle, out(angle, -0.2), (long, long * 0.4, 0.15), shade);
    }
    if crop.show() != Some(Show::Ripe) {
        return;
    }
    // Fiddleheads: young fronds still curled, on short stalks.
    for i in 0..4 {
        let angle = i as f32 / 4.0 * TAU + crop.between(-0.4, 0.4);
        let s = look.part_size.y.max(0.3);
        let tip = out(angle, 0.0) * look.radius * 0.15 + Vec3::Y * s * 3.0;
        pieces.stem(Vec3::ZERO, tip, (s * 0.4, s * 0.4), crop.colour());
        pieces.ball(
            tip + Vec3::Y * s * 0.6,
            Vec3::splat(s),
            angle,
            crop.colour(),
        );
    }
}
