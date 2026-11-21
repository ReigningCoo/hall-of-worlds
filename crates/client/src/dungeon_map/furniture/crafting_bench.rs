//! Crafting benches (`Furniture::CraftingBench`, crafting benches plan PR 3):
//! a joined table with a second layer raised over its rear third, the side
//! by the wall behind it, and where each of its tools stands: tall ones on
//! that shelf, the rest on the table in front of it. Hearths are drawn by
//! `hearth.rs`.

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::{Furniture, Piece};
use shared::item::ItemKind;

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::tables::Table;
use super::{Grain, INSET, Part, TABLE_TOP, wooden_tables};

/// How high the shelf's top is, and how thick.
const SHELF_TOP: f32 = TABLE_TOP + CELL_SIZE * 0.24;
const SHELF_THICKNESS: f32 = CELL_SIZE * 0.04;
/// How thick the boards holding the shelf up are.
const END_BOARD: f32 = CELL_SIZE * 0.04;

/// Tools that stand on the shelf; the rest stand in front of it.
const TALL: [ItemKind; 3] = [ItemKind::Retort, ItemKind::Alembic, ItemKind::Pelican];
const LOW: [ItemKind; 3] = [
    ItemKind::MortarAndPestle,
    ItemKind::Crucible,
    ItemKind::Filter,
];

/// A bench's parts (a hearth's are `hearth.rs`'s), or `None` for anything
/// else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let Furniture::CraftingBench { craft } = piece.furniture else {
        return None;
    };
    if craft.is_hearth() {
        return super::hearth::parts(piece);
    }
    let table = Table::of(piece);
    let mut parts = wooden_tables::joined(&table);
    let (back, front) = shelf_band(piece, table.length);
    let (near, far) = (INSET, table.width - INSET);
    let shelf = Part::block(
        Vec3::new(back, SHELF_TOP - SHELF_THICKNESS, near),
        Vec3::new(front, SHELF_TOP, far),
        Grain::Across,
    );
    parts.push(table.vary(shelf, 60));
    // A board under each end of it, and one in the middle of a long one.
    let under = SHELF_TOP - SHELF_THICKNESS;
    let mut boards = vec![near, far - END_BOARD];
    if table.width > CELL_SIZE * 1.5 {
        boards.push((table.width - END_BOARD) / 2.0);
    }
    for (i, z) in boards.into_iter().enumerate() {
        let board = Part::block(
            Vec3::new(back, TABLE_TOP, z),
            Vec3::new(front, under, z + END_BOARD),
            Grain::Up,
        );
        parts.push(table.vary(board, 61 + i as i64));
    }
    Some(parts)
}

/// Its parts as plain boxes in the world (on flat ground: the caller raises
/// them with the ground), lowest corner first, for Create Crafting Bench's
/// ghost; nothing for anything else.
pub(in crate::dungeon_map) fn ghost_boxes(piece: &Piece) -> Vec<(Vec3, Vec3)> {
    let frame = Frame::of(piece);
    let boxes = parts(piece).unwrap_or_default().into_iter().map(|part| {
        let (a, b) = (frame.place(part.low), frame.place(part.high));
        (a.min(b), a.max(b))
    });
    boxes.collect()
}

/// Where across its depth the shelf lies: its rear third, by the wall behind
/// (a bench faces away from it, `Piece::facing`), within the top's inset.
fn shelf_band(piece: &Piece, depth: f32) -> (f32, f32) {
    let third = (depth - 2.0 * INSET) / 3.0;
    match piece.facing {
        // Its frame runs east or south from the wall's side.
        Side::East | Side::South => (INSET, INSET + third),
        Side::West | Side::North => (depth - INSET - third, depth - INSET),
    }
}

/// Where a tool of `kind` stands on the bench `piece` in the world (on the
/// level's flat floor: the caller raises it with the ground), and how much
/// room it has across the ground, or `None` for a kind it never shows.
pub(in crate::dungeon_map) fn tool_spot(piece: &Piece, kind: ItemKind) -> Option<(Vec3, f32)> {
    if let Some((local, room)) = super::hearth::tool_place(piece, kind) {
        return Some((Frame::of(piece).place(local), room));
    }
    let table = Table::of(piece);
    let (back, front) = shelf_band(piece, table.length);
    let (place, depth, height) = match TALL.iter().position(|&tall| tall == kind) {
        Some(place) => (place, (back + front) / 2.0, SHELF_TOP),
        None => {
            let place = LOW.iter().position(|&low| low == kind)?;
            // The middle of the rest of the top.
            let (from, to) = match back < table.length / 2.0 {
                true => (front, table.length - INSET),
                false => (INSET, back),
            };
            (place, (from + to) / 2.0, TABLE_TOP)
        }
    };
    let along = table.width * (1.0 + 2.0 * place as f32) / 6.0;
    let room = (table.width / 3.0).min(front - back + CELL_SIZE * 0.08);
    let frame = Frame::of(piece);
    Some((frame.place(Vec3::new(depth, height, along)), room))
}

#[cfg(test)]
mod tests;
