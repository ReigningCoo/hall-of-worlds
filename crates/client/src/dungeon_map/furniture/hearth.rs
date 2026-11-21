//! Hearths (`Craft::Stove`, `Hearth`, `Range`; cooking plan PR 8): stone, on a
//! plinth. Their first cell is a stove: two jambs and a back round an open
//! firebox under a slab, a hood rising from its back. A Hearth adds a stone
//! counter for its kitchen tools; a Range a longer one with drawers on its
//! front. Flames in a lit one's firebox are `hearth_fires`'.

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::{Furniture, Piece};
use shared::item::ItemKind;

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::tables::Table;
use super::{Grain, INSET, Part};

/// How high a stove's and a counter's top is, and how thick.
const STOVE_TOP: f32 = CELL_SIZE * 0.5;
const SLAB: f32 = CELL_SIZE * 0.07;
/// The plinth all of it stands on.
const PLINTH: f32 = CELL_SIZE * 0.06;
/// How wide each jamb of the firebox is, and how far back its back stands
/// (as a share of the depth, from the back).
const JAMB: f32 = CELL_SIZE * 0.2;
const FIREBOX_BACK: f32 = 0.35;
/// How high the hood rises.
const HOOD_TOP: f32 = CELL_SIZE * 1.1;

/// Where a hearth's piece puts things: across its depth from the back (0)
/// to the front (1), and along the wall from its stove.
struct Shape {
    frame_depth: f32,
    length: f32,
    back_low: bool,
}

impl Shape {
    fn of(piece: &Piece) -> Self {
        let table = Table::of(piece);
        Self {
            // A bench's frame runs across it (`x`) and along the wall (`z`).
            frame_depth: table.length,
            length: table.width,
            back_low: matches!(piece.facing, Side::East | Side::South),
        }
    }

    /// The frame's `x` at `t` of the way from the back to the front.
    fn depth(&self, t: f32) -> f32 {
        let inner = self.frame_depth - 2.0 * INSET;
        match self.back_low {
            true => INSET + t * inner,
            false => self.frame_depth - INSET - t * inner,
        }
    }

    /// A stone block from back `t0` to front `t1`, `y0` to `y1`, along
    /// `z0` to `z1`.
    fn block(&self, (t0, t1): (f32, f32), (y0, y1): (f32, f32), (z0, z1): (f32, f32)) -> Part {
        Part::block(
            Vec3::new(self.depth(t0), y0, z0),
            Vec3::new(self.depth(t1), y1, z1),
            Grain::Up,
        )
    }
}

/// A hearth's parts, or `None` for anything else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let Furniture::CraftingBench { craft } = piece.furniture else {
        return None;
    };
    if !craft.is_hearth() {
        return None;
    }
    let table = Table::of(piece);
    let shape = Shape::of(piece);
    let (c, end) = (CELL_SIZE, shape.length - INSET);
    let under = STOVE_TOP - SLAB;
    let mut parts = vec![
        shape.block((0.0, 1.0), (0.0, PLINTH), (INSET, end)),
        // The stove: jambs, the firebox's back, its slab and the hood.
        shape.block((0.0, 1.0), (PLINTH, under), (INSET, INSET + JAMB)),
        shape.block((0.0, 1.0), (PLINTH, under), (c - INSET - JAMB, c - INSET)),
        shape.block(
            (0.0, FIREBOX_BACK),
            (PLINTH, under),
            (INSET + JAMB, c - INSET - JAMB),
        ),
        shape.block((0.0, 1.0), (under, STOVE_TOP), (INSET, c - INSET)),
        shape.block(
            (0.0, FIREBOX_BACK),
            (STOVE_TOP, HOOD_TOP),
            (INSET + 0.05 * c, c - INSET - 0.05 * c),
        ),
    ];
    if shape.length > c * 1.5 {
        // The counter, its slab overhanging a little at the front.
        parts.push(shape.block((0.0, 0.92), (PLINTH, under), (c, end)));
        parts.push(shape.block((0.0, 1.0), (under, STOVE_TOP), (c - INSET, end)));
    }
    if craft.has_storage() {
        // Two drawers on the counter's front under the far cell, each with a
        // knob.
        let (from, to) = (2.0 * c + 0.06 * c, end - 0.06 * c);
        for (low, high) in [(0.1, 0.24), (0.27, 0.41)] {
            let (low, high) = (low * c, high * c);
            parts.push(shape.block((0.92, 0.97), (low, high), (from, to)));
            let middle = (from + to) / 2.0;
            let knob = (middle - 0.03 * c, middle + 0.03 * c);
            let rise = (low + high) / 2.0;
            parts.push(shape.block((0.97, 1.0), (rise - 0.02 * c, rise + 0.02 * c), knob));
        }
    }
    let varied = parts.into_iter().enumerate();
    Some(
        varied
            .map(|(i, part)| table.vary(part, 70 + i as i64))
            .collect(),
    )
}

/// Where a kitchen tool of `kind` stands on hearth `piece` in the frame (on
/// flat floor), and how much room it has across, or `None`: the Cauldron on
/// the stove, the rest along the counter in front of the wall.
pub(super) fn tool_place(piece: &Piece, kind: ItemKind) -> Option<(Vec3, f32)> {
    let Furniture::CraftingBench { craft } = piece.furniture else {
        return None;
    };
    if !craft.is_hearth() {
        return None;
    }
    let place = craft.tools().iter().position(|&tool| tool == kind)?;
    let shape = Shape::of(piece);
    let c = CELL_SIZE;
    if kind == ItemKind::Cauldron {
        let at = Vec3::new(shape.depth(0.62), STOVE_TOP, c / 2.0);
        return Some((at, c * 0.5));
    }
    let rest: Vec<ItemKind> = (craft.tools().into_iter())
        .filter(|&tool| tool != ItemKind::Cauldron)
        .collect();
    let index = rest.iter().position(|&tool| tool == kind).unwrap_or(place);
    let counter = (shape.length - INSET - c).max(c * 0.5);
    let room = counter / rest.len() as f32;
    let along = c + room * (index as f32 + 0.5);
    // Staggered back and front, so neighbours don't touch.
    let t = if index % 2 == 0 { 0.4 } else { 0.7 };
    Some((
        Vec3::new(shape.depth(t), STOVE_TOP, along),
        (room * 1.6).min(c * 0.45),
    ))
}

/// The middle of a hearth's firebox in the world (on flat floor), where its
/// flames stand, or `None` for anything else.
pub(in crate::dungeon_map) fn firebox(piece: &Piece) -> Option<Vec3> {
    let Furniture::CraftingBench { craft } = piece.furniture else {
        return None;
    };
    if !craft.is_hearth() {
        return None;
    }
    let shape = Shape::of(piece);
    let middle = Vec3::new(shape.depth(0.62), PLINTH, CELL_SIZE / 2.0);
    Some(Frame::of(piece).place(middle))
}

#[cfg(test)]
mod tests;
