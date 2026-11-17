//! Carried items by kind, for a kind used from the action bar: how many there
//! are, and which stack one is taken from.

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Placed};

impl Inventory {
    /// How many of this kind are carried, in the character's own grid and in
    /// every bag, worn ones too, however deep; never a quest item, nor the bin.
    pub fn count_of(&self, kind: ItemKind) -> u32 {
        self.stacks_of(kind).map(|(stack, _)| stack.count()).sum()
    }

    /// The stack one of this kind is used from (owner, 2026-11-15): the
    /// smallest in the character's own grid, or if there is none there, the
    /// smallest in a bag. Never a worn item, nor a quest item.
    pub fn stack_to_use(&self, kind: ItemKind) -> Option<Id> {
        let smallest = |own: bool| {
            self.stacks_of(kind)
                .filter(|&(_, in_own)| in_own == own)
                .min_by_key(|(stack, _)| stack.count())
                .map(|(stack, _)| stack.id)
        };
        smallest(true).or_else(|| smallest(false))
    }

    /// Every carried stack of this kind (by its top), and whether it is in the
    /// character's own grid rather than in a bag.
    fn stacks_of(&self, kind: ItemKind) -> impl Iterator<Item = (&Item, bool)> {
        let mut found = Vec::new();
        fn search<'a>(placed: &'a [Placed], own: bool, found: &mut Vec<(&'a Item, bool)>) {
            for placed in placed {
                found.push((&placed.item, own));
                search(&placed.item.contents, false, found);
            }
        }
        search(&self.items, true, &mut found);
        for worn in self.worn.iter().chain(&self.bag_slots) {
            search(&worn.contents, false, &mut found);
        }
        found
            .into_iter()
            .filter(move |(stack, _)| stack.top().kind == kind && !stack.holds_quest_item())
    }
}
