//! Whether what a recipe asks for is carried (recipes plan PR 4): the
//! reading panel draws what is missing in red, and the Cooking panel's
//! Recipes tab (PR 5) fills itself from what is there.

use super::{Ingredients, RecipeInput, Wanted};
use crate::food::Rest;
use crate::item::{Inventory, Item};
use crate::potion::Liquid;
use crate::pour::Held;

impl Inventory {
    /// Whether enough of `input` is carried: as many items of any of its
    /// foods, as many millilitres of any of its liquids, or of water.
    pub fn has_recipe_input(&self, input: &RecipeInput, ingredients: &Ingredients) -> bool {
        match input {
            RecipeInput::Food { foods, count } => {
                let is_food = |top: &Item| {
                    (ingredients.of_item(top))
                        .is_some_and(|def| wanted_by(foods, def.source, &def.key))
                };
                let carried: u32 = (self.stacks_where(is_food))
                    .map(|(stack, _)| stack.count())
                    .sum();
                carried >= *count
            }
            RecipeInput::Liquid { foods, ml } => {
                let is_liquid = |liquid: &Liquid| {
                    (ingredients.of_liquid(liquid))
                        .is_some_and(|def| wanted_by(foods, def.source, &def.key))
                };
                self.liquid_ml(is_liquid) >= *ml
            }
            RecipeInput::Water { ml } => {
                self.liquid_ml(|liquid| matches!(liquid, Liquid::Water(_))) >= *ml
            }
        }
    }

    /// Whether an intermediate of this kind is carried, to start a row from.
    pub fn has_intermediate(&self, rest: Rest) -> bool {
        let is_it = |top: &Item| {
            (top.food())
                .and_then(|food| food.rest.as_ref())
                .is_some_and(|clock| clock.kind == rest)
        };
        self.stacks_where(is_it).next().is_some()
    }

    /// How many millilitres of liquids passing `wanted` are carried, in
    /// bottles, vessels and casks.
    fn liquid_ml(&self, wanted: impl Fn(&Liquid) -> bool) -> u32 {
        let holds = |top: &Item| {
            (Held::of(top)).is_some_and(|held| held.liquid.as_ref().is_some_and(&wanted))
        };
        (self.stacks_where(holds))
            .map(|(stack, _)| Held::of(stack.top()).map_or(0, |held| held.ml) * stack.count())
            .sum()
    }
}

/// Whether a food is any of these.
fn wanted_by(foods: &[Wanted], source: super::Source, key: &str) -> bool {
    foods.iter().any(|wanted| wanted.matches(source, key))
}
