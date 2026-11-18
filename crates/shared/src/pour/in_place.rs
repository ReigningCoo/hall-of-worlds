//! Changing a carried container where it lies, once poured: its shape never
//! changes with what is in it, so it stays in place.

use crate::item::{Inventory, Item};

impl Inventory {
    /// Puts `item` in place of the carried one with its id, if that is alone
    /// (not a stack) and of the same kind: whether it was.
    pub fn replace_carried(&mut self, item: Item) -> bool {
        let Some(carried) = self.find_mut(item.id) else {
            return false;
        };
        if carried.count() != 1 || carried.kind != item.kind {
            return false;
        }
        *carried = item;
        true
    }
}
