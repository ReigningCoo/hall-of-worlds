//! Tying bags to the belt and untying them (owner, 2026-11-15): only an empty bag
//! that can go on a belt is tied; one on a filled slot swaps, the old one coming
//! out empty. A bag comes off only if the cells it adds are empty at the end of the
//! own grid (any cells there, not its own), and a belt only if it would leave as
//! many slots as there are bags. A holder (owner, 2026-11-17: a keyring) is tied
//! and untied with what is in it, and only one of each kind is tied at a time.

use crate::id::Id;
use crate::item::{Holder, Inventory, Item, ItemKind, MoveRefused, Placed, Rotation};

impl Inventory {
    /// Whether this carried item may leave where it is, as far as bag slots go: a
    /// tied bag only with room to spare at the grid's end, the worn belt only if
    /// what would be left takes every tied bag. Anything else may.
    pub fn check_lift(&self, id: Id) -> Result<(), MoveRefused> {
        if self.is_tied(id) {
            return self.clone().untie(id).map(|_| ());
        }
        let belt = self
            .worn
            .iter()
            .any(|item| item.id == id && item.kind == ItemKind::Belt);
        if belt {
            let mut after = self.clone();
            after.worn.retain(|item| item.id != id);
            if after.bag_slots.len() > after.bag_slot_count() {
                return Err(MoveRefused::BeltInUse);
            }
        }
        Ok(())
    }

    /// Takes a tied bag off its slot, if the grid can shrink by its cells.
    /// Leaves `self` part done when refused: callers work on a copy.
    pub(super) fn untie(&mut self, id: Id) -> Result<Item, MoveRefused> {
        let at = self.bag_slots.iter().position(|bag| bag.id == id);
        let bag = self.bag_slots.remove(at.ok_or(MoveRefused::NoSuchItem)?);
        match self.all_on_cells() {
            true => Ok(bag),
            false => Err(MoveRefused::BagInUse),
        }
    }

    /// Whether this carried bag could be tied to this slot (counting from 0).
    pub fn check_tie(&self, id: Id, slot: u8) -> Result<(), MoveRefused> {
        self.clone().tie_bag(id, slot)
    }

    /// Ties this carried bag to this slot: after the last bag if that slot is
    /// empty, or in place of the one there, which goes in the own grid's first
    /// room, empty. A tied bag moves to the slot, the others keeping their order.
    pub fn tie_bag(&mut self, id: Id, slot: u8) -> Result<(), MoveRefused> {
        let item = self.find(id).ok_or(MoveRefused::NoSuchItem)?;
        let container = item.kind.def().container;
        if !container.is_some_and(|inside| inside.on_belt) || self.is_worn(id) {
            return Err(MoveRefused::NotForBelt);
        }
        let slot = slot as usize;
        if slot >= self.bag_slot_count() {
            return Err(MoveRefused::NoSlot);
        }
        if let Some(from) = self.bag_slots.iter().position(|bag| bag.id == id) {
            let bag = self.bag_slots.remove(from);
            let to = slot.min(self.bag_slots.len());
            self.bag_slots.insert(to, bag);
            return Ok(());
        }
        let holder = container.is_some_and(|inside| inside.holder);
        if (!holder && !item.contents.is_empty()) || item.count() > 1 {
            return Err(MoveRefused::NotEmpty);
        }
        let kind = item.kind;
        let mut after = self.clone();
        let bag = after.take(id).ok_or(MoveRefused::NoSuchItem)?;
        if slot < after.bag_slots.len() {
            let old = std::mem::replace(&mut after.bag_slots[slot], bag);
            if !after.all_on_cells() {
                return Err(MoveRefused::BagInUse);
            }
            let room = after.first_room(Holder::Inventory, old.shape(), Rotation::None, None);
            let (x, y, rotation) = room.ok_or(MoveRefused::NoRoom)?;
            after.items.push(Placed {
                item: old,
                x,
                y,
                rotation,
            });
        } else {
            after.bag_slots.push(bag);
        }
        if holder
            && after
                .bag_slots
                .iter()
                .filter(|bag| bag.kind == kind)
                .count()
                > 1
        {
            return Err(MoveRefused::HolderTied);
        }
        *self = after;
        Ok(())
    }
}
