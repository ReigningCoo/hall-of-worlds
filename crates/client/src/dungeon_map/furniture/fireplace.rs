//! Fireplaces (`Furniture::Fireplace`, tavern furniture plan PR 4): stone, on
//! a hearthstone, against the wall. The fire's cell has jambs and a back
//! round its opening under a mantel shelf, and a chimney breast rising behind
//! to the top of the wall (nothing hangs from the ceiling). An inglenook's
//! opening spans all three cells under one long lintel, a settle either side
//! of the fire inside it. Fire tools (a stand with a poker, tongs and a
//! shovel, and bellows leaning by the fire, dark as iron) and a log basket
//! are part of the model; the kindling in the basket's places is drawn by
//! `basket_logs`, flames in a lit one by `hearth_fires`.

use bevy::prelude::*;
use shared::fireplace::Fireplace;
use shared::furniture::{Furniture, Piece};

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::hearth::Shape;
use super::tables::Table;
use super::{INSET, Part};

const C: f32 = CELL_SIZE;
/// The hearthstone everything stands on.
const HEARTHSTONE: f32 = C * 0.05;
/// How wide a jamb is, and how far out the opening's back stands (a share of
/// the depth, from the wall).
const JAMB: f32 = C * 0.16;
const BACK: f32 = 0.3;
/// The top of the opening (an inglenook's higher), and the mantel over it.
const LINTEL: f32 = C * 0.55;
const INGLENOOK_LINTEL: f32 = C * 0.8;
const MANTEL: f32 = C * 0.07;
/// How far the chimney breast stands out from the wall (a share of the
/// depth), and its top: the wall's.
const BREAST: f32 = 0.45;
const BREAST_TOP: f32 = C;
/// Iron's shade against the stone, the bellows', the basket's and the
/// settles'.
const IRON: f32 = 0.3;
const BELLOWS: f32 = 0.55;
const WICKER: f32 = 0.75;
const SETTLE: f32 = 0.85;

/// Where a fireplace's log basket stands in the piece's frame: `t` from the
/// back to the front, `z` along the wall.
struct Basket {
    t: (f32, f32),
    z: (f32, f32),
}

impl Basket {
    /// In the front corner of a fireplace 1 long, in the second cell of one
    /// 2 long, in front of an inglenook's fire to one side.
    fn of(fireplace: Fireplace, length: f32) -> Self {
        match fireplace.length {
            1 => Self {
                t: (0.72, 0.97),
                z: (C * 0.64, C - INSET),
            },
            2 => Self {
                t: (0.35, 0.85),
                z: (C * 1.25, length - INSET),
            },
            _ => Self {
                t: (0.76, 0.98),
                z: (C * 1.66, C * 2.18),
            },
        }
    }
}

/// A fireplace's parts, or `None` for anything else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let Furniture::Fireplace { fireplace } = piece.furniture else {
        return None;
    };
    let shape = Shape::of(piece);
    let length = shape.length;
    let mut parts = vec![shape.block((0.0, 1.0), (0.0, HEARTHSTONE), (0.0, length))];
    let fire = f32::from(fireplace.fire_cell()) * C;
    // The opening's span along the wall: one cell, or the whole inglenook.
    let (from, to, top) = match fireplace.is_inglenook() {
        true => (0.0, length, INGLENOOK_LINTEL),
        false => (fire, fire + C, LINTEL),
    };
    let (jambs, low) = ((0.0, BREAST + 0.15), (HEARTHSTONE, top));
    parts.push(shape.block(jambs, low, (from, from + JAMB)));
    parts.push(shape.block(jambs, low, (to - JAMB, to)));
    parts.push(shape.block((0.0, BACK), low, (from + JAMB, to - JAMB)));
    // The mantel shelf, standing out a little, and the breast over it.
    parts.push(shape.block((0.0, BREAST + 0.22), (top, top + MANTEL), (from, to)));
    let (inset, breast) = (C * 0.04, (top + MANTEL, BREAST_TOP));
    parts.push(shape.block((0.0, BREAST), breast, (from + inset, to - inset)));
    if fireplace.is_inglenook() {
        settles(&shape, length, &mut parts);
    }
    tools(&shape, fireplace, &mut parts);
    basket(&shape, &Basket::of(fireplace, length), &mut parts);
    let table = Table::of(piece);
    let varied = parts.into_iter().enumerate().map(|(i, part)| {
        // Stone varies from block to block; iron and wood keep their shade.
        let shade = part.shade;
        let part = table.vary(part, 80 + i as i64);
        Part {
            shade: part.shade * shade,
            ..part
        }
    });
    Some(varied.collect())
}

/// An inglenook's settles inside it, either side of the fire, at right angles
/// to it (owner's): each runs from the back to the front of the opening, its
/// high back against the jamb, its sitters facing the fire.
fn settles(shape: &Shape, length: f32, parts: &mut Vec<Part>) {
    let (seat, depth) = ((C * 0.26, C * 0.31), (BACK + 0.02, 0.95));
    let (board, wide) = (C * 0.08, C * 0.3);
    for near in [true, false] {
        let (back, seat_at) = match near {
            true => ((JAMB, JAMB + board), (JAMB + board, JAMB + board + wide)),
            false => (
                (length - JAMB - board, length - JAMB),
                (length - JAMB - board - wide, length - JAMB - board),
            ),
        };
        parts.push(shape.block(depth, seat, seat_at).varied(SETTLE, false));
        let high = (HEARTHSTONE, C * 0.62);
        parts.push(shape.block(depth, high, back).varied(SETTLE, true));
    }
}

