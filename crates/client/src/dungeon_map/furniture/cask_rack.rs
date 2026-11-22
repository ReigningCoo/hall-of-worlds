//! Cask racks (`Furniture::CaskRack`, tavern furniture plan PR 1): a rack's
//! two low rails with posts at their ends, the casks lying across them in a
//! pyramid; a cradle's pair of saddles under each cell's cask; a Tun stand's
//! heavy blocks across its depth, the Tun lying along the wall on them. And
//! where each cask lies (`cask_spot`), drawn by `rack_casks.rs`. Casks on
//! racks and cradles lie heads to the room. A cell is taken as about a metre;
//! a rack's casks are drawn to fill its places, the rest at their size, fitted
//! in (first guesses for the owner to tune).

use bevy::prelude::*;
use shared::cask::CaskSize;
use shared::cask_rack::{CaskRack, RackKind};
use shared::furniture::{Furniture, Piece};

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::tables::Table;
use super::{Grain, INSET, Part};
use crate::item_art::{cask_height, cask_radius_share};

/// How high a rack's rails are, how thick, and its posts.
const RAIL: f32 = CELL_SIZE * 0.07;
const RAIL_WIDTH: f32 = CELL_SIZE * 0.08;
const POST: f32 = CELL_SIZE * 0.05;
/// How high a cradle's saddles are, and a stand's blocks.
const CRADLE: f32 = CELL_SIZE * 0.1;
const STAND: f32 = CELL_SIZE * 0.22;
/// How far a cask sinks into its saddles, as a share of its radius.
const SINK: f32 = 0.1;

/// World units a metre of cask of `size` is drawn at on its piece: a rack's
/// to fill its places (two a cell), the rest at a cell a metre, fitted in.
pub(in crate::dungeon_map) fn cask_scale(size: CaskSize) -> f32 {
    let (length, across) = (
        cask_height(size),
        2.0 * cask_radius_share(size) * cask_height(size),
    );
    match RackKind::of(size) {
        RackKind::Rack { .. } => CELL_SIZE * 0.48 / across,
        RackKind::Cradle => CELL_SIZE.min(CELL_SIZE * 0.9 / across.max(length)),
        RackKind::Stand => CELL_SIZE
            .min(CELL_SIZE * 0.95 / across)
            .min(CELL_SIZE * 1.9 / length),
    }
}

/// A cask's belly radius on its piece, in world units.
fn cask_radius(size: CaskSize) -> f32 {
    cask_radius_share(size) * cask_height(size) * cask_scale(size)
}

/// How high the middle of a cask in `layer` lies on a piece for `rack`.
fn cask_middle(rack: CaskRack, layer: u8) -> f32 {
    let radius = cask_radius(rack.size);
    match rack.kind() {
        RackKind::Rack { .. } => RAIL + radius + f32::from(layer) * radius * 3f32.sqrt(),
        RackKind::Cradle => CRADLE + radius * (1.0 - SINK),
        RackKind::Stand => STAND + radius * (1.0 - SINK),
    }
}

/// How high a cask rack and its casks reach, for picking; `None` for anything
/// else.
pub(in crate::dungeon_map) fn rack_height(piece: &Piece) -> Option<f32> {
    let Furniture::CaskRack { rack } = piece.furniture else {
        return None;
    };
    let top = rack
        .places()
        .iter()
        .map(|place| place.layer)
        .max()
        .unwrap_or(0);
    Some(cask_middle(rack, top) + cask_radius(rack.size))
}

/// Where cask `index` of `piece`'s places lies in the world (on the level's
/// flat floor: the caller raises it with the ground), and the turn that lays
/// an upright cask (its middle at the origin) down there; `None` for anything
/// else.
pub(in crate::dungeon_map) fn cask_spot(piece: &Piece, index: usize) -> Option<(Vec3, Quat)> {
    let Furniture::CaskRack { rack } = piece.furniture else {
        return None;
    };
    let place = *rack.places().get(index)?;
    let frame = Frame::of(piece);
    let depth = Table::of(piece).length;
    let along = f32::from(place.along) / 4.0 * CELL_SIZE;
    let middle = Vec3::new(depth / 2.0, cask_middle(rack, place.layer), along);
    // Heads to the room, or a Tun lying along the wall.
    let axis = match rack.kind() {
        RackKind::Stand => Vec3::Z,
        _ => Vec3::X,
    };
    let turn = Quat::from_rotation_arc(Vec3::Y, frame.turn(axis));
    Some((frame.place(middle), turn))
}

/// A cask rack's parts, or `None` for anything else.
pub(super) fn parts(piece: &Piece) -> Option<Vec<Part>> {
    let Furniture::CaskRack { rack } = piece.furniture else {
        return None;
    };
    let table = Table::of(piece);
    let (depth, length) = (table.length, table.width);
    let mut parts = Vec::new();
    let add = |part: Part, parts: &mut Vec<Part>| {
        let choice = parts.len() as i64;
        parts.push(table.vary(part, choice));
    };
    match rack.kind() {
        RackKind::Rack { .. } => {
            let top = rack_height(piece).unwrap_or(RAIL) * 0.75;
            for x in [depth * 0.2, depth * 0.8 - RAIL_WIDTH] {
                let rail = Part::block(
                    Vec3::new(x, 0.0, INSET),
                    Vec3::new(x + RAIL_WIDTH, RAIL, length - INSET),
                    Grain::Across,
                );
                add(rail, &mut parts);
                for z in [INSET, length - INSET - POST] {
                    let post = Part::block(
                        Vec3::new(x, RAIL, z),
                        Vec3::new(x + RAIL_WIDTH, top, z + POST),
                        Grain::Up,
                    );
                    add(post, &mut parts);
                }
            }
        }
        RackKind::Cradle => {
            for cell in 0..rack.length {
                let from = f32::from(cell) * CELL_SIZE;
                for x in [depth * 0.22, depth * 0.78 - RAIL_WIDTH] {
                    let saddle = Part::block(
                        Vec3::new(x, 0.0, from + CELL_SIZE * 0.15),
                        Vec3::new(x + RAIL_WIDTH, CRADLE, from + CELL_SIZE * 0.85),
                        Grain::Across,
                    );
                    add(saddle, &mut parts);
                }
            }
        }
        RackKind::Stand => {
            for x in [INSET, depth - INSET - RAIL_WIDTH] {
                let beam = Part::block(
                    Vec3::new(x, 0.0, INSET),
                    Vec3::new(x + RAIL_WIDTH, RAIL, length - INSET),
                    Grain::Across,
                );
                add(beam, &mut parts);
            }
            for share in [0.15, 0.5, 0.85] {
                let z = length * share - RAIL_WIDTH;
                let block = Part::block(
                    Vec3::new(INSET, 0.0, z),
                    Vec3::new(depth - INSET, STAND, z + 2.0 * RAIL_WIDTH),
                    Grain::Along,
                );
                add(block, &mut parts);
            }
        }
    }
    Some(parts)
}

#[cfg(test)]
mod tests;
