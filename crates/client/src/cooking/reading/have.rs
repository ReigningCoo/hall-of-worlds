//! What of a recipe is to hand (recipes plan PR 4): each input carried in
//! enough (water faced counting as water), each tool carried or in the
//! kitchen (the Cooking panel's rule, `has_cooking_tool`), each row's
//! intermediate carried or in the hearth's storage. What is not is drawn red
//! in the grid and in the written steps.

use shared::cooking::{Ingredients, Recipe, RecipeInput, Wanted};
use shared::item::Inventory;
use shared::item_text::KindToken;

use super::super::kitchen::KitchenNow;

/// Whether each part of a recipe is to hand.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Have {
    /// Per step, in the recipe's order: its tool.
    pub(super) tools: Vec<bool>,
    /// Per step, per input.
    pub(super) inputs: Vec<Vec<bool>>,
    /// Per row: its intermediate, or nothing asked.
    pub(super) sources: Vec<bool>,
}

impl Have {
    pub(super) fn of(recipe: &Recipe, carried: &Inventory, kitchen: &KitchenNow) -> Self {
        let ingredients = Ingredients::built_in();
        let input_there = |input: &RecipeInput| match input {
            RecipeInput::Water { .. } if kitchen.water_faced => true,
            _ => carried.has_recipe_input(input, ingredients),
        };
        let stored_rest = |rest| {
            (kitchen.storage.iter())
                .filter_map(|item| item.food()?.rest.as_ref())
                .any(|clock| clock.kind == rest)
        };
        Self {
            tools: (recipe.steps.iter())
                .map(|step| carried.has_cooking_tool(step.tool, kitchen.kitchen()))
                .collect(),
            inputs: (recipe.steps.iter())
                .map(|step| step.inputs.iter().map(input_there).collect())
                .collect(),
            sources: (recipe.sources.iter())
                .map(|source| {
                    source.is_none_or(|rest| carried.has_intermediate(rest) || stored_rest(rest))
                })
                .collect(),
        }
    }

    /// Whether this step's tool is to hand.
    pub(super) fn tool(&self, step: usize) -> bool {
        self.tools.get(step).copied().unwrap_or(true)
    }

    /// Whether this step's input is to hand.
    pub(super) fn input(&self, step: usize, input: usize) -> bool {
        (self.inputs.get(step))
            .and_then(|inputs| inputs.get(input))
            .copied()
            .unwrap_or(true)
    }

    /// Whether this row's intermediate is to hand (or none is asked).
    pub(super) fn source(&self, row: usize) -> bool {
        self.sources.get(row).copied().unwrap_or(true)
    }
}

/// The kind of item an input is written as: its usual food, or water.
pub(super) fn input_token(input: &RecipeInput) -> Option<KindToken> {
    match input {
        RecipeInput::Food { foods, .. } | RecipeInput::Liquid { foods, .. } => {
            foods.first().cloned().map(KindToken::Food)
        }
        RecipeInput::Water { .. } => Some(KindToken::Water(None)),
    }
}

/// An input's foods, by name: "Carrot", "Carrot or Parsnip".
pub(super) fn foods_named(foods: &[Wanted]) -> String {
    let names: Vec<String> = (foods.iter())
        .map(|food| KindToken::Food(food.clone()).name())
        .collect();
    names.join(" or ")
}
