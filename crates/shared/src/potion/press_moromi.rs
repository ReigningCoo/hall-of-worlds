//! Moromi pressed with the oil press (cooking plan PR 12): each moromi is
//! pressed whole into as many bottles of soy sauce as its soybeans give
//! (`food::soy_sauce_bottles`), at its quality, so it needs that many empty
//! bottles carried. What is left of the mash is not kept.

use super::{BottleForm, Liquid};
use crate::food::soy_sauce_bottles;
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::item_form::ItemForm;

impl Inventory {
    /// The empty bottles carried.
    pub fn empty_bottle_count(&self) -> u32 {
        self.stacks_where(|item| item.bottle().is_some_and(BottleForm::is_empty))
            .map(|(stack, _)| stack.count())
            .sum()
    }

    /// The bottles of soy sauce every moromi carried that could be pressed
    /// now would give.
    pub fn soy_sauce_carried(&self) -> u32 {
        let empty = self.empty_bottle_count();
        self.stacks_where(|item| soy_sauce_bottles(item) > 0)
            .map(|(stack, _)| soy_sauce_bottles(stack.top()))
            .filter(|&bottles| bottles <= empty)
            .sum()
    }

    /// The moromi pressed next: the one giving the fewest bottles, if that
    /// many empty bottles are carried.
    pub fn moromi_to_press(&self) -> Option<Id> {
        let (stack, _) = self
            .stacks_where(|item| soy_sauce_bottles(item) > 0)
            .min_by_key(|(stack, _)| soy_sauce_bottles(stack.top()))?;
        (soy_sauce_bottles(stack.top()) <= self.empty_bottle_count()).then_some(stack.id)
    }
}

/// The bottle `item` holding `liquid`, keeping its look.
pub fn pressed_into(mut item: Item, liquid: Liquid) -> Item {
    let look = item.bottle().map_or(0, |bottle| bottle.look);
    item.form = Some(ItemForm::Bottle(BottleForm::new(Some(liquid), look)));
    item
}
