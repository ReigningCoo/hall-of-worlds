//! The carried ointment and water stacks Alchemy's panel can offer (they were
//! Brew's, brewing plan PR 7, before the Alchemy plan replaced it), and a
//! bottle given a potion made in it, or emptied when none forms.

use super::quality;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Contents, Liquid, Water};

/// A carried ointment stack the Alchemy panel can offer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OintmentStack {
    pub stack: Id,
    pub contents: Contents,
    pub quality: Quality,
    pub count: u32,
}

/// A carried stack of water bottles the Alchemy panel can offer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaterStack {
    pub stack: Id,
    pub water: Water,
    pub count: u32,
}

/// A bottle (of water) given the potion brewed in it, keeping its look, and
/// the potion's quality; empty when no potion formed.
pub fn brewed_into(mut bottle: Item, potion: Option<Contents>) -> Item {
    let look = bottle.bottle().map_or(0, |bottle| bottle.look);
    bottle.quality = potion.as_ref().map(quality);
    let liquid = potion.map(Liquid::Potion);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, look)));
    bottle
}

/// The water in a bottle, if that is what it holds.
fn water_in(item: &Item) -> Option<Water> {
    match item.bottle().and_then(|bottle| bottle.liquid.as_ref()) {
        Some(Liquid::Water(water)) => Some(*water),
        _ => None,
    }
}

impl Inventory {
    /// Every carried ointment stack, not worn, best quality first.
    pub fn ointment_stacks(&self) -> Vec<OintmentStack> {
        let stacks = self.stacks_where(|top| top.kind == ItemKind::Ointment);
        let mut found: Vec<OintmentStack> = stacks
            .filter(|(stack, _)| !self.is_worn(stack.id))
            .filter_map(|(stack, _)| {
                Some(OintmentStack {
                    stack: stack.id,
                    contents: stack.top().ointment()?.contents.clone(),
                    quality: stack.top().quality.unwrap_or(Quality::Common),
                    count: stack.count(),
                })
            })
            .collect();
        found.sort_by_key(|stack| std::cmp::Reverse(stack.quality));
        found
    }

    /// Every carried stack of water bottles, not worn.
    pub fn water_stacks(&self) -> Vec<WaterStack> {
        let stacks = self.stacks_where(|top| water_in(top).is_some());
        let mut found: Vec<WaterStack> = stacks
            .filter(|(stack, _)| !self.is_worn(stack.id))
            .filter_map(|(stack, _)| {
                Some(WaterStack {
                    stack: stack.id,
                    water: water_in(stack.top())?,
                    count: stack.count(),
                })
            })
            .collect();
        found.sort_by_key(|stack| stack.water);
        found
    }
}
