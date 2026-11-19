//! Broadleaf crowns, in the fruit trees' style: a trunk forking into branches
//! under masses of leaves, shaped by the kind's crown.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::look::Look;
use super::{around, out};

/// A narrow upright oval (birch, aspen, alder): masses stacked up the trunk.
pub(super) fn oval(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    pieces.stem(
        Vec3::ZERO,
        Vec3::Y * h * 0.8,
        (look.thick, look.thick * 0.35),
        look.bark,
    );
    for (up, share) in [(0.45, 0.85), (0.64, 1.0), (0.83, 0.7)] {
        let at = Vec3::Y * h * up + out(between(rng, 0.0, 6.3), 0.0) * look.radius * 0.12;
        let size = Vec3::new(look.radius * share, h * 0.17, look.radius * share);
        mass(pieces, look, at, size, rng);
    }
}

/// A dense, high dome (beech, sycamore): one great mass over a few lower ones.
pub(super) fn dome(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let fork = Vec3::Y * h * 0.35;
    pieces.stem(Vec3::ZERO, fork, (look.thick, look.thick * 0.75), look.bark);
    let top = Vec3::Y * h * 0.68;
    pieces.stem(fork, top, (look.thick * 0.6, look.thick * 0.3), look.bark);
    mass(
        pieces,
        look,
        top,
        Vec3::new(look.radius * 0.85, h * 0.3, look.radius * 0.85),
        rng,
    );
    branches(pieces, look, fork, (0.5, 0.55), 0.55, rng);
}

/// Broad and spreading on crooked limbs (oak, elm): masses far out round a
/// lower middle one.
pub(super) fn spreading(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let fork = Vec3::Y * h * 0.32;
    pieces.stem(Vec3::ZERO, fork, (look.thick, look.thick * 0.8), look.bark);
    let middle = Vec3::Y * h * 0.75;
    mass(
        pieces,
        look,
        middle,
        Vec3::new(look.radius * 0.55, h * 0.24, look.radius * 0.55),
        rng,
    );
    branches(pieces, look, fork, (0.68, 0.62), 0.5, rng);
}

/// Rounded with long twigs hanging (willow): a dome, its edge hung with
/// strands down towards the ground.
pub(super) fn weeping(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let fork = Vec3::Y * h * 0.4;
    pieces.stem(Vec3::ZERO, fork, (look.thick, look.thick * 0.7), look.bark);
    let top = Vec3::Y * h * 0.75;
    mass(
        pieces,
        look,
        top,
        Vec3::new(look.radius * 0.8, h * 0.24, look.radius * 0.8),
        rng,
    );
    branches(pieces, look, fork, (0.45, 0.62), 0.45, rng);
    let strands = 11;
    for i in 0..strands {
        let angle = around(i, strands, rng);
        let from = top + out(angle, 0.0) * look.radius * between(rng, 0.75, 0.95);
        let long = h * between(rng, 0.38, 0.5);
        let width = look.radius * 0.28;
        let shade = look.leaf * between(rng, 0.9, 1.15);
        pieces.leaf(from, out(angle, -6.0), (long, width, width * 0.3), shade);
    }
}

/// A flat-topped umbrella (thorn trees, mahogany): a trunk forking high into
/// limbs under a wide, thin crown.
pub(super) fn umbrella(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let fork = Vec3::Y * h * 0.5;
    pieces.stem(Vec3::ZERO, fork, (look.thick, look.thick * 0.7), look.bark);
    let top = Vec3::Y * h * 0.86;
    mass(
        pieces,
        look,
        top,
        Vec3::new(look.radius * 0.7, h * 0.12, look.radius * 0.7),
        rng,
    );
    let limbs = 3;
    for i in 0..limbs {
        let angle = around(i, limbs, rng);
        let tip = top + out(angle, 0.0) * look.radius * 0.5 - Vec3::Y * h * 0.03;
        pieces.stem(fork, tip, (look.thick * 0.5, look.thick * 0.25), look.bark);
        mass(
            pieces,
            look,
            tip,
            Vec3::new(look.radius * 0.5, h * 0.1, look.radius * 0.5),
            rng,
        );
    }
}

/// Low and spreading on several stems (mesquite).
pub(super) fn shrubby(pieces: &mut Pieces, look: &Look, rng: &mut Rng) {
    let h = look.height;
    let stems = 3;
    for i in 0..stems {
        let angle = around(i, stems, rng);
        let tip = out(angle, 0.0) * look.radius * 0.45 + Vec3::Y * h * between(rng, 0.55, 0.7);
        pieces.stem(
            Vec3::ZERO,
            tip,
            (look.thick * 0.6, look.thick * 0.3),
            look.bark,
        );
        let size = Vec3::new(look.radius * 0.6, h * 0.22, look.radius * 0.6);
        mass(pieces, look, tip, size, rng);
    }
    let top = Vec3::Y * h * 0.75;
    mass(
        pieces,
        look,
        top,
        Vec3::new(look.radius * 0.5, h * 0.2, look.radius * 0.5),
        rng,
    );
}

/// Branches from `fork` out to masses round the crown: `(out, up)` as shares
/// of the tree's radius and height, each mass `share` of the radius.
fn branches(
    pieces: &mut Pieces,
    look: &Look,
    fork: Vec3,
    (reach, up): (f32, f32),
    share: f32,
    rng: &mut Rng,
) {
    let count = 3 + (rng.next_u64() % 2) as usize;
    for i in 0..count {
        let angle = around(i, count, rng);
        let tip = out(angle, 0.0) * look.radius * reach * between(rng, 0.85, 1.05)
            + Vec3::Y * look.height * up * between(rng, 0.9, 1.08);
        pieces.stem(fork, tip, (look.thick * 0.5, look.thick * 0.22), look.bark);
        let size = Vec3::new(look.radius, look.height * 0.2, look.radius) * share;
        mass(pieces, look, tip, size * between(rng, 0.9, 1.1), rng);
    }
}

/// A mass of leaves at `at`, `size` its half widths, in a shade of the leaves.
pub(super) fn mass(pieces: &mut Pieces, look: &Look, at: Vec3, size: Vec3, rng: &mut Rng) {
    let shade = look.leaf * between(rng, 0.85, 1.1);
    pieces.ball(at, size, between(rng, 0.0, 1.6), shade);
}
