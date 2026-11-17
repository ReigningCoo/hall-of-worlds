//! The collar and the shank, from the bow out to the tip: a skeleton key's
//! hollow pipe end, or a mortice key's solid end with a stop before the bit.

use bevy::prelude::*;

use super::look::{Collar, KeyLook};
use super::piece::{Finish, Piece, Shape};

/// Where the bit goes along x, and how thick the shank is: what the bit is built against.
#[derive(Clone, Copy, Debug)]
pub(super) struct Shank {
    /// Where the bit starts and ends.
    pub(super) bit_from: f32,
    pub(super) bit_to: f32,
    /// Its half thickness.
    pub(super) radius: f32,
}

/// The shank's pieces, from `start` (the bow's reach) out.
pub(super) fn shank(look: &KeyLook, start: f32, bit_width: f32) -> (Vec<Piece>, Shank) {
    let radius = 0.05 + 0.014 * look.thickness as f32;
    let length = 1.25 + 0.16 * look.length as f32;
    let tip = start + length;
    // A mortice key's tip runs a little past its bit.
    let bit_to = if look.pipe { tip } else { tip - radius * 1.6 };
    let bit_from = bit_to - bit_width;
    let from = start - 0.03;
    let mut pieces = vec![if look.square {
        Piece::block(
            Vec2::new(from, -radius),
            Vec2::new(tip, radius),
            radius * 2.0,
        )
    } else {
        Piece::along(from, tip, radius)
    }];
    if look.pipe {
        // The pipe's end, a little thicker, and the dark hole in it.
        pieces.push(Piece::along(bit_from - 0.12, tip, radius * 1.3));
        let mut hole = Piece::along(tip, tip + 0.012, radius * 0.75);
        hole.finish = Finish::Hole;
        pieces.push(hole);
    } else {
        // The stop: a flat disc just before the bit.
        pieces.push(Piece::along(bit_from - 0.08, bit_from - 0.04, radius * 2.0));
    }
    pieces.extend(collar(look.collar, start, radius));
    let shank = Shank {
        bit_from,
        bit_to,
        radius,
    };
    (pieces, shank)
}

/// What sits on the shank where it leaves the bow.
fn collar(collar: Collar, start: f32, radius: f32) -> Vec<Piece> {
    let bead = |x: f32, size: f32, long: f32| {
        Piece::new(
            Shape::Bead(size),
            Transform::from_xyz(x, 0.0, 0.0).with_scale(Vec3::new(long, 1.0, 1.0)),
        )
    };
    match collar {
        Collar::None => Vec::new(),
        Collar::Bead => vec![bead(start + radius * 1.6, radius * 1.9, 1.0)],
        Collar::Rings => [0.08, 0.17]
            .map(|x| Piece::round_shank(start + x, radius * 1.8, radius * 0.9))
            .to_vec(),
        Collar::Baluster => vec![
            bead(start + radius * 1.4, radius * 1.6, 0.9),
            bead(start + radius * 4.2, radius * 2.0, 1.5),
            bead(start + radius * 7.0, radius * 1.4, 0.9),
        ],
    }
}
