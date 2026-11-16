//! A piece's own frame: along its length and across it, from its corner.

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::Piece;

use super::super::CELL_SIZE;

/// Where a piece's frame is in the world: `x` along its length (east or
/// south, as it faces), `z` across it, from its north-west corner.
#[derive(Clone, Copy, Debug)]
pub(super) struct Frame {
    corner: Vec3,
    /// Whether its length runs east–west.
    east_west: bool,
}

impl Frame {
    pub(super) fn of(piece: &Piece) -> Self {
        let (x, y) = piece.at;
        Self {
            corner: Vec3::new(
                (x as f32 - 0.5) * CELL_SIZE,
                0.0,
                (y as f32 - 0.5) * CELL_SIZE,
            ),
            east_west: matches!(piece.facing, Side::East | Side::West),
        }
    }

    /// How long and wide the piece is, in world units.
    pub(super) fn size(&self, piece: &Piece) -> (f32, f32) {
        let (width, height) = (f32::from(piece.size.0), f32::from(piece.size.1));
        let (length, across) = if self.east_west {
            (width, height)
        } else {
            (height, width)
        };
        (length * CELL_SIZE, across * CELL_SIZE)
    }

    /// A point (or a direction, without the corner) in the world.
    pub(super) fn turn(&self, local: Vec3) -> Vec3 {
        if self.east_west {
            local
        } else {
            Vec3::new(local.z, local.y, local.x)
        }
    }

    pub(super) fn place(&self, local: Vec3) -> Vec3 {
        self.corner + self.turn(local)
    }
}
