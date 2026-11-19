//! The carried herb and oil stacks Alchemy's panel can offer for a Mortar
//! (they were Grind's, brewing plan PR 6, before the Alchemy plan replaced it).

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::potion::{Liquid, Oil};

/// A carried stack the Alchemy panel can offer as a herb.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HerbStack {
    pub stack: Id,
    pub herb: String,
    pub quality: Quality,
    pub count: u32,
}

/// A carried stack of oil bottles the Alchemy panel can offer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OilStack {
    pub stack: Id,
    pub oil: Oil,
    pub count: u32,
}

/// Whether a carried stack may be used up: not worn (a herb pouch's
/// contents are carried, not worn).
fn usable(carried: &Inventory, item: &Item) -> bool {
    !carried.is_worn(item.id)
}

impl Inventory {
    /// Every carried herb stack, by herb key then best quality first.
    pub fn herb_stacks(&self) -> Vec<HerbStack> {
        let herbs = self.stacks_where(|top| top.kind == ItemKind::Herb);
        let mut found: Vec<HerbStack> = herbs
            .filter(|(stack, _)| usable(self, stack))
            .filter_map(|(stack, _)| {
                Some(HerbStack {
                    stack: stack.id,
                    herb: stack.top().herb()?.herb.clone(),
                    quality: stack.top().quality.unwrap_or(Quality::Common),
                    count: stack.count(),
                })
            })
            .collect();
        found.sort_by(|a, b| (&a.herb, b.quality).cmp(&(&b.herb, a.quality)));
        found
    }

    /// Every carried stack of oil bottles.
    pub fn oil_stacks(&self) -> Vec<OilStack> {
        let oil = |item: &Item| match item.bottle().and_then(|b| b.liquid.as_ref()) {
            Some(Liquid::Oil(oil)) => Some(*oil),
            _ => None,
        };
        let stacks = self.stacks_where(|top| oil(top).is_some());
        let stacks = stacks.filter(|(stack, _)| usable(self, stack));
        let mut found: Vec<OilStack> = stacks
            .filter_map(|(stack, _)| {
                Some(OilStack {
                    stack: stack.id,
                    oil: oil(stack.top())?,
                    count: stack.count(),
                })
            })
            .collect();
        found.sort_by_key(|stack| stack.oil);
        found
    }
}
