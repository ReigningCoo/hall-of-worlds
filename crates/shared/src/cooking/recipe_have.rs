//! Whether what a recipe asks for is carried (recipes plan PR 4): the
//! reading panel draws what is missing in red, and the Cooking panel's
//! Recipes tab (PR 5) fills itself from what is there (`recipe_stacks`,
//! `intermediate_stack`, `RecipeInput::takes`).

use super::{Ingredients, RecipeInput, Wanted};
use crate::food::Rest;
use crate::item::{Inventory, Item};
use crate::potion::Liquid;
use crate::pour::Held;

impl RecipeInput {
    /// Whether a stack whose top is `top` can go in for this input: an item
    /// of any of its foods, a bottle, vessel or cask of any of its liquids,
    /// or of water.
    pub fn takes(&self, top: &Item, ingredients: &Ingredients) -> bool {
        let liquid = || Held::of(top).and_then(|held| held.liquid);
        match self {
            Self::Food { foods, .. } => {
                (ingredients.of_item(top)).is_some_and(|def| wanted_by(foods, def.source, &def.key))
            }
            Self::Liquid { foods, .. } => liquid().is_some_and(|liquid| {
                (ingredients.of_liquid(&liquid))
                    .is_some_and(|def| wanted_by(foods, def.source, &def.key))
            }),
            Self::Water { .. } => liquid().is_some_and(|l| matches!(l, Liquid::Water(_))),
        }
    }

    /// How much it asks for: items of a food, millilitres of a liquid.
    pub fn amount(&self) -> u32 {
        match self {
            Self::Food { count, .. } => *count,
            Self::Liquid { ml, .. } | Self::Water { ml } => *ml,
        }
    }

    /// Whether it is counted in millilitres.
    pub fn is_liquid(&self) -> bool {
        !matches!(self, Self::Food { .. })
    }
}

impl Inventory {
    /// Whether enough of `input` is carried: as many items of any of its
    /// foods, as many millilitres of any of its liquids, or of water.
    pub fn has_recipe_input(&self, input: &RecipeInput, ingredients: &Ingredients) -> bool {
        let stacks = self.recipe_stacks(input, ingredients);
        let carried: u32 = match input.is_liquid() {
            false => stacks.iter().map(|stack| stack.count()).sum(),
            true => (stacks.iter())
                .map(|stack| Held::of(stack.top()).map_or(0, |held| held.ml) * stack.count())
                .sum(),
        };
        carried >= input.amount()
    }

    /// Every carried stack that can go in for `input` (`RecipeInput::takes`),
    /// in the character's own grid first, then in bags; never worn, never a
    /// quest item.
    pub fn recipe_stacks(&self, input: &RecipeInput, ingredients: &Ingredients) -> Vec<&Item> {
        let takes = |top: &Item| input.takes(top, ingredients);
        let mut stacks: Vec<(&Item, bool)> = (self.stacks_where(takes))
            .filter(|(stack, _)| !self.is_worn(stack.id))
            .collect();
        stacks.sort_by_key(|(_, own)| !own);
        stacks.into_iter().map(|(stack, _)| stack).collect()
    }

    /// Whether an intermediate of this kind is carried, to start a row from.
    pub fn has_intermediate(&self, rest: Rest) -> bool {
        self.intermediate_stack(rest).is_some()
    }

    /// A carried stack of intermediate of this kind, to start a row from.
    pub fn intermediate_stack(&self, rest: Rest) -> Option<&Item> {
        let is_it = |top: &Item| {
            (top.food())
                .and_then(|food| food.rest.as_ref())
                .is_some_and(|clock| clock.kind == rest)
        };
        (self.stacks_where(is_it))
            .find(|(stack, _)| !self.is_worn(stack.id))
            .map(|(stack, _)| stack)
    }
}

/// Whether a food is any of these.
fn wanted_by(foods: &[Wanted], source: super::Source, key: &str) -> bool {
    foods.iter().any(|wanted| wanted.matches(source, key))
}
