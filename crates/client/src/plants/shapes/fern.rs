//! Ferns: arching fronds, each a stalk with pairs of leaflets.

use bevy::prelude::*;
use shared::rng::Rng;

use super::{Pieces, Plan, around, between, out};

/// Pairs of leaflets on a frond.
const PAIRS: usize = 4;

/// Fronds rising from the foot and arching out, never flowering.
pub(super) fn fern(pieces: &mut Pieces, plan: &Plan, rng: &mut Rng) {
    let fronds = plan.count(7);
    for i in 0..fronds {
        let angle = around(i, fronds, rng);
        let reach = plan.radius * between(rng, 0.8, 1.0);
        let rise = plan.height * between(rng, 0.75, 1.0);
        let bend = out(angle, 0.0);
        let points = [
            Vec3::ZERO,
            bend * reach * 0.35 + Vec3::Y * rise * 0.75,
            bend * reach * 0.75 + Vec3::Y * rise,
            bend * reach + Vec3::Y * rise * 0.8,
        ];
        for pair in points.windows(2) {
            pieces.stem(pair[0], pair[1], (0.2, 0.15), plan.stem_colour());
        }
        let side = Vec3::new(-angle.sin(), 0.0, angle.cos());
        for pair in 0..PAIRS {
            let share = (pair as f32 + 1.0) / (PAIRS as f32 + 1.0);
            let at = along(&points, 0.3 + share * 0.7);
            let long = plan.leaf * 0.6 * (1.0 - share * 0.5);
            for way in [side, -side] {
                let towards = way + bend * 0.5 - Vec3::Y * 0.2;
                pieces.leaf(at, towards, (long, long * 0.35, 0.15), plan.leaf_colour);
            }
        }
    }
}

/// A point `share` of the way along a line of points, as if all its pieces
/// were the same length.
fn along(points: &[Vec3; 4], share: f32) -> Vec3 {
    let at = share * 3.0;
    let piece = (at.floor() as usize).min(2);
    points[piece].lerp(points[piece + 1], at - piece as f32)
}
