//! What Ferment's panel offers from what is carried: casks that can be set
//! fermenting, fruit and grain to mash into water, and yeast stacks with
//! their usable yeasts. Casks lying in reach are the client's to add.

use crate::fermentation::Mash;
use crate::fruit::Fruits;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::potion::{Liquid, Water};
use crate::vegetable::{VegetableSet, Vegetables};

use super::yeasts::usable;

/// A carried yeast stack and how many of its yeasts can be used now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YeastStack {
    pub stack: Id,
    /// Its top one's quality.
    pub quality: Quality,
    pub usable: u32,
}

/// What the panel lists, each in the order found (own grid first).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FermentChoices {
    /// Casks holding juice, cordial or fresh water, not fermenting.
    pub casks: Vec<Id>,
    /// Fruit and grain stacks that ferment mashed into water.
    pub adds: Vec<Id>,
    /// Yeast stacks with a usable yeast.
    pub yeasts: Vec<YeastStack>,
}

/// Whether `item` is a cask Ferment could set going: holding juice, cordial
/// or water other than salt water, and not fermenting.
pub fn fermentable_cask(item: &Item) -> bool {
    item.cask().is_some_and(|cask| {
        !cask.time.is_fermenting()
            && match &cask.liquid {
                Some(Liquid::Juice(_) | Liquid::Cordial(_)) => true,
                Some(Liquid::Water(water)) => *water != Water::Salt,
                _ => false,
            }
    })
}

/// Whether `item` (a stack's top) is fruit or grain that ferments in water.
fn fermentable_add(item: &Item) -> bool {
    if let Some(fruit) = item.fruit() {
        let def = Fruits::built_in().get(&fruit.fruit);
        return def.is_some_and(|def| Mash::fruit(def, fruit.grams, 1000).is_some());
    }
    item.vegetable().is_some_and(|form| {
        let def = Vegetables::built_in().get(&form.vegetable);
        !form.serving && def.is_some_and(|def| def.set == VegetableSet::Grain)
    })
}

impl Inventory {
    /// What Ferment's panel offers from what is carried.
    pub fn ferment_choices(&self) -> FermentChoices {
        let casks = self.stacks_where(fermentable_cask);
        let adds = self.stacks_where(fermentable_add);
        let yeasts = self.stacks_where(|item| item.kind == ItemKind::Yeast);
        FermentChoices {
            casks: (casks.filter(|(stack, _)| stack.count() == 1))
                .map(|(stack, _)| stack.id)
                .collect(),
            adds: adds.map(|(stack, _)| stack.id).collect(),
            yeasts: (yeasts.filter(|(stack, _)| !self.is_worn(stack.id)))
                .map(|(stack, _)| YeastStack {
                    stack: stack.id,
                    quality: stack.top().quality.unwrap_or_default(),
                    usable: usable(stack, u32::MAX).len() as u32,
                })
                .filter(|yeast| yeast.usable > 0)
                .collect(),
        }
    }
}
