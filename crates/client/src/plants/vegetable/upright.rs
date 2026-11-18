//! Upright plants. An upright leafy herb (potato, kale, celery, quinoa) is a
//! few stems with leaves along them, topped by flowers or its heads; spears
//! come up round the foot of one picked for its shoots (asparagus). A bush
//! (okra, beans, lentils) is stems from the foot, each ending in a mass of
//! leaves, with flowers, fruit or pods hanging round them.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{METRE, between};
use super::look::Bears;
use super::{Crop, Look, around, out};

/// Bushes narrower than this (half their width, in metres) are small.
const SMALL_BUSH: f32 = 0.32;

/// An upright leafy herb.
pub(super) fn upright(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let stems = if look.radius > look.height * 0.45 {
        3
    } else {
        1
    };
    let thick = (look.height * 0.03).clamp(0.2, 0.5);
    // Stalks picked (leek, celery) are drawn in their own colour.
    let stalk = if look.bears == Bears::Heart && look.part_size.x > look.part_size.y * 2.5 {
        look.part
    } else {
        look.stem()
    };
    let levels = ((5.0 * look.grown).round() as usize).max(2);
    for s in 0..stems {
        let lean = if stems == 1 {
            Vec3::ZERO
        } else {
            out(around(s, stems, rng), 0.0) * look.radius * 0.35
        };
        let top = Vec3::Y * look.height * between(rng, 0.85, 1.0) + lean;
        pieces.stem(Vec3::ZERO, top, (thick, thick * 0.5), stalk);
        let start = between(rng, 0.0, TAU);
        for level in 0..levels {
            let up = (level as f32 + 0.6) / (levels as f32 + 0.4);
            let at = top * up;
            let angle = start + level as f32 * 2.4;
            let length = look.leaf_size.min(look.radius).max(look.radius * 0.6);
            let towards = out(angle, between(rng, 0.1, 0.6));
            let shade = look.leaf * between(rng, 0.85, 1.1);
            pieces.leaf(at, towards, (length, length * 0.6, 0.2), shade);
            if matches!(look.bears, Bears::Fruit | Bears::Pods { .. }) && level % 2 == 1 {
                crop.hang(pieces, at + out(angle + 1.2, 0.0) * thick * 2.0);
            }
        }
        upright_top(pieces, look, crop, top);
    }
    crop.shoots(pieces, look.radius);
    crop.shoulder(pieces);
}

/// What tops an upright herb's stem: its heads, or flowers.
fn upright_top(pieces: &mut Pieces, look: &Look, crop: &mut Crop, top: Vec3) {
    if look.bears == Bears::Heads {
        crop.head(pieces, top, Vec3::Y);
    } else {
        crop.flowers_over_hidden(pieces, top);
    }
}

/// A bush.
pub(super) fn bush(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    // Small bushes stand three to a row: two stems each keeps a field cheap.
    let small = look.radius < SMALL_BUSH * METRE;
    let stems = if small { 2 } else { 3 };
    let thick = (look.height * 0.03).clamp(0.2, 0.5);
    let size = Vec3::new(look.radius * 0.55, look.height * 0.32, look.radius * 0.55);
    let middle = Vec3::Y * look.height * 0.66;
    let mut masses = if small { Vec::new() } else { vec![middle] };
    for i in 0..stems {
        let angle = around(i, stems, rng);
        let tip = Vec3::Y * look.height * between(rng, 0.45, 0.6)
            + out(angle, 0.0) * look.radius * between(rng, 0.4, 0.55);
        pieces.stem(Vec3::ZERO, tip, (thick, thick * 0.5), look.stem());
        masses.push(tip);
    }
    for at in masses {
        let own = size * between(rng, 0.85, 1.15);
        let turn = between(rng, 0.0, 1.6);
        pieces.ball(at, own, turn, look.leaf * between(rng, 0.85, 1.1));
        // Pods are thin: two a mass, so they show.
        let hanging = if matches!(look.bears, Bears::Pods { .. }) {
            2
        } else {
            1
        };
        for _ in 0..hanging {
            let angle = crop.between(0.0, TAU);
            let rise = crop.between(-0.4, 0.3);
            let flat = (1.0 - rise * rise).sqrt();
            let way = Vec3::new(angle.cos() * flat, rise, angle.sin() * flat);
            crop.hang(pieces, at + way * own * 0.9);
        }
    }
    crop.flowers_over_hidden(pieces, middle + Vec3::Y * size.y);
}
