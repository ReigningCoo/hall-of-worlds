//! Where on a level a bench could stand: room floor along a plain wall, its
//! back to it, with floor in front to stand at it, in hashed order.

use std::collections::BTreeSet;

use super::Craft;
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::place::rooms::Room;
use crate::furniture::{Furniture, Material, Piece, Stone, Wood, footprint};
use crate::rng::Seed;

/// Each side a wall can be behind a bench: where the wall is from it, and
/// the way its front then faces (as a seat's).
const BACKS: [((i64, i64), Side); 4] = [
    ((0, -1), Side::South),
    ((1, 0), Side::West),
    ((0, 1), Side::North),
    ((-1, 0), Side::East),
];

/// Purpose numbers for the hashes.
const ORDER: i64 = 0x6f72;
const WOOD: i64 = 0x776f;

/// Every bench of `craft` that could stand in the level's rooms, by `hash`:
/// each cell usable, unfurnished and with a plain `Wall` behind it, and the
/// cell in front of each open floor, unfurnished.
pub(super) fn spots(
    level: &Level,
    rooms: &[Room],
    usable: &BTreeSet<(i64, i64)>,
    craft: Craft,
    hash: impl Fn(&[i64]) -> u64,
) -> Vec<Piece> {
    let length = i64::from(craft.length());
    let furniture = Furniture::CraftingBench { craft };
    let free = |(x, y): (i64, i64)| level.furniture_at(x, y).is_none();
    let mut found: Vec<(u64, Piece)> = Vec::new();
    for &(x, y) in rooms.iter().flat_map(|room| room.cells.iter()) {
        for (back, &((dx, dy), facing)) in BACKS.iter().enumerate() {
            // Along the wall, east or south.
            let (ax, ay) = (dy.abs(), dx.abs());
            let cells = (0..length).map(|i| (x + ax * i, y + ay * i));
            let fits = cells.clone().all(|(cx, cy)| {
                usable.contains(&(cx, cy))
                    && free((cx, cy))
                    && level.cell(cx + dx, cy + dy) == Cell::Wall
                    && level.cell(cx - dx, cy - dy) == Cell::Floor
                    && free((cx - dx, cy - dy))
            });
            if !fits {
                continue;
            }
            let mut rng = Seed::new(hash(&[x, y, WOOD])).rng();
            // Hearths are of stone, other benches of wood.
            let material = match craft.is_hearth() {
                true => Material::Stone(rng.pick_weighted(&Stone::WEIGHTS)),
                false => Material::Wood(rng.pick_weighted(&Wood::WEIGHTS)),
            };
            let piece = Piece {
                at: (x, y),
                size: footprint(furniture, (craft.length(), 1), facing),
                facing,
                material,
                furniture,
            };
            found.push((hash(&[x, y, back as i64, ORDER]), piece));
        }
    }
    found.sort_by_key(|&(tie, piece)| (tie, piece.at));
    found.into_iter().map(|(_, piece)| piece).collect()
}
