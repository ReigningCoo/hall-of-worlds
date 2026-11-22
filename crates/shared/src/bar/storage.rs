//! Which drinks a bar pours from (owner's): those stored on pieces with
//! places in the bar's own room (cask racks, cradles, Tun stands, wine and
//! spirit racks, and bar counters' places), never anything lying on the
//! ground, nor a crafting bench's storage.

use std::collections::BTreeSet;

use crate::dungeon::Level;
use crate::furniture::{Held, Holder, Piece};
use crate::item::{Inventory, Item, ItemKind};
use crate::pour;

/// Every piece of drink storage (`Holder::is_drink_storage`) on `level`
/// sharing a room with `bar` (a cell of each in one room), the bar among
/// them. Cells in no room count as one room of their own (Claude's choice:
/// a bar on open ground, as in a Dev Area, pours from storage also outside
/// rooms). Floor pieces first, then those standing on furniture, each by its
/// north-west cell.
pub fn room_storage(level: &Level, bar: &Piece) -> Vec<Piece> {
    let rooms: BTreeSet<_> = (bar.cells()).map(|(x, y)| level.room(x, y)).collect();
    let shares_room = |piece: &Piece| {
        piece.at == bar.at || (piece.cells()).any(|(x, y)| rooms.contains(&level.room(x, y)))
    };
    let stores = |piece: &&Piece| Holder::of(piece.furniture).is_some_and(Holder::is_drink_storage);
    let floor = level.furniture().filter(stores);
    let on_top = level.furniture_on_top().filter(stores);
    floor
        .chain(on_top)
        .filter(|piece| shares_room(piece))
        .copied()
        .collect()
}

/// What on `held` can be poured from: each cask, vessel or bottle with
/// something in it, in its places' order.
pub fn drinks_in(held: &Held) -> impl Iterator<Item = &Item> {
    (held.items()).filter(|item| pour::Held::of(item).is_some_and(|inside| inside.ml > 0))
}

/// The carried vessels a bar fills (any kind, in the character's own grid or
/// a bag, never one worn nor a quest item), in the inventory's order.
pub fn vessels_to_fill(inventory: &Inventory) -> Vec<&Item> {
    (inventory.stacks_where(|top| top.kind == ItemKind::Vessel))
        .map(|(stack, _)| stack)
        .collect()
}
