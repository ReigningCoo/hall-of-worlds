//! Rosettes. A leafy rosette (cabbage, lettuce, rhubarb) is a ring of broad
//! leaves arching out from the foot, with its heart, stalks or flower head in
//! the middle when full grown. A succulent (purslane) is a low mat of fleshy
//! leaves on trailing stems. An agave is a ring of thick spiked leaves.

use bevy::prelude::*;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::between;
use super::look::{Bears, Show};
use super::{Crop, Look, around, out};

/// A leafy rosette.
pub(super) fn rosette(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let leaves = ((8.0 * look.grown).round() as usize).max(3);
    let low = look.height * 0.25;
    let stalks = look.bears == Bears::Heart && look.part_size.x > look.part_size.y * 2.5;
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let rise = between(rng, 0.3, 0.9);
        let base = out(angle, 0.0) * look.radius * 0.1 + Vec3::Y * low * between(rng, 0.4, 1.0);
        let length = look.radius * between(rng, 0.8, 1.0);
        let width = length * between(rng, 0.55, 0.75);
        let shade = look.leaf * between(rng, 0.85, 1.1);
        if stalks {
            // Rhubarb: each leaf on a stalk of its own, red or green.
            let tip = out(angle, rise * 1.6) * length * 0.55;
            pieces.stem(Vec3::ZERO, tip, (0.3, 0.25), look.part);
            pieces.leaf(
                tip,
                out(angle, rise * 0.4),
                (length * 0.6, width, 0.2),
                shade,
            );
        } else {
            pieces.leaf(base, out(angle, rise), (length, width, 0.25), shade);
        }
    }
    heart(pieces, look, crop, stalks);
    crop.shoots(pieces, look.radius * 0.5);
    crop.shoulder(pieces);
    crop.flowers_over_hidden(pieces, Vec3::Y * look.height * 0.6);
}

/// What is in a rosette's middle when full grown: a head of leaves, or a
/// flower head on its stalk (an artichoke's, green until it is ready).
fn heart(pieces: &mut Pieces, look: &Look, crop: &mut Crop, stalks: bool) {
    match (crop.show(), look.bears) {
        (Some(Show::Unripe | Show::Ripe), Bears::Heads) => {
            let tip = Vec3::Y * look.height * 0.85;
            pieces.stem(Vec3::ZERO, tip, (0.35, 0.3), look.stem());
            let s = look.part_size.y;
            pieces.ball(
                tip + Vec3::Y * s,
                Vec3::new(s, s * 1.1, s),
                0.3,
                crop.colour(),
            );
        }
        (Some(Show::Ripe), Bears::Heart) if !stalks => {
            let s = look.radius * 0.45;
            let size = Vec3::new(s, s * 0.85, s);
            pieces.ball(
                Vec3::Y * s * 0.9,
                size,
                crop.between(0.0, 1.6),
                crop.colour(),
            );
        }
        _ => {}
    }
}

/// A succulent's mat.
pub(super) fn succulent(pieces: &mut Pieces, look: &Look, rng: &mut Rng, crop: &mut Crop) {
    let stems = ((6.0 * look.grown).round() as usize).max(3);
    let stem = look.leaf * Vec3::new(1.5, 0.6, 0.6);
    let low = look.height * 0.3;
    for i in 0..stems {
        let angle = around(i, stems, rng);
        let end = out(angle, 0.0) * look.radius * between(rng, 0.7, 1.0) + Vec3::Y * low;
        pieces.stem(Vec3::Y * low * 0.5, end, (0.2, 0.15), stem);
        for along in [0.5, 1.0] {
            let at = Vec3::Y * low * 0.5 + (end - Vec3::Y * low * 0.5) * along;
            let s = look.leaf_size.max(0.6) * 0.5;
            let shade = look.leaf * between(rng, 0.9, 1.1);
            pieces.blob(at + Vec3::Y * s * 0.4, Vec3::new(s, s * 0.6, s), shade);
        }
        if i % 2 == 0 && crop.show().is_some() {
            crop.flower(pieces, end + Vec3::Y * look.flower_size);
        }
    }
}

/// An agave.
pub(super) fn agave(pieces: &mut Pieces, look: &Look, rng: &mut Rng, _crop: &mut Crop) {
    let leaves = ((12.0 * look.grown).round() as usize).max(5);
    for i in 0..leaves {
        let angle = around(i, leaves, rng);
        let rise = between(rng, 0.5, 2.2);
        let towards = out(angle, rise);
        // As far as its height up or its radius out lets it go.
        let reach = (look.height / rise).min(look.radius);
        let length = reach * towards.length() * between(rng, 0.8, 1.0);
        let width = length * 0.18;
        let shade = look.leaf * between(rng, 0.85, 1.1);
        pieces.leaf(Vec3::Y * 0.2, towards, (length, width, width * 0.5), shade);
    }
}
