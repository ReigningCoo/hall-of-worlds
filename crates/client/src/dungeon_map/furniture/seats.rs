//! What chairs and benches share: a seat's own frame (from its back
//! forwards, and along its length), where it stands in its cells (pushed a
//! little under a table's edge when it faces one, else back near the wall
//! behind it), and looks hashed from where it stands.

use bevy::prelude::*;
use shared::door::Side;
use shared::dungeon::Level;
use shared::furniture::{Furniture, Piece};
use shared::rng::Seed;

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::{Grain, Part, benches, chairs};

/// How high a seat's top is.
pub(super) const SEAT_TOP: f32 = CELL_SIZE * 0.23;
/// How far a seat facing a table reaches past its own cell, under the
/// table's top (which stands in 0.07 of a cell), stopping short of its legs.
pub(super) const OVERHANG: f32 = CELL_SIZE * 0.09;
/// How far a seat's back stands from the back of its cell, off the wall.
const BACK_GAP: f32 = CELL_SIZE * 0.04;

/// What a seat's parts are worked out from. Parts are given in the seat's
/// own terms, `(forward, up, along)`: forward from the back of the seat
/// (towards where its sitter looks), up from the ground, along its length
/// from one end of its cells; `Grain::Along` runs along its length,
/// `Grain::Across` from back to front.
pub(super) struct Seat {
    /// Along its cells, in world units.
    pub(super) length: f32,
    /// How deep the seat is, from back to front.
    pub(super) depth: f32,
    /// Where its back is, from the back of its cell.
    back: f32,
    /// Whether forward runs west or north, against the frame.
    mirrored: bool,
    seed: Seed,
}

impl Seat {
    /// A number for one choice about how it looks, the same every time.
    pub(super) fn hash(&self, choice: i64) -> u64 {
        self.seed.at(&[choice])
    }

    /// A box from `low` to `high` in the seat's terms, varied by `choice`.
    pub(super) fn part(
        &self,
        low: (f32, f32, f32),
        high: (f32, f32, f32),
        grain: Grain,
        choice: i64,
    ) -> Part {
        let (a, b) = (self.place(low), self.place(high));
        let part = Part::block(a, b, self.grain(grain));
        let hash = self.hash(choice);
        let shade = 0.9 + (hash % 1000) as f32 / 1000.0 * 0.18;
        part.varied(shade, hash & (1 << 20) != 0)
    }

    /// The same, its top moved by `(forward, along)`.
    pub(super) fn leaning(
        &self,
        low: (f32, f32, f32),
        high: (f32, f32, f32),
        grain: Grain,
        (forward, along): (f32, f32),
    ) -> Part {
        let sign = if self.mirrored { -1.0 } else { 1.0 };
        self.part(low, high, grain, 0)
            .leaning(Vec2::new(forward * sign, along))
    }

    /// A point in the seat's terms in the piece's frame (`x` the way it
    /// faces, `z` along its cells).
    fn place(&self, (forward, up, along): (f32, f32, f32)) -> Vec3 {
        let from_back = self.back + forward;
        let x = if self.mirrored {
            CELL_SIZE - from_back
        } else {
            from_back
        };
        Vec3::new(x, up, along)
    }

    fn grain(&self, grain: Grain) -> Grain {
        match grain {
            Grain::Along => Grain::Across,
            Grain::Across => Grain::Along,
            Grain::Up => Grain::Up,
        }
    }
}

/// Whether a seat faces a table (or a bar counter): one is in front of any
/// of its cells.
pub(super) fn faces_table(level: &Level, piece: &Piece) -> bool {
    piece.cells().any(|cell| {
        let (x, y) = piece.facing.beside(cell);
        level
            .furniture_at(x, y)
            .is_some_and(|other| other.furniture.holds_furniture())
    })
}

/// A seat's parts, or `None` for a table. `under_table`: it faces one.
pub(super) fn parts(piece: &Piece, under_table: bool) -> Option<Vec<Part>> {
    let frame = Frame::of(piece);
    let (_, length) = frame.size(piece);
    let depth = match piece.furniture {
        Furniture::Table { .. }
        | Furniture::CraftingBench { .. }
        | Furniture::CaskRack { .. }
        | Furniture::BottleRack { .. }
        | Furniture::BarCounter { .. }
        | Furniture::Fireplace { .. }
        | Furniture::Candles { .. } => {
            return None;
        }
        Furniture::Chair { .. } => chairs::DEPTH,
        Furniture::Bench { .. } => benches::DEPTH,
    };
    let seat = Seat {
        length,
        depth,
        back: if under_table {
            CELL_SIZE + OVERHANG - depth
        } else {
            BACK_GAP
        },
        mirrored: matches!(piece.facing, Side::West | Side::North),
        seed: Seed::new(
            Seed::new(0)
                .derive("furniture look")
                .at(&[piece.at.0, piece.at.1]),
        ),
    };
    Some(match piece.furniture {
        Furniture::Chair { style, arms } => chairs::parts(&seat, style, arms),
        Furniture::Bench { style } => benches::parts(&seat, style),
        Furniture::Table { .. }
        | Furniture::CraftingBench { .. }
        | Furniture::CaskRack { .. }
        | Furniture::BottleRack { .. }
        | Furniture::BarCounter { .. }
        | Furniture::Fireplace { .. }
        | Furniture::Candles { .. } => {
            return None;
        }
    })
}
