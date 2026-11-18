//! Collect Water filling casks by the litre (drinks plan PR 1): once no empty
//! bottle is left, each wind-up of `CASK_FILL_SECONDS` pours `cask_litres` into
//! a carried cask that takes the water (1 to 2 litres a second by the Water
//! skill). The server decides; the client greys Collect Water by the same rule.

use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::{cube_root, skill_percent, skill_value};
use crate::id::Id;
use crate::item::{Inventory, Item};
use crate::item_form::ItemForm;
use crate::potion::{Liquid, WATER_SKILL};
use crate::skill::Skills;

/// How long each pour into a cask takes, in seconds.
pub const CASK_FILL_SECONDS: u32 = 5;

/// Water practice for each pour into a cask: as much as a bottle gives.
pub const CASK_PRACTICE: u32 = crate::potion::COLLECT_PRACTICE;

/// The same in action ticks.
pub fn cask_fill_ticks() -> u64 {
    u64::from(CASK_FILL_SECONDS) * ACTION_TICKS_PER_SECOND as u64
}

/// How many litres one pour adds: 5 untrained, up to 10 at 100% Water (the
/// cube root of its percent, 0 to 4, times 5 / 4), so 1 to 2 litres a second.
pub fn cask_litres(skills: &Skills) -> u32 {
    let root = cube_root(skill_percent(skill_value(skills, WATER_SKILL)));
    5 + root * 5 / 4
}

/// Whether this is a cask water can go into: one with room, empty or already
/// holding a water (which one is the server's to say, by where it is filled).
fn takes_some_water(item: &Item) -> bool {
    item.cask().is_some_and(|cask| {
        cask.room() > 0
            && cask
                .liquid
                .as_ref()
                .is_none_or(|held| matches!(held, Liquid::Water(_)))
    })
}

impl Inventory {
    /// Whether a carried cask could take some water: what Collect Water needs
    /// when no empty bottle is left.
    pub fn cask_for_water(&self) -> bool {
        self.stacks_where(takes_some_water).next().is_some()
    }

    /// The carried cask `liquid` is poured into next: one already holding it
    /// before an empty one, the fullest first, the own grid before bags.
    pub fn cask_to_fill(&self, liquid: &Liquid) -> Option<Id> {
        let takes = |top: &Item| top.cask().is_some_and(|cask| cask.takes(liquid));
        self.stacks_where(takes)
            .max_by_key(|(cask, own)| (*own, cask.cask().map_or(0, |cask| cask.litres)))
            .map(|(cask, _)| cask.id)
    }
}

/// Pours up to `litres` of `liquid` into the carried cask `id`: how many went
/// in (0 if it isn't a cask that takes it).
pub fn fill_cask_with(inventory: &mut Inventory, id: Id, liquid: Liquid, litres: u32) -> u32 {
    let Some(item) = inventory.find_mut(id) else {
        return 0;
    };
    let Some(ItemForm::Cask(cask)) = &mut item.form else {
        return 0;
    };
    let (poured, added) = cask.clone().poured(liquid, litres);
    *cask = poured;
    added
}
