//! The carried herb and oil stacks Alchemy's panel can offer for a Mortar
//! (they were Grind's, brewing plan PR 6, before the Alchemy plan replaced it),
//! herbs including the fungi used like herbs (fungi plan PR 8).

use super::Ingredient;
use crate::fungus::Fungi;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::potion::{Liquid, Oil};

/// A carried stack the Alchemy panel can offer as a herb: a herb, or a
/// fungus with herb effects (Common, as fungi have no quality).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HerbStack {
    pub stack: Id,
    /// Which table `herb` is from.
    pub ingredient: Ingredient,
    /// The herb's (or fungus's) key.
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

/// What a Mortar can grind an item as: a herb, or a fungus with herb effects
/// (not a serving cut from one), by key.
fn ingredient_of(item: &Item) -> Option<(Ingredient, String)> {
    if let Some(herb) = item.herb() {
        return Some((Ingredient::Herb, herb.herb.clone()));
    }
    let fungus = item.fungus().filter(|fungus| !fungus.serving)?;
    let def = Fungi::built_in().get(&fungus.fungus)?;
    (!def.herb.is_empty()).then(|| (Ingredient::Fungus, fungus.fungus.clone()))
}

/// Whether a carried stack may be used up: not worn (a herb pouch's
/// contents are carried, not worn).
fn usable(carried: &Inventory, item: &Item) -> bool {
    !carried.is_worn(item.id)
}

impl Inventory {
    /// Every carried herb stack, then every stack of a fungus with herb
    /// effects (whole ones, not servings), each by key then best quality
    /// first.
    pub fn herb_stacks(&self) -> Vec<HerbStack> {
        let stacks = self.stacks_where(|top| matches!(top.kind, ItemKind::Herb | ItemKind::Fungus));
        let mut found: Vec<HerbStack> = stacks
            .filter(|(stack, _)| usable(self, stack))
            .filter_map(|(stack, _)| {
                let (ingredient, herb) = ingredient_of(stack.top())?;
                Some(HerbStack {
                    stack: stack.id,
                    ingredient,
                    herb,
                    quality: stack.top().quality.unwrap_or(Quality::Common),
                    count: stack.count(),
                })
            })
            .collect();
        found.sort_by(|a, b| {
            (a.ingredient, &a.herb, b.quality).cmp(&(b.ingredient, &b.herb, a.quality))
        });
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

    /// How many carried bottles hold `liquid`, worn or not.
    pub fn bottles_of(&self, liquid: &Liquid) -> u32 {
        let holds = |item: &Item| item.bottle().and_then(|b| b.liquid.as_ref()) == Some(liquid);
        let stacks = self.stacks_where(|top| holds(top));
        stacks.map(|(stack, _)| stack.count()).sum()
    }
}
