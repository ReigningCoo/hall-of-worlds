//! A recipe loaded onto the bench by the Recipes tab's Make (recipes plan
//! PR 5): its steps placed, and what each asks for (`Station::wanted`, each
//! row's intermediate in `Bench::wanted_sources`) filled from matching
//! carried stacks with the recipe's counts and millilitres; the water faced
//! gives water when there is some. Whatever is still missing is a ghost: a
//! faint red picture of it in the step's spaces, and Start stays off while
//! any is left. Ghosts fill themselves when what they want is picked up,
//! take a matching stack dropped on them, and a click or tap on one gives up
//! on it.

use shared::cooking::{Ingredients, Recipe, RecipeInput};
use shared::food::Rest;
use shared::id::Id;
use shared::item::Inventory;

use super::bench::{Bench, ROWS, Station};
use super::kitchen::KitchenNow;
use super::spaces::{ML_STEP, MOST_FOODS, MOST_LIQUIDS, most_ml};

/// One ghost: which of the step's wanted inputs, and how much of it is
/// still missing (items, or millilitres).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Ghost {
    pub(super) input: usize,
    pub(super) missing: u32,
}

impl Station {
    /// What of `wanted` is not put in yet: each input takes from the stacks
    /// put in that match it (the water faced counting as water), in order.
    pub(super) fn ghosts(&self, carried: &Inventory) -> Vec<Ghost> {
        let ingredients = Ingredients::built_in();
        let mut foods: Vec<(Id, u32)> = self.foods.clone();
        let mut liquids: Vec<(Id, u32)> = self.liquids.clone();
        let mut faced = self.faced;
        let mut ghosts = Vec::new();
        for (input, wanted) in self.wanted.iter().enumerate() {
            let mut need = wanted.amount();
            if matches!(wanted, RecipeInput::Water { .. }) {
                let taken = need.min(faced);
                (need, faced) = (need - taken, faced - taken);
            }
            let list = if wanted.is_liquid() {
                &mut liquids
            } else {
                &mut foods
            };
            for (id, left) in list.iter_mut() {
                let fits = (carried.find(*id)).is_some_and(|s| wanted.takes(s.top(), ingredients));
                if need > 0 && fits {
                    let taken = need.min(*left);
                    (need, *left) = (need - taken, *left - taken);
                }
            }
            if need > 0 {
                ghosts.push(Ghost {
                    input,
                    missing: need,
                });
            }
        }
        ghosts
    }
}

impl Bench {
    /// Clears the bench and lays `recipe` on it, filled from what is carried.
    pub(super) fn load(&mut self, recipe: &Recipe, carried: &Inventory, kitchen: &KitchenNow) {
        self.sources = [None; ROWS];
        self.wanted_sources = [None; ROWS];
        for (row, source) in recipe.sources.iter().take(ROWS).enumerate() {
            self.wanted_sources[row] = *source;
        }
        self.steps = (recipe.steps.iter())
            .map(|step| Station {
                height: step.height.max(1),
                wanted: step.inputs.clone(),
                ..Station::new(step.tool, step.column, step.row)
            })
            .collect();
        self.count = 1;
        self.made = None;
        self.top_up(carried, kitchen);
    }

    /// The intermediate row `row` still waits for, if any.
    pub(super) fn source_missing(&self, row: usize) -> Option<Rest> {
        let wanted = (*self.wanted_sources.get(row)?)?;
        self.sources[row].is_none().then_some(wanted)
    }

    /// Whether anything a loaded recipe asks for is still missing.
    pub(super) fn missing_any(&self, carried: &Inventory) -> bool {
        (0..ROWS).any(|row| self.source_missing(row).is_some())
            || (self.steps.iter()).any(|step| !step.ghosts(carried).is_empty())
    }

