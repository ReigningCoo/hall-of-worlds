//! Brew as asked for (PR 7): which carried ointment and which bottle of water,
//! checked against what is carried with the same rules on both sides
//! (`Inventory::check_brew`), what the brewing panel offers, and the bottle of
//! water turned into the potion (or emptied, when none forms: decision 15).

use serde::{Deserialize, Serialize};

use super::{BrewSkills, brew, quality};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::cube_root;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Contents, Liquid, Water};

/// How long brewing takes untrained, in seconds.
const FIRST_SECONDS: u32 = 15;
/// Alchemy practice for one brewing, whatever comes of it (decisions 15, 23;
/// first guess).
pub const BREW_PRACTICE: u32 = 10;
/// Herbalism practice for one brewing: watching what the herbs became (first
/// guess).
pub const BREW_HERB_PRACTICE: u32 = 2;

/// A Brew asked for: one carried ointment and one carried bottle of water.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BrewOrder {
    pub ointment: Id,
    pub water: Id,
}

/// Why a Brew can't be done with what is carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrewRefused {
    /// The ointment isn't a carried ointment.
    Ointment,
    /// The water isn't a carried bottle of water.
    Water,
}

/// What a Brew would make: the ointment's doses, the water, and the potion
/// (`None` when none forms, both used up all the same).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Brewed {
    pub ointment: Contents,
    pub water: Water,
    pub potion: Option<Contents>,
}

/// A carried ointment stack the brewing panel can offer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OintmentStack {
    pub stack: Id,
    pub contents: Contents,
    pub quality: Quality,
    pub count: u32,
}

/// A carried stack of water bottles the brewing panel can offer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaterStack {
    pub stack: Id,
    pub water: Water,
    pub count: u32,
}

/// Seconds a brewing takes: 15 − ∛Alchemy%, 15 to 11 (first guess).
pub fn brew_seconds(skills: BrewSkills) -> u32 {
    FIRST_SECONDS - cube_root(skills.alchemy_percent())
}

/// The same in action ticks.
pub fn brew_ticks(skills: BrewSkills) -> u64 {
    u64::from(brew_seconds(skills)) * ACTION_TICKS_PER_SECOND as u64
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

    /// What `order` would make from what is carried, or why not. The server
    /// checks again as it lands, taking what was used.
    pub fn check_brew(&self, order: &BrewOrder) -> Result<Brewed, BrewRefused> {
        let ointments = self.ointment_stacks();
        let ointment = ointments.iter().find(|stack| stack.stack == order.ointment);
        let ointment = ointment.ok_or(BrewRefused::Ointment)?.contents.clone();
        let waters = self.water_stacks();
        let water = waters.iter().find(|stack| stack.stack == order.water);
        let water = water.ok_or(BrewRefused::Water)?.water;
        let potion = brew(&ointment, water);
        Ok(Brewed {
            ointment,
            water,
            potion,
        })
    }
}
