//! Wine and spirit racks (`Furniture::BottleRack`, tavern furniture plan PR
//! 2). A wine rack stands full height against its wall: a plinth, two side
//! panels, a back and a top, with diagonal slats making a diamond lattice
//! whose every diamond holds one bottle lying down, its base to the room. A
//! spirit rack stands on the table under it: three solid steps rising from
//! its front to its back between two cheeks, three upright bottles a cell on
//! each step. Where each bottle is (`bottle_spot`) is drawn by
//! `rack_bottles.rs`. Sizes are first guesses for the owner to tune.

use bevy::prelude::*;
use shared::bottle_rack::{BottleRack, BottleRackKind, SPIRIT_SHELVES, WINE_ROWS};
use shared::door::Side;
use shared::furniture::{Furniture, Piece};

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::tables::Table;
use super::{Grain, INSET, Part, TABLE_TOP};

/// A wine rack: its plinth, where its top board sits, and how thick its
/// boards and slats are.
const PLINTH: f32 = CELL_SIZE * 0.05;
const WINE_TOP: f32 = CELL_SIZE * 0.92;
const BOARD: f32 = CELL_SIZE * 0.03;
const SLAT: f32 = CELL_SIZE * 0.015;
/// How much each of a spirit rack's steps rises, and its cheeks' thickness.
const STEP: f32 = CELL_SIZE * 0.08;
const CHEEK: f32 = CELL_SIZE * 0.03;
/// The tallest bottle's height (a wine bottle's), for picking.
pub(in crate::dungeon_map) const TALLEST: f32 = CELL_SIZE * 0.3;
/// Twelfths of a cell: how `BottlePlace::along` is counted.
const TWELFTH: f32 = CELL_SIZE / 12.0;

/// How tall a wine rack's lattice rows are.
fn row_height() -> f32 {
    (WINE_TOP - PLINTH) / f32::from(WINE_ROWS + 1)
}

/// A rack's frame `x` a share `t` of the way from its back (by the wall,
/// 0) to its front (1), within `depth`.
pub(super) fn from_back(piece: &Piece, depth: f32, t: f32) -> f32 {
    let x = INSET + (depth - 2.0 * INSET) * t;
    match piece.facing {
        // Its frame runs east or south from the back.
        Side::East | Side::South => x,
        Side::West | Side::North => depth - x,
    }
}

/// The rack `piece` is, if it is one.
fn rack_of(piece: &Piece) -> Option<BottleRack> {
    match piece.furniture {
        Furniture::BottleRack { rack } => Some(rack),
        _ => None,
    }
}

/// How high a bottle rack and its bottles reach above the floor, for
/// picking; `None` for anything else.
pub(in crate::dungeon_map) fn bottle_rack_height(piece: &Piece) -> Option<f32> {
    Some(match rack_of(piece)?.kind {
        BottleRackKind::Wine => WINE_TOP + BOARD,
        BottleRackKind::Spirit => TABLE_TOP + STEP * f32::from(SPIRIT_SHELVES) + TALLEST,
    })
}

/// Where bottle `index` of `piece`'s places is in the world (on the level's
/// flat floor: the caller raises it with the ground), and the turn that puts
/// an upright bottle (its middle at the origin) there: lying with its base to
/// the room on a wine rack, standing on a spirit rack's step. `None` for
/// anything else. `half` is the bottle's half height.
pub(in crate::dungeon_map) fn bottle_spot(
    piece: &Piece,
    index: usize,
    half: f32,
) -> Option<(Vec3, Quat)> {
    let rack = rack_of(piece)?;
    let place = *rack.places().get(index)?;
    let frame = Frame::of(piece);
    let depth = Table::of(piece).length;
    let along = f32::from(place.along) * TWELFTH;
    let (middle, axis) = match rack.kind {
        BottleRackKind::Wine => {
            let y = PLINTH + f32::from(place.row + 1) * row_height();
            let x = from_back(piece, depth, 0.5);
            // Its neck towards the wall.
            let back = from_back(piece, depth, 0.0) - x;
            (Vec3::new(x, y, along), Vec3::new(back.signum(), 0.0, 0.0))
        }
        BottleRackKind::Spirit => {
            let shelves = f32::from(SPIRIT_SHELVES);
            let band = shelves - 1.0 - f32::from(place.row);
            let x = from_back(piece, depth, (band + 0.5) / shelves);
            let y = TABLE_TOP + STEP * f32::from(place.row + 1) + half;
            (Vec3::new(x, y, along), Vec3::Y)
        }
    };
    let turn = Quat::from_rotation_arc(Vec3::Y, frame.turn(axis));
    Some((frame.place(middle), turn))
}

