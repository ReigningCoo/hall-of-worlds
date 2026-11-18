//! Vines. A ground vine (pumpkin, sweet potato) runs along the ground with
//! big leaves, its fruit or pods lying beside its runners. A climber on a
//! pole (runner beans, yams) winds up a pole in masses of leaves, hanging its
//! fruit or pods below them. A climber on sticks (peas) scrambles up a few
//! twiggy sticks leant together.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Crop, Look, around, out};

/// A ground vine.
pub(super) fn ground_vine(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let runners = 4;
    let thick = 0.18;
    let low = look.height * 0.2;
    for i in 0..runners {
        let angle = around(i, runners, rng);
        let bend = angle + between(rng, -0.5, 0.5);
        let middle = out(angle, 0.0) * look.radius * 0.5 + Vec3::Y * low;
        let end = middle + out(bend, 0.0) * look.radius * 0.45;
        pieces.stem(Vec3::Y * low, middle, (thick, thick), look.stem());
        pieces.stem(middle, end, (thick, thick * 0.7), look.stem());
        let quarter = Vec3::Y * low + (middle - Vec3::Y * low) * 0.5;
        for at in [quarter, middle, end] {
            let towards = out(bend + between(rng, -1.2, 1.2), 0.35);
            let length = look
                .leaf_size
                .max(look.radius * 0.28)
                .min(look.radius * 0.4);
            let size = (length, length * 0.9, 0.15);
            pieces.leaf(at, towards, size, look.leaf * between(rng, 0.85, 1.1));
        }
        let side = out(angle + 0.7, 0.0) * look.part_size.y * 1.5;
        let at = if i % 2 == 0 { middle + side } else { end };
        crop.lie(pieces, at.with_y(0.0));
    }
}

/// A climber on its pole.
pub(super) fn pole(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let post = 0.3;
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * look.height,
        (post, post * 0.8),
        WOOD * 1.2,
    );
    let masses = ((5.0 * look.grown).round() as usize).max(2);
    let start = between(rng, 0.0, TAU);
    let wide = look.radius.min(look.height * 0.3);
    for i in 0..masses {
        let up = (i as f32 + 0.5) / masses as f32 * look.height * 0.95;
        let angle = start + i as f32 * 2.1;
        let at = Vec3::Y * up + out(angle, 0.0) * wide * 0.35;
        let size = Vec3::new(wide * 0.5, look.height * 0.12, wide * 0.5) * between(rng, 0.85, 1.15);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.ball(at, size, between(rng, 0.0, 1.6), shade);
        let under = at - Vec3::Y * size.y * 0.6 + out(angle + 0.5, 0.0) * size.x * 0.6;
        crop.hang(pieces, under);
    }
}

/// A climber on sticks.
pub(super) fn sticks(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let sticks = 3;
    let top = Vec3::Y * look.height;
    let spread = look.radius.min(look.height * 0.3);
    for i in 0..sticks {
        let foot = out(around(i, sticks, rng), 0.0) * spread;
        pieces.stem(foot, top + foot * 0.1, (0.15, 0.1), WOOD);
    }
    let masses = ((5.0 * look.grown).round() as usize).max(2);
    for i in 0..masses {
        let up = (i as f32 + 0.5) / masses as f32;
        let angle = between(rng, 0.0, TAU);
        let at = top * up * 0.95 + out(angle, 0.0) * spread * (1.0 - up) * 0.9;
        let s = (spread * 0.55).max(0.8);
        let size = Vec3::new(s, look.height * 0.1, s) * between(rng, 0.85, 1.15);
        pieces.ball(
            at,
            size,
            between(rng, 0.0, 1.6),
            look.leaf * between(rng, 0.85, 1.1),
        );
        crop.hang(
            pieces,
            at - Vec3::Y * size.y * 0.5 + out(angle, 0.0) * size.x * 0.7,
        );
    }
}
