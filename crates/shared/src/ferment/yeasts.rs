//! The yeasts Ferment pitches (owner: each yeast is used once between feeds,
//! not used up; yeasts stack as separate items, so several go into a large
//! cask): the usable ones of a carried stack, top first, and marking them
//! used; and taking the fruit or grain mashed in.

use crate::id::Id;
use crate::item::{Inventory, Item, Quality};

/// The qualities of up to `count` usable yeasts in `stack`, top first.
pub(super) fn usable(stack: &Item, count: u32) -> Vec<Quality> {
    (stack.members().rev())
        .filter(|one| one.yeast().is_some_and(|yeast| yeast.usable()))
        .take(count as usize)
        .map(|one| one.quality.unwrap_or_default())
        .collect()
}

impl Inventory {
    /// Takes the whole carried stack `stack`, the fruit or grain Ferment mashes
    /// in: used up.
    pub fn take_added(&mut self, stack: Id) -> Option<Item> {
        self.take(stack)
    }

    /// The qualities of up to `count` usable yeasts in the carried stack
    /// `stack` (not worn), top first: what Ferment would pitch.
    pub fn usable_yeasts(&self, stack: Id, count: u32) -> Vec<Quality> {
        match self.find(stack).filter(|_| !self.is_worn(stack)) {
            Some(found) => usable(found, count),
            None => Vec::new(),
        }
    }

    /// Marks up to `count` usable yeasts in the stack `stack` used, top
    /// first: how many were.
    pub fn use_yeasts(&mut self, stack: Id, count: u32) -> u32 {
        let Some(found) = self.find_mut(stack) else {
            return 0;
        };
        let mut used = 0;
        for one in found.above.iter_mut().rev() {
            if used < count && mark(one) {
                used += 1;
            }
        }
        if used < count && mark(found) {
            used += 1;
        }
        used
    }
}

/// Marks one yeast used if it is usable.
fn mark(one: &mut Item) -> bool {
    match one.yeast_mut() {
        Some(yeast) if yeast.usable() => {
            yeast.used = true;
            true
        }
        _ => false,
    }
}
