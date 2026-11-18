//! Grind as asked for (PR 6): which carried herb stacks, which bottle of oil
//! and how many doses; checked against what is carried with the same rules on
//! both sides (`Inventory::check_grind`), and what the mixing panel offers.

use serde::{Deserialize, Serialize};

use super::{BrewSkills, GroundHerb, Ointment, grind};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::{Herbs, cube_root};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::potion::{Liquid, Oil};

/// How long grinding takes untrained, in seconds.
const FIRST_SECONDS: u32 = 10;
/// Alchemy practice for one grinding, whatever comes of it (decisions 15, 23).
pub const GRIND_PRACTICE: u32 = 8;
/// Herbalism practice for each kind of herb handled (decision 23).
pub const HERB_PRACTICE: u32 = 2;

/// A Grind asked for: one carried stack of each herb (one of each kind counts),
/// a carried bottle of oil, and how many doses (`doses` of each herb used).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GrindOrder {
    pub herbs: Vec<Id>,
    pub oil: Id,
    pub doses: u32,
}

/// Why a Grind can't be done with what is carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrindRefused {
    /// A herb stack isn't carried, isn't a herb, or holds fewer than the doses.
    Herb,
    /// A herb the server's table doesn't have.
    UnknownHerb,
    /// The oil isn't a carried bottle of oil.
    Oil,
    /// The rules refused the mix (`grind::Refused`).
    Rules(super::Refused),
}

/// Seconds a grinding takes: 10 − ∛Alchemy%, 10 to 6 (first guess).
pub fn grind_seconds(skills: BrewSkills) -> u32 {
    FIRST_SECONDS - cube_root(skills.alchemy_percent())
}

/// The same in action ticks.
pub fn grind_ticks(skills: BrewSkills) -> u64 {
    u64::from(grind_seconds(skills)) * ACTION_TICKS_PER_SECOND as u64
}

/// A carried stack the mixing panel can offer as a herb.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HerbStack {
    pub stack: Id,
    pub herb: String,
    pub quality: Quality,
    pub count: u32,
}

/// A carried stack of oil bottles the mixing panel can offer.
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

    /// What `order` would make from what is carried, and its oil, or why not.
    /// The server checks again as it lands, taking what was used.
    pub fn check_grind(
        &self,
        order: &GrindOrder,
        herbs: &Herbs,
        skills: BrewSkills,
    ) -> Result<(Ointment, Oil), GrindRefused> {
        let stacks = self.herb_stacks();
        let mut ground = Vec::new();
        for id in &order.herbs {
            let stack = stacks.iter().find(|stack| stack.stack == *id);
            let stack = stack.filter(|stack| stack.count >= order.doses);
            let stack = stack.ok_or(GrindRefused::Herb)?;
            let def = herbs.get(&stack.herb).ok_or(GrindRefused::UnknownHerb)?;
            ground.push(GroundHerb {
                def,
                quality: stack.quality,
            });
        }
        let oils = self.oil_stacks();
        let oil = oils.iter().find(|stack| stack.stack == order.oil);
        let oil = oil.ok_or(GrindRefused::Oil)?.oil;
        let made = grind(&ground, oil, order.doses, skills).map_err(GrindRefused::Rules)?;
        Ok((made, oil))
    }
}
