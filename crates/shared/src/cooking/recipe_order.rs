//! A cooked order written down as a recipe (`Inventory::cooking_recipe`),
//! for the cook's recipe book (`recipe::RecipeBook::remember_cooking`): each
//! food by what it is and how many, liquids by the millilitre, and the kind
//! of intermediate starting each row. Read before the order takes anything.

use super::{CookOrder, Ingredients, OrderInput, Recipe, RecipeInput, RecipeStep, Wanted};
use crate::item::{Inventory, Item};
use crate::potion::Liquid;
use crate::pour::Held;

impl Inventory {
    /// `order` as a recipe, from what is carried and in `storage` (a
    /// hearth's): `None` if a stack in it isn't there or isn't food.
    pub fn cooking_recipe(
        &self,
        order: &CookOrder,
        storage: &[Item],
        ingredients: &Ingredients,
    ) -> Option<Recipe> {
        let mut sources = Vec::new();
        for source in &order.sources {
            sources.push(match source {
                Some(id) => {
                    let carried = self.cook_stack(*id).map(Item::top);
                    let item = carried.or_else(|| storage.iter().find(|item| item.id == *id))?;
                    let rest = item.food()?.rest.as_ref()?.kind;
                    Some(rest)
                }
                None => None,
            });
        }
        let mut steps = Vec::new();
        for step in &order.steps {
            let inputs = (step.inputs.iter())
                .map(|input| self.recipe_input(input, ingredients))
                .collect::<Option<Vec<_>>>()?;
            steps.push(RecipeStep {
                tool: step.tool,
                column: step.column,
                row: step.row,
                height: step.height,
                inputs,
            });
        }
        let recipe = Recipe {
            rows: order.rows,
            sources,
            steps,
        };
        Some(recipe.normalised())
    }

    /// One input as a recipe's.
    fn recipe_input(&self, input: &OrderInput, ingredients: &Ingredients) -> Option<RecipeInput> {
        Some(match *input {
            OrderInput::Food { stack, count } => {
                let def = ingredients.of_item(self.cook_stack(stack)?.top())?;
                RecipeInput::Food {
                    foods: vec![Wanted::One(def.source, def.key.clone())],
                    count,
                }
            }
            OrderInput::Liquid { stack, ml } => {
                let top = self.cook_stack(stack)?.top();
                match Held::of(top)?.liquid? {
                    Liquid::Water(_) => RecipeInput::Water { ml },
                    liquid => {
                        let def = ingredients.of_liquid(&liquid)?;
                        RecipeInput::Liquid {
                            foods: vec![Wanted::One(def.source, def.key.clone())],
                            ml,
                        }
                    }
                }
            }
            OrderInput::WaterFaced { ml } => RecipeInput::Water { ml },
        })
    }
}