/// A bottle rack's parts, or `None` for anything else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let rack = rack_of(piece)?;
    let table = Table::of(piece);
    let (depth, length) = (table.length, table.width);
    let mut parts = match rack.kind {
        BottleRackKind::Wine => wine_parts(piece, rack, (depth, length)),
        BottleRackKind::Spirit => spirit_parts(piece, depth, length),
    };
    for (choice, part) in parts.iter_mut().enumerate() {
        *part = table.vary(*part, choice as i64);
    }
    Some(parts)
}

/// A wine rack's plinth, sides, back, top and lattice.
fn wine_parts(piece: &Piece, rack: BottleRack, (depth, length): (f32, f32)) -> Vec<Part> {
    let (front, back) = (from_back(piece, depth, 1.0), from_back(piece, depth, 0.0));
    let (near, far) = (front.min(back), front.max(back));
    let mut parts = vec![
        Part::block(
            Vec3::new(near, 0.0, INSET),
            Vec3::new(far, PLINTH, length - INSET),
            Grain::Across,
        ),
        Part::block(
            Vec3::new(near, WINE_TOP, INSET),
            Vec3::new(far, WINE_TOP + BOARD, length - INSET),
            Grain::Across,
        ),
        Part::block(
            Vec3::new(back - BOARD / 2.0, PLINTH, INSET),
            Vec3::new(back + BOARD / 2.0, WINE_TOP, length - INSET),
            Grain::Up,
        ),
    ];
    for z in [INSET, length - INSET - BOARD] {
        parts.push(Part::block(
            Vec3::new(near, 0.0, z),
            Vec3::new(far, WINE_TOP, z + BOARD),
            Grain::Up,
        ));
    }
    // Slats along the diamonds' edges, a row's height at a time: rising one
    // way where `z − 2 t` and the other where `z + 2 t` is 2 more than a
    // multiple of 4 (in twelfths, `t` in rows from the lattice's foot).
    let h = row_height();
    let most = 12 * i32::from(rack.length);
    for band in 0..=i32::from(WINE_ROWS) {
        let foot = PLINTH + band as f32 * h;
        for (first, rise) in [
            ((2 + 2 * band).rem_euclid(4), 2),
            ((2 - 2 * band).rem_euclid(4), -2),
        ] {
            for z in (first..=most).step_by(4) {
                if !(0..=most).contains(&(z + rise)) {
                    continue;
                }
                let z = z as f32 * TWELFTH;
                let slat = Part::block(
                    Vec3::new(near, foot, z - SLAT / 2.0),
                    Vec3::new(far, foot + h, z + SLAT / 2.0),
                    Grain::Up,
                );
                parts.push(slat.leaning(Vec2::new(0.0, rise as f32 * TWELFTH)));
            }
        }
    }
    parts
}

/// A spirit rack's steps and cheeks, on the table under it.
fn spirit_parts(piece: &Piece, depth: f32, length: f32) -> Vec<Part> {
    let shelves = f32::from(SPIRIT_SHELVES);
    let mut parts = Vec::new();
    for row in 0..SPIRIT_SHELVES {
        let band = shelves - 1.0 - f32::from(row);
        let a = from_back(piece, depth, band / shelves);
        let b = from_back(piece, depth, (band + 1.0) / shelves);
        parts.push(Part::block(
            Vec3::new(a.min(b), TABLE_TOP, INSET + CHEEK),
            Vec3::new(
                a.max(b),
                TABLE_TOP + STEP * f32::from(row + 1),
                length - INSET - CHEEK,
            ),
            Grain::Across,
        ));
    }
    let (front, back) = (from_back(piece, depth, 1.0), from_back(piece, depth, 0.0));
    for z in [INSET, length - INSET - CHEEK] {
        parts.push(Part::block(
            Vec3::new(front.min(back), TABLE_TOP, z),
            Vec3::new(
                front.max(back),
                TABLE_TOP + STEP * (shelves + 0.5),
                z + CHEEK,
            ),
            Grain::Up,
        ));
    }
    parts
}

#[cfg(test)]
mod tests;
