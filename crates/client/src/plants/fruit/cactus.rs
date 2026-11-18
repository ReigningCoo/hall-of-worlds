//! Cacti. A pad cactus (prickly pear) is flat pads standing on each other's
//! edges, its flowers and fruit sitting along the top edges. A column cactus
//! (saguaro) is a tall rounded column with arms bending up, flowering and
//! fruiting at their tips.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::{Dress, Look, around, out};

/// A pad cactus.
pub(super) fn pads(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    dress.sits = true;
    let high = look.height * 0.2;
    let size = Vec3::new(high * 0.75, high, high * 0.18);
    // Pads to stand on, each with its turn, then those standing on them.
    let mut pads: Vec<(Vec3, f32)> = Vec::new();
    for i in 0..2 {
        let angle = around(i, 2, rng);
        let at = out(angle, 0.0) * look.radius * 0.2 + Vec3::Y * high;
        pads.push((at, between(rng, 0.0, std::f32::consts::PI)));
    }
    for level in 0..2 {
        let below: Vec<_> = pads[pads.len() - 2..].to_vec();
        for (at, turn) in below {
            let east = Vec3::new(turn.cos(), 0.0, turn.sin());
            let side = if level == 0 { -0.6 } else { 0.6 };
            let up = at + east * size.x * side + Vec3::Y * size.y * 1.7;
            pads.push((up, turn + between(rng, 0.5, 1.2)));
        }
    }
    for &(at, turn) in &pads {
        let own = size * between(rng, 0.85, 1.1);
        pieces.ball(at, own, turn, look.leaf * between(rng, 0.85, 1.1));
    }
    if dress.bare() {
        return;
    }
    // Along the top edges of the highest pads.
    for &(at, turn) in &pads[2..] {
        let east = Vec3::new(turn.cos(), 0.0, turn.sin());
        for along in [-0.45f32, 0.0, 0.45] {
            let rise = (1.0 - along * along).sqrt();
            dress.at(
                pieces,
                at + east * size.x * along + Vec3::Y * size.y * rise * 0.95,
            );
        }
    }
}

/// A column cactus.
pub(super) fn column(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    dress.sits = true;
    let thick = (look.height * 0.08).max(0.4);
    let tips = arm(pieces, look, Vec3::ZERO, look.height, thick, Vec3::ZERO);
    let mut tops = vec![tips];
    let arms = 1 + (rng.next_u64() % 3) as usize;
    for i in 0..arms {
        let angle = around(i, arms, rng);
        let from = Vec3::Y * look.height * between(rng, 0.35, 0.55);
        let side = out(angle, 0.0) * look.radius * between(rng, 0.6, 0.85);
        let tall = look.height * between(rng, 0.25, 0.4);
        tops.push(arm(pieces, look, from, tall, thick * 0.75, side));
    }
    for top in tops {
        dress.at(pieces, top);
        if !dress.bare() {
            for i in 0..2 {
                let angle = around(i, 2, &mut dress.rng);
                dress.at(pieces, top + out(angle, -0.5) * thick * 0.6);
            }
        }
    }
}

/// A column (or an arm out `side` from `from`, then up `tall`): rounded,
/// as stretched balls; where its top is.
fn arm(pieces: &mut Pieces, look: &Look, from: Vec3, tall: f32, thick: f32, side: Vec3) -> Vec3 {
    let bend = from + side;
    if side != Vec3::ZERO {
        let middle = (from + bend) / 2.0;
        let along = side.length() / 2.0 + thick * 0.5;
        let turn = side.z.atan2(side.x);
        pieces.ball(
            middle,
            Vec3::new(along, thick * 0.85, thick * 0.85),
            turn,
            look.leaf,
        );
    }
    let middle = bend + Vec3::Y * tall / 2.0;
    pieces.ball(
        middle,
        Vec3::new(thick, tall / 2.0 + thick * 0.6, thick),
        0.0,
        look.leaf,
    );
    bend + Vec3::Y * (tall + thick * 0.6)
}
