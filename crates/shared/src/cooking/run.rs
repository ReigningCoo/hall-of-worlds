//! Running a process (cooking plan PR 9): how long a run takes, the heat it
//! uses (for a hearth's Ash), and the practice it gives.

use super::{CookingSkills, OrderStep, Tool};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::Item;
use crate::item_form::ItemForm;

/// Practice to a step's sub-skill each run, whatever comes of it (flowing up
/// to Cooking; as Alchemy's).
pub const COOK_PRACTICE: u32 = 8;

impl Tool {
    /// Seconds a step with it takes untrained (first guesses: cutting,
    /// mixing, straining and pounding quick, frying longer, boiling and
    /// baking longest).
    pub fn first_seconds(self) -> u32 {
        match self {
            Self::Pan => 20,
            Self::Cauldron => 30,
            Self::Oven => 40,
            Self::Knife | Self::Bowl | Self::Sieve | Self::Mortar => 10,
        }
    }

    /// Seconds a step with it takes for these skills: less the ∛ of its
    /// sub-skill% (0 to 4).
    pub fn seconds(self, skills: CookingSkills) -> u32 {
        self.first_seconds() - skills.sub_root(self)
    }
}

/// Seconds a step takes: its tool's time, a join's half again.
fn step_seconds(step: &OrderStep, skills: CookingSkills) -> u32 {
    let halves = if step.height > 1 { 3 } else { 2 };
    step.tool.seconds(skills) * halves / 2
}

/// Seconds one run takes: every step's own time.
pub fn run_seconds(steps: &[OrderStep], skills: CookingSkills) -> u32 {
    let step = |step: &OrderStep| step_seconds(step, skills);
    steps.iter().map(step).sum::<u32>().max(1)
}

/// The same in action ticks.
pub fn run_ticks(steps: &[OrderStep], skills: CookingSkills) -> u64 {
    u64::from(run_seconds(steps, skills)) * ACTION_TICKS_PER_SECOND as u64
}

/// Seconds of heat a run uses: its heat steps' times
/// (`crafting_bench::hearth_ash` turns them into Ash at a hearth).
pub fn heat_seconds(steps: &[OrderStep], skills: CookingSkills) -> u64 {
    (steps.iter())
        .filter(|step| step.tool.needs_heat())
        .map(|step| u64::from(step_seconds(step, skills)))
        .sum()
}

/// A carried container with `ml` of its liquid drawn off: a bottle empty
/// (bottles are drawn whole), a vessel or cask holding less (opened). The
/// item as it was for anything else.
pub fn drawn(item: &Item, ml: u32) -> Item {
    let form = match &item.form {
        Some(ItemForm::Bottle(_)) => return crate::brewing::brewed_into(item.clone(), None),
        Some(ItemForm::Vessel(vessel)) => {
            let left = vessel.ml.saturating_sub(ml);
            let liquid = vessel.liquid.clone().filter(|_| left > 0);
            ItemForm::Vessel(vessel.clone().holding(liquid, left))
        }
        Some(ItemForm::Cask(cask)) => {
            let left = cask.ml.saturating_sub(ml);
            let liquid = cask.liquid.clone().filter(|_| left > 0);
            ItemForm::Cask(cask.clone().holding(liquid, left).opened())
        }
        _ => return item.clone(),
    };
    Item {
        form: Some(form),
        ..item.clone()
    }
}
