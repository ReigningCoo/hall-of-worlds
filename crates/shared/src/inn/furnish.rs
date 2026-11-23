//! The inn's furniture and decor (The Way Inn plan, PR 5), placed in code
//! from its fixed places (`spots`), the same for everyone: on the ground
//! floor the bar and its storage, three fireplaces, booths and private rooms,
//! about 30 scattered table groups (`tables.rs`, from a fixed seed), the
//! kitchen, brewing and ageing rooms, the treasury and the foyer; upstairs
//! every bedroom by its tier (`bedroom.rs`; own rooms too). Then decor:
//! hangings on most walls, sconces, chandeliers and rugs (`decor.rs`).
//!
//! What the racks hold, the hearths' tools and the fires are the server's
//! (`sites/inn/stock.rs`), kept full.

mod back_rooms;
mod bar;
pub(super) mod bedroom;
mod decor;
mod rows;
mod tables;

use super::{GROUND, InnRoom, UPSTAIRS};
use crate::door::Side;
use crate::dungeon::Level;
use crate::furniture::{Furniture, Material, Piece, Stone, Wood, footprint};

pub(super) const OAK: Material = Material::Wood(Wood::Oak);
pub(super) const WALNUT: Material = Material::Wood(Wood::Walnut);
pub(super) const PINE: Material = Material::Wood(Wood::Pine);
pub(super) const GRANITE: Material = Material::Stone(Stone::Granite);

/// A piece whose north-west cell is `at`, `(long, short)` as `footprint`
/// takes them.
pub(super) fn piece(
    furniture: Furniture,
    at: (i64, i64),
    (long, short): (u8, u8),
    facing: Side,
    material: Material,
) -> Piece {
    Piece {
        at,
        size: footprint(furniture, (long, short), facing),
        facing,
        material,
        furniture,
    }
}

/// Every piece on floor `index`, those on the floor before those standing
/// on furniture (`rooms` are its rooms, for the bedrooms).
pub(super) fn pieces(index: u32, rooms: &[InnRoom]) -> Vec<Piece> {
    let mut pieces = match index {
        GROUND => {
            let mut pieces = bar::pieces();
            pieces.extend(rows::pieces());
            pieces.extend(back_rooms::pieces());
            pieces.extend(tables::pieces());
            pieces
        }
        UPSTAIRS => bedroom::pieces(rooms),
        _ => Vec::new(),
    };
    pieces.sort_by_key(|piece| piece.furniture.stands_on_furniture());
    pieces
}

/// Furnishes and decorates floor `index`.
pub(super) fn furnish(level: &mut Level, index: u32, rooms: &[InnRoom]) {
    for piece in pieces(index, rooms) {
        level.place_furniture(piece);
    }
    for decor in decor::decor(level, index, rooms) {
        level.place_decor(decor);
    }
}

#[cfg(test)]
mod tests;
