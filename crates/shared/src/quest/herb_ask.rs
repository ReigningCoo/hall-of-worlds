//! A gathering step's herb (owner, 2026-11-27): one named herb that grows wild
//! where the step sends the player, never one the giver's town's shops could
//! stock. Ordinary herb items of it count, however they were got and wherever
//! they are carried (bags at any depth). Some steps ask for a rarer herb, and
//! pay more for it (`reward`).

use serde::{Deserialize, Serialize};

use crate::herb::{Abundance, HerbDef, Herbs, wild_herbs};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};
use crate::mob::Habitat;
use crate::rng::Rng;
use crate::shop::could_stock_herb;
use crate::site::BuildingKind;

/// Percent of gathering steps asking for a rarer herb (Uncommon or Rare),
/// where one grows. *To balance.*
pub const RARER_PERCENT: u64 = 25;

/// The herb a gathering step asks for, and the least quality that counts.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HerbAsk {
    /// The herb's key (`HerbDef::key`).
    pub herb: String,
    /// The least quality counted: Poor (any) for most, the herb's own rarity
    /// for a step asking a rarer one. A picked herb's quality is its plant's
    /// rarity (`Abundance::quality`), so asking a better quality means asking
    /// a rarer herb.
    pub least: Quality,
}

/// The herbs a gathering step at a place like `habitat` may ask for: those
/// growing wild there that the giver's town's shops couldn't stock (`stocked`,
/// by key: `shops_stock`).
pub(super) fn herb_choices<'a>(
    herbs: &'a Herbs,
    habitat: &Habitat,
    stocked: &dyn Fn(&str) -> bool,
) -> Vec<&'a HerbDef> {
    (wild_herbs(herbs, habitat).into_iter())
        .filter(|def| !stocked(&def.key))
        .collect()
}

/// Whether any of `shops` (a town's buildings' kinds) could stock a herb.
pub(super) fn shops_stock(shops: &[BuildingKind], herb: &str) -> bool {
    shops.iter().any(|&shop| could_stock_herb(shop, herb))
}

/// The herb a step asks for, from `choices` (commoner ones more often), now
/// and then a rarer one asked at its own quality. `None` with no choices.
pub(super) fn choose_ask(choices: &[&HerbDef], rng: &mut Rng) -> Option<HerbAsk> {
    let rarer = rng.range_u64(0, 100) < RARER_PERCENT;
    let rare: Vec<&HerbDef> = (choices.iter().copied())
        .filter(|def| def.abundance != Abundance::Common)
        .collect();
    let (from, rarer) = match rarer && !rare.is_empty() {
        true => (rare, true),
        false => (choices.to_vec(), false),
    };
    let def = crate::herb::choose(&from, Abundance::wild_weight, rng)?;
    Some(HerbAsk {
        herb: def.key.clone(),
        least: match rarer {
            true => def.abundance.quality(),
            false => Quality::Poor,
        },
    })
}

impl HerbAsk {
    /// Whether a stack (by its top) counts: the herb, at the quality asked or
    /// better. Never a quest item.
    pub fn counts(&self, top: &Item) -> bool {
        top.kind == ItemKind::Herb
            && top.herb().is_some_and(|form| form.herb == self.herb)
            && top.quality.unwrap_or(Quality::Common) >= self.least
            && !top.is_quest_item()
    }

    /// How many are carried, in the own grid, worn bags and bags in bags.
    pub fn carried(&self, inventory: &Inventory) -> u32 {
        (inventory.stacks_where(|top| self.counts(top)))
            .map(|(stack, _)| stack.count())
            .sum()
    }

    /// Takes `count` of them out of what is carried, the poorest first (then
    /// the smallest stacks), so the best are kept. `false`, taking nothing, if
    /// fewer are carried.
    pub fn take(
        &self,
        inventory: &mut Inventory,
        count: u32,
        new_id: &mut impl FnMut() -> Id,
    ) -> bool {
        if self.carried(inventory) < count {
            return false;
        }
        let mut stacks: Vec<(Quality, u32, Id)> = (inventory.stacks_where(|top| self.counts(top)))
            .map(|(stack, _)| {
                let quality = stack.top().quality.unwrap_or(Quality::Common);
                (quality, stack.count(), stack.id)
            })
            .collect();
        stacks.sort();
        let mut left = count;
        for (_, has, stack) in stacks {
            if left == 0 {
                break;
            }
            let taken = has.min(left);
            let Some(part) = inventory.lift_count(stack, taken, new_id) else {
                return false;
            };
            if inventory.hand_over(part).is_none() {
                return false;
            }
            left -= taken;
        }
        true
    }

    /// Quality steps asked above Common: 0 for any, 1 for Uncommon, 2 for
    /// Rare (the reward is raised by them).
    pub fn steps_above_common(&self) -> u64 {
        (self.least as u64).saturating_sub(Quality::Common as u64)
    }
}

#[cfg(test)]
mod tests;
