//! What feeding yeast and starting a starter use from what is carried: flour
//! by the sack, fresh water (any but salt) 100 ml at a time from bottles,
//! whole, or from vessels, and empty bottles. Never worn things, quest items
//! or the bin.

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid, Water};
use crate::pour::BOTTLE_ML;

use super::feed_stack::Supplies;

/// Fresh water feeds and starters take, a bottle's worth.
pub const WATER_ML: u32 = BOTTLE_ML;

/// Whether this is fresh water: any water but salt.
pub fn fresh(liquid: Option<&Liquid>) -> bool {
    matches!(liquid, Some(Liquid::Water(water)) if *water != Water::Salt)
}

/// How many 100 ml portions of fresh water one container holds: a full
/// bottle one, a vessel its millilitres in hundreds. Casks are left alone (a
/// tap would be needed).
pub fn water_portions(one: &Item) -> u32 {
    match &one.form {
        Some(ItemForm::Bottle(bottle)) if fresh(bottle.liquid.as_ref()) => 1,
        Some(ItemForm::Vessel(vessel)) if fresh(vessel.liquid.as_ref()) => vessel.ml / WATER_ML,
        _ => 0,
    }
}

/// The container `one` after 100 ml of its fresh water is taken: a bottle
/// empty, a vessel holding less. `None` if it holds too little.
pub fn drawn(one: &Item) -> Option<Item> {
    if water_portions(one) == 0 {
        return None;
    }
    let form = match &one.form {
        Some(ItemForm::Bottle(bottle)) => ItemForm::Bottle(BottleForm::new(None, bottle.look)),
        Some(ItemForm::Vessel(vessel)) => {
            let ml = vessel.ml - WATER_ML;
            let liquid = (ml > 0).then(|| vessel.liquid.clone()).flatten();
            ItemForm::Vessel(vessel.clone().holding(liquid, ml))
        }
        _ => return None,
    };
    Some(Item {
        form: Some(form),
        ..one.clone()
    })
}

/// Whether it is an empty bottle.
fn empty_bottle(one: &Item) -> bool {
    one.kind == ItemKind::Bottle && one.bottle().is_some_and(BottleForm::is_empty)
}

impl Inventory {
    /// What there is to feed yeast with: the sacks in the carried flour stack
    /// `flour` (none if it isn't flour), the fresh water and empty bottles
    /// carried.
    pub fn feed_supplies(&self, flour: Id) -> Supplies {
        let sacks = self.find(flour).filter(|found| {
            found.kind == ItemKind::Flour && !self.is_worn(flour) && !found.holds_quest_item()
        });
        let water = (self.stacks_where(|top| water_portions(top) > 0))
            .map(|(stack, _)| stack.members().map(water_portions).sum::<u32>())
            .sum();
        let bottles = (self.stacks_where(empty_bottle))
            .map(|(stack, _)| stack.count())
            .sum();
        Supplies {
            flour: sacks.map_or(0, Item::count),
            water,
            bottles,
        }
    }

    /// A carried stack to draw fresh water from: bottles first, then vessels.
    pub fn water_source(&self) -> Option<Id> {
        let of = |kind: ItemKind| {
            (self.stacks_where(move |top| top.kind == kind && water_portions(top) > 0))
                .map(|(stack, _)| stack.id)
                .next()
        };
        of(ItemKind::Bottle).or_else(|| of(ItemKind::Vessel))
    }

    /// A carried stack of empty bottles, for a split.
    pub fn empty_bottles(&self) -> Option<Id> {
        (self.stacks_where(empty_bottle))
            .map(|(stack, _)| stack.id)
            .next()
    }

    /// Uses up `sacks` of the carried flour stack `flour`: whether there were
    /// that many.
    pub fn use_flour(&mut self, flour: Id, sacks: u32) -> bool {
        let Some(found) = self.find_mut(flour).filter(|found| found.is_counted()) else {
            return false;
        };
        match found.count {
            count if count > sacks => {
                found.count -= sacks;
                true
            }
            count if count == sacks => self.take(flour).is_some(),
            _ => false,
        }
    }
}