    /// Fills every ghost it can from what is carried (and the water faced,
    /// and the hearth's storage for intermediates); whether anything went in.
    pub(super) fn top_up(&mut self, carried: &Inventory, kitchen: &KitchenNow) -> bool {
        let mut changed = false;
        for row in 0..ROWS {
            let Some(rest) = self.source_missing(row) else {
                continue;
            };
            let stored = (kitchen.storage.iter()).find(|item| {
                (item.food().and_then(|f| f.rest.as_ref())).is_some_and(|c| c.kind == rest)
            });
            let found = carried.intermediate_stack(rest).or(stored);
            if let Some(item) = found {
                self.sources[row] = Some(item.id);
                changed = true;
            }
        }
        for step in 0..self.steps.len() {
            for ghost in self.steps[step].ghosts(carried) {
                changed |= self.fill(carried, kitchen, (step, ghost), None);
            }
        }
        if changed {
            self.made = None;
        }
        changed
    }

    /// Whether the carried stack `id` can go into the step's `n`th ghost.
    pub(super) fn ghost_takes(
        &self,
        carried: &Inventory,
        (step, n): (usize, usize),
        id: Id,
    ) -> bool {
        let Some(station) = self.steps.get(step) else {
            return false;
        };
        let Some(ghost) = station.ghosts(carried).get(n).copied() else {
            return false;
        };
        let wanted = &station.wanted[ghost.input];
        let stack = (!carried.is_worn(id)).then(|| carried.find(id)).flatten();
        stack.is_some_and(|stack| wanted.takes(stack.top(), Ingredients::built_in()))
    }

    /// A stack dropped on the step's `n`th ghost: as much of it as is missing.
    pub(super) fn drop_on_ghost(
        &mut self,
        carried: &Inventory,
        kitchen: &KitchenNow,
        (step, n): (usize, usize),
        id: Id,
    ) {
        let ghost = (self.steps.get(step)).and_then(|s| s.ghosts(carried).get(n).copied());
        if let Some(ghost) = ghost
            && self.ghost_takes(carried, (step, n), id)
            && self.fill(carried, kitchen, (step, ghost), Some(id))
        {
            self.made = None;
        }
    }

    /// A click or tap on the step's `n`th ghost: cook without it.
    pub(super) fn give_up(&mut self, carried: &Inventory, (step, n): (usize, usize)) {
        let Some(station) = self.steps.get(step) else {
            return;
        };
        if let Some(ghost) = station.ghosts(carried).get(n) {
            let input = ghost.input;
            self.steps[step].wanted.remove(input);
            self.made = None;
        }
    }

    /// Puts what a ghost misses into its step: water faced for water if
    /// there is some, else from `only` or every matching carried stack, as
    /// much as each has left after what the bench uses of it.
    fn fill(
        &mut self,
        carried: &Inventory,
        kitchen: &KitchenNow,
        (step, ghost): (usize, Ghost),
        only: Option<Id>,
    ) -> bool {
        let wanted = self.steps[step].wanted[ghost.input].clone();
        let liquid = wanted.is_liquid();
        // Liquids go in by whole 100 ml, as a drop does.
        let mut need = match liquid {
            true => ghost.missing.div_ceil(ML_STEP) * ML_STEP,
            false => ghost.missing,
        };
        if matches!(wanted, RecipeInput::Water { .. }) && kitchen.water_faced && only.is_none() {
            self.steps[step].faced += need;
            return true;
        }
        let stacks: Vec<(Id, u32)> = (carried.recipe_stacks(&wanted, Ingredients::built_in()))
            .into_iter()
            .filter(|stack| only.is_none_or(|id| id == stack.id))
            .map(|stack| match liquid {
                true => (stack.id, most_ml(carried, stack.id)),
                false => (stack.id, stack.count()),
            })
            .collect();
        let mut changed = false;
        for (id, has) in stacks {
            let used = (self.uses().into_iter())
                .find(|u| u.0 == id)
                .map_or(0, |(_, count, ml)| if liquid { ml } else { count });
            let take = need.min(has.saturating_sub(used));
            let station = &mut self.steps[step];
            let (list, most) = match liquid {
                true => (&mut station.liquids, MOST_LIQUIDS),
                false => (&mut station.foods, MOST_FOODS),
            };
            if take == 0 {
                continue;
            }
            match list.iter().position(|(it, _)| *it == id) {
                Some(at) => list[at].1 += take,
                None if list.len() < most => list.push((id, take)),
                None => continue,
            }
            need -= take;
            changed = true;
            if need == 0 {
                break;
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests;
