//! Tables' parts by style, and what all tables share: their size, their
//! board top, and looks hashed from where they stand.

use bevy::prelude::*;
use shared::furniture::{Furniture, Piece, TableStyle};
use shared::rng::Seed;

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::{Grain, INSET, Part, TABLE_TOP, TOP_THICKNESS, stone_tables, wooden_tables};

/// How wide a top's boards are, about, and the gap between them.
const BOARD_WIDTH: f32 = CELL_SIZE * 0.3;
const BOARD_GAP: f32 = CELL_SIZE * 0.01;

/// What a table's parts are worked out from.
pub(super) struct Table {
    /// Along its length and across it, in world units.
    pub(super) length: f32,
    pub(super) width: f32,
    /// Where its top's underside is.
    pub(super) under: f32,
    seed: Seed,
}

impl Table {
    /// A number for one choice about how it looks, the same every time.
    pub(super) fn hash(&self, choice: i64) -> u64 {
        self.seed.at(&[choice])
    }

    /// A part's shade and turn, varied by `choice`.
    pub(super) fn vary(&self, part: Part, choice: i64) -> Part {
        let hash = self.hash(choice);
        let shade = 0.9 + (hash % 1000) as f32 / 1000.0 * 0.18;
        part.varied(shade, hash & (1 << 20) != 0)
    }

    /// A wooden top of boards along its length.
    pub(super) fn board_top(&self) -> Vec<Part> {
        let across = self.width - 2.0 * INSET;
        let count = (across / BOARD_WIDTH).round().max(2.0);
        let board = (across - BOARD_GAP * (count - 1.0)) / count;
        (0..count as i64)
            .map(|i| {
                let z = INSET + i as f32 * (board + BOARD_GAP);
                let part = Part::block(
                    Vec3::new(INSET, self.under, z),
                    Vec3::new(self.length - INSET, TABLE_TOP, z + board),
                    Grain::Along,
                );
                self.vary(part, i)
            })
            .collect()
    }
}

/// A table's parts, or `None` for anything else (seats: `seats::parts`).
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let Furniture::Table { style } = piece.furniture else {
        return None;
    };
    let frame = Frame::of(piece);
    let (length, width) = frame.size(piece);
    let table = Table {
        length,
        width,
        under: TABLE_TOP - TOP_THICKNESS,
        seed: Seed::new(
            Seed::new(0)
                .derive("furniture look")
                .at(&[piece.at.0, piece.at.1]),
        ),
    };
    Some(match style {
        TableStyle::Trestle => wooden_tables::trestle(&table),
        TableStyle::Joined => wooden_tables::joined(&table),
        TableStyle::PlankLegs => wooden_tables::plank_legs(&table),
        TableStyle::Pedestal => wooden_tables::pedestal(&table),
        TableStyle::StoneSlab => stone_tables::slab(&table),
        TableStyle::StoneBlock => stone_tables::block(&table),
    })
}
