//! The beyond-skill note (recipes plan): when a process has more steps than
//! the cook's Cooking skill allows, how many and the chance left, in the
//! Recipes tab's list, under the Cooking tab's grid (after Make, or any
//! process built by hand) and on the reading panel.

use shared::alchemy::{CERTAIN, steps_allowed};
use shared::cooking::{CookingSkills, chance};
use shared::lang::{Args, Lang};

use super::bench::Bench;

/// "2 steps beyond your Cooking skill: a 25% chance.", or `None` when a
/// process of this many halves of a step is certain.
pub(super) fn beyond_words(halves: u32, skills: CookingSkills, lang: &Lang) -> Option<String> {
    let percent = skills.cooking_percent();
    let chance = chance(percent, halves);
    if chance >= CERTAIN {
        return None;
    }
    let allowed = steps_allowed(percent.min(100));
    let over = halves.saturating_sub(2 * allowed).div_ceil(2).max(1);
    let args = Args::new()
        .count("over", i64::from(over))
        .count("chance", i64::from(chance / 100));
    Some(
        lang.render("recipe-beyond", &args)
            .unwrap_or_else(|error| error.text),
    )
}

impl Bench {
    /// Its steps' weight in halves of a step (a join 3), as `Recipe::halves`.
    pub(super) fn halves(&self) -> u32 {
        (self.steps.iter())
            .map(|step| if step.height > 1 { 3 } else { 2 })
            .sum()
    }
}
