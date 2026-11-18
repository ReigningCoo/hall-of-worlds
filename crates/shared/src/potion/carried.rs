//! Carried potions, for the action bar (brewing decision 22): which bottles
//! hold a particular potion, how many there are, and which one is drunk.

use super::PotionKey;
use crate::id::Id;
use crate::item::{Inventory, Item};

impl Item {
    /// Whether it is a bottle holding a potion: something to drink.
    pub fn is_potion(&self) -> bool {
        self.bottle().is_some_and(|bottle| bottle.is_potion())
    }

    /// Its potion's key, if it is a bottle holding a potion.
    pub fn potion_key(&self) -> Option<PotionKey> {
        let bottle = self.bottle().filter(|bottle| bottle.is_potion())?;
        Some(PotionKey::of(bottle.contents()))
    }
}

impl Inventory {
    /// How many bottles of this potion are carried, bags too; never a quest item.
    pub fn count_of_potion(&self, potion: PotionKey) -> u32 {
        let stacks = self.stacks_where(|top| top.potion_key() == Some(potion));
        stacks.map(|(stack, _)| stack.count()).sum()
    }

    /// The stack a bottle of this potion is drunk from, as `stack_to_use`
    /// chooses for a kind: the smallest in the character's own grid, or if
    /// there is none there, the smallest in a bag. Never a worn item.
    pub fn potion_to_use(&self, potion: PotionKey) -> Option<Id> {
        let stacks: Vec<_> = self
            .stacks_where(|top| top.potion_key() == Some(potion))
            .collect();
        let smallest = |own: bool| {
            stacks
                .iter()
                .filter(|&&(_, in_own)| in_own == own)
                .min_by_key(|(stack, _)| stack.count())
                .map(|(stack, _)| stack.id)
        };
        smallest(true).or_else(|| smallest(false))
    }
}