/// Fire tools on the hearthstone: a stand with a poker, tongs and a shovel
/// hung round its post, dark as iron, and bellows leaning by the fire.
fn tools(shape: &Shape, fireplace: Fireplace, parts: &mut Vec<Part>) {
    // A corner clear of the basket: behind it in the second cell of a
    // fireplace 2 long, in front of an inglenook's fire to the other side,
    // else the front of the near end.
    let (z, (t0, t1)) = match fireplace.length {
        1 => (C * 0.1, (0.78, 0.96)),
        2 => (C * 1.12, (0.08, 0.28)),
        _ => (C * 1.05, (0.78, 0.96)),
    };
    let rod = C * 0.018;
    let iron = |part: Part| part.varied(IRON, false);
    let foot = (HEARTHSTONE, HEARTHSTONE + C * 0.02);
    parts.push(iron(shape.block((t0, t1), foot, (z, z + C * 0.15))));
    let post = z + C * 0.065;
    let stand = shape.block(
        (t0 + 0.08, t0 + 0.12),
        (HEARTHSTONE, C * 0.5),
        (post, post + rod),
    );
    parts.push(iron(stand));
    // Poker, tongs (two legs) and shovel (with its blade), hung round it.
    let hang = (HEARTHSTONE + C * 0.04, C * 0.46);
    let alongs = [z, z + C * 0.03, z + C * 0.1, z + C * 0.13];
    for (i, along) in alongs.into_iter().enumerate() {
        parts.push(iron(shape.block(
            (t0 + 0.02, t0 + 0.05),
            hang,
            (along, along + rod),
        )));
        if i == 3 {
            let blade = (hang.0, hang.0 + C * 0.07);
            let wide = (along - C * 0.015, along + C * 0.035);
            parts.push(iron(shape.block((t0 + 0.01, t0 + 0.08), blade, wide)));
        }
    }
    // Bellows standing on end by the fire's far jamb, leaning on it (in an
    // inglenook, on the far settle's end).
    let fire_end = (f32::from(fireplace.fire_cell()) + 1.0) * C;
    let jamb = match fireplace.is_inglenook() {
        true => fire_end + C * 0.36,
        false => fire_end - JAMB,
    };
    let bellows = shape.block((0.55, 0.7), (HEARTHSTONE, C * 0.3), (jamb - C * 0.05, jamb));
    let lean = bellows.leaning(Vec2::new(0.0, C * 0.03));
    parts.push(lean.varied(BELLOWS, false));
}

/// The log basket: a floor and four low sides.
fn basket(shape: &Shape, basket: &Basket, parts: &mut Vec<Part>) {
    let ((t0, t1), (z0, z1)) = (basket.t, basket.z);
    let (wall, high) = (C * 0.02, (HEARTHSTONE, HEARTHSTONE + C * 0.09));
    let side = 0.05;
    let wicker = |part: Part| part.varied(WICKER, false);
    let bottom = (HEARTHSTONE, HEARTHSTONE + wall);
    parts.push(wicker(shape.block((t0, t1), bottom, (z0, z1))));
    parts.push(wicker(shape.block((t0, t1), high, (z0, z0 + wall))));
    parts.push(wicker(shape.block((t0, t1), high, (z1 - wall, z1))));
    parts.push(wicker(shape.block((t0, t0 + side), high, (z0, z1))));
    parts.push(wicker(shape.block((t1 - side, t1), high, (z0, z1))));
}

/// Where the fire burns, in the world (on flat floor): its cell's middle
/// along the wall, inside the opening.
pub(super) fn fire_spot(piece: &Piece, fireplace: Fireplace) -> Vec3 {
    let shape = Shape::of(piece);
    let along = (f32::from(fireplace.fire_cell()) + 0.5) * C;
    Frame::of(piece).place(Vec3::new(shape.depth(0.55), HEARTHSTONE, along))
}

/// Where the kindling in basket place `index` of `piece` lies, in the world
/// (on flat floor), and the way its logs run (front to back, level); `None`
/// for anything else or past its places.
pub(in crate::dungeon_map) fn basket_spot(piece: &Piece, index: usize) -> Option<(Vec3, Vec3)> {
    let Furniture::Fireplace { fireplace } = piece.furniture else {
        return None;
    };
    let places = fireplace.basket_places();
    if index >= places {
        return None;
    }
    let shape = Shape::of(piece);
    let basket = Basket::of(fireplace, shape.length);
    let (z0, z1) = basket.z;
    let along = z0 + (z1 - z0) * (index as f32 + 0.5) / places as f32;
    let middle = (basket.t.0 + basket.t.1) / 2.0;
    let frame = Frame::of(piece);
    let lie = HEARTHSTONE + C * 0.07;
    let at = frame.place(Vec3::new(shape.depth(middle), lie, along));
    let front = frame.place(Vec3::new(shape.depth(1.0), lie, along));
    Some((at, (front - at).normalize_or_zero()))
}

#[cfg(test)]
mod tests;
