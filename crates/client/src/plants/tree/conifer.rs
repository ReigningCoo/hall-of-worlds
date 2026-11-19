//! Conifers' crowns: cones of needles stacked up a trunk, a pine's flat top on
//! its tall bare trunk, wide flat tiers, and a narrow column.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::broadleaf::mass;
use super::look::Look;
use super::{around, out};

/// A narrow spire of tiers down near the ground (spruce, fir).
pub(super) fn spire(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    tiers_of_cones(pieces, look, (5, 0.1, 1.0), rng);
}

/// An open, airy cone (larch): fewer, smaller tiers, the trunk showing.
pub(super) fn open_cone(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    tiers_of_cones(pieces, look, (4, 0.22, 0.8), rng);
}

/// `count` cones from `foot` (a share of the height) up, `width` of the
/// radius at the bottom, narrowing to the top.
fn tiers_of_cones(
    pieces: &mut Pieces,
    look: &Look,
    (count, foot, width): (usize, f32, f32),
    rng: &mut Rng,
) {
    let h = look.height;
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * h * 0.9,
        (look.thick, look.thick * 0.2),
        look.bark,
    );
    let step = h * (1.0 - foot) / count as f32;
    for tier in 0..count {
        let share = 1.0 - tier as f32 / count as f32 * 0.75;
        let base = Vec3::Y * (h * foot + step * tier as f32 * 0.95);
        // The top one comes to a point at the tree's height.
        let tall = if tier + 1 == count {
            h - base.y
        } else {
            step * 1.45
        };
        let radius = look.radius * width * share * between(rng, 0.92, 1.05);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.cone(base, (radius, tall), between(rng, 0.0, 1.0), shade);
    }
}

/// A tall bare trunk under a flat, open top (Scots pine): the trunk orange
/// higher up, a few flat masses round its head.
pub(super) fn pine_top(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let fork = Vec3::Y * h * 0.62;
    pieces.stem(Vec3::ZERO, fork, (look.thick, look.thick * 0.7), look.bark);
    let top = Vec3::Y * h * 0.86;
    pieces.stem(
        fork,
        top,
        (look.thick * 0.7, look.thick * 0.35),
        look.bark * 1.2,
    );
    mass(
        pieces,
        look,
        top,
        Vec3::new(look.radius * 0.6, h * 0.1, look.radius * 0.6),
        rng,
    );
    let limbs = 3;
    for i in 0..limbs {
        let angle = around(i, limbs, rng);
        let tip =
            top + out(angle, 0.0) * look.radius * 0.5 - Vec3::Y * h * between(rng, 0.04, 0.14);
        pieces.stem(fork, tip, (look.thick * 0.4, look.thick * 0.2), look.bark);
        mass(
            pieces,
            look,
            tip,
            Vec3::new(look.radius * 0.45, h * 0.08, look.radius * 0.45),
            rng,
        );
    }
}

/// Wide, flat tiers one over another (cedar of Lebanon, kapok).
pub(super) fn tiers(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * h * 0.88,
        (look.thick, look.thick * 0.35),
        look.bark,
    );
    let count = 3;
    for tier in 0..count {
        let up = 0.45 + tier as f32 * 0.2;
        let share = 1.0 - tier as f32 * 0.25;
        let at = Vec3::Y * h * up + out(between(rng, 0.0, 6.3), 0.0) * look.radius * 0.1;
        let size = Vec3::new(look.radius * share, h * 0.07, look.radius * share);
        mass(pieces, look, at, size, rng);
    }
}

/// A narrow flame-shaped column (cypress): one tall mass with a point.
pub(super) fn column(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * h * 0.3,
        (look.thick, look.thick * 0.7),
        look.bark,
    );
    let at = Vec3::Y * h * 0.5;
    mass(
        pieces,
        look,
        at,
        Vec3::new(look.radius, h * 0.38, look.radius),
        rng,
    );
    let shade = look.leaf * between(rng, 0.85, 1.05);
    pieces.cone(
        Vec3::Y * h * 0.72,
        (look.radius * 0.7, h * 0.28),
        0.0,
        shade,
    );
}
