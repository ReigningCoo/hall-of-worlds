//! Vines. A climbing vine (grape, kiwi, passion fruit) grows up a post and
//! along a rail between two, as it would on a trellis, with its fruit hanging
//! under the leaves. A ground vine (melons) runs along the ground with big
//! leaves, its fruit lying beside its runners.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Dress, Look, Show, around, out};

/// A climbing vine on its trellis.
pub(super) fn vine(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let post = (look.height * 0.03).clamp(0.2, 0.45);
    let half = look.radius * 0.85;
    let top = look.height;
    // The trellis: two posts and a rail, the same way round on every one.
    for side in [-half, half] {
        let foot = Vec3::X * side;
        pieces.stem(foot, foot + Vec3::Y * top, (post, post), WOOD * 1.2);
    }
    let rail = Vec3::Y * top * 0.95;
    pieces.stem(
        rail - Vec3::X * half,
        rail + Vec3::X * half,
        (post * 0.6, post * 0.6),
        WOOD * 1.2,
    );
    // The vine: up the middle, then along the rail both ways.
    let stem = look.leaf * Vec3::new(1.4, 0.7, 0.5);
    let thick = post * 0.5;
    pieces.stem(Vec3::ZERO, rail, (thick, thick * 0.8), stem);
    let masses = 4;
    for i in 0..masses {
        let along = (i as f32 + 0.5) / masses as f32 * 2.0 - 1.0;
        let at = rail + Vec3::X * along * half * between(rng, 0.85, 1.0)
            - Vec3::Y * top * between(rng, 0.0, 0.08);
        let size = Vec3::new(half * 0.4, top * 0.16, look.radius * 0.35) * between(rng, 0.9, 1.15);
        pieces.ball(
            at,
            size,
            between(rng, -0.3, 0.3),
            look.leaf * between(rng, 0.85, 1.1),
        );
        if !dress.bare() {
            let under = at - Vec3::Y * size.y * 0.7;
            let side = out(dress.between(0.0, TAU), 0.0) * size.x * 0.5;
            dress.at(pieces, under + side);
        }
    }
}

/// A ground vine's runners, leaves and fruit.
pub(super) fn ground_vine(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    let runners = 4;
    let thick = 0.15;
    let stem = look.leaf * 0.8;
    let low = look.height * 0.15;
    for i in 0..runners {
        let angle = around(i, runners, rng);
        let bend = angle + between(rng, -0.5, 0.5);
        let middle = out(angle, 0.0) * look.radius * 0.5 + Vec3::Y * low;
        let end = middle + out(bend, 0.0) * look.radius * 0.45;
        pieces.stem(Vec3::Y * low, middle, (thick, thick), stem);
        pieces.stem(middle, end, (thick, thick * 0.7), stem);
        for at in [middle, end] {
            let towards = out(bend + between(rng, -1.2, 1.2), 0.3);
            let length = look.leaf_size.min(look.radius * 0.3);
            let size = (length, length * 0.9, length * 0.1);
            pieces.leaf(at, towards, size, look.leaf * between(rng, 0.85, 1.1));
        }
        // Fruit lies on the ground; flowers stand a little above it.
        let y = if dress.look.show == Show::Flowers {
            low
        } else {
            0.0
        };
        if i % 2 == 0 {
            let side = out(angle + 0.6, 0.0) * dress.look.fruit_size * 1.2;
            dress.at(pieces, middle.with_y(y) + side);
        } else {
            dress.at(pieces, end.with_y(y));
        }
    }
}
