//! A cooking recipe (recipes plan, `docs/context/recipes.md`): a process
//! written down as what goes where, foods by what they are and how many, not
//! by the item or its quality. A real dish's is built from its pattern
//! (`dish_recipe`); a character's own is remembered from what they cooked
//! (`Inventory::cooking_recipe`).

use serde::{Deserialize, Serialize};

use super::{CookingSkills, Heat, Ingredients, Portion, Process, Step, Tool, Wanted, chance};
use crate::alchemy::{CERTAIN, steps_allowed};
use crate::food::Rest;
use crate::item::Quality;

/// Something put into a step.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipeInput {
    /// `count` items of a food: any of `foods`, the first the usual one.
    Food { foods: Vec<Wanted>, count: u32 },
    /// Millilitres of a liquid food (milk, honey, soy sauce): any of `foods`.
    Liquid { foods: Vec<Wanted>, ml: u32 },
    /// Millilitres of water, from any source.
    Water { ml: u32 },
}

/// One tool step: where it is in the grid (a height over 1 joins rows) and
/// what is put into it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeStep {
    pub tool: Tool,
    pub column: u8,
    pub row: u8,
    pub height: u8,
    pub inputs: Vec<RecipeInput>,
}

/// A whole recipe: its rows, the kind of intermediate starting each row (if
/// any), and its steps by column, then row.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Recipe {
    pub rows: u8,
    #[serde(default)]
    pub sources: Vec<Option<Rest>>,
    pub steps: Vec<RecipeStep>,
}

impl Recipe {
    /// The same recipe, its steps by column then row and no trailing empty
    /// sources: two ways of writing one process compare equal.
    pub fn normalised(mut self) -> Self {
        self.steps.sort_by_key(|step| (step.column, step.row));
        while self.sources.last() == Some(&None) {
            self.sources.pop();
        }
        self
    }

    /// Its steps' weight in halves of a step (a join 3), as `Step::halves`.
    pub fn halves(&self) -> u32 {
        (self.steps.iter())
            .map(|step| if step.height > 1 { 3 } else { 2 })
            .sum()
    }

    /// Its chance for a cook this good (`cooking::chance`), in hundredths of
    /// a percent.
    pub fn chance(&self, skills: CookingSkills) -> u32 {
        chance(skills.cooking_percent(), self.halves())
    }

    /// The steps, or part of one, it has beyond what this Cooking skill
    /// allows (`alchemy::steps_allowed`): each halves its chance.
    pub fn steps_beyond(&self, skills: CookingSkills) -> u32 {
        let allowed = steps_allowed(skills.cooking_percent().min(100));
        self.halves().saturating_sub(2 * allowed).div_ceil(2)
    }

    /// Whether it is beyond this cook's skill: its chance under 100%.
    pub fn beyond_skill(&self, skills: CookingSkills) -> bool {
        self.chance(skills) < CERTAIN
    }

    /// The process it makes with the usual food of each input, every food at
    /// `quality`, worked at `heat`: `None` when a row starts from an
    /// intermediate (only the cook's own can be put there) or a food is
    /// unknown.
    pub fn process(
        &self,
        ingredients: &Ingredients,
        quality: Option<Quality>,
        heat: Heat,
    ) -> Option<Process> {
        if self.sources.iter().any(Option::is_some) {
            return None;
        }
        let mut steps = Vec::new();
        for step in &self.steps {
            let mut portions = Vec::new();
            for input in &step.inputs {
                portions.push(match input {
                    RecipeInput::Water { ml } => Portion::Water { ml: *ml },
                    RecipeInput::Food { foods, count } => {
                        let (source, key) = super::usual_food(foods.first()?, ingredients)?;
                        let unit = super::unit_grams(source, &key)?;
                        let grams = unit.saturating_mul(*count);
                        Portion::Food {
                            source,
                            key,
                            grams,
                            quality,
                        }
                    }
                    RecipeInput::Liquid { foods, ml } => {
                        let (source, key) = super::usual_food(foods.first()?, ingredients)?;
                        Portion::Food {
                            source,
                            key,
                            grams: *ml,
                            quality,
                        }
                    }
                });
            }
            steps.push(Step {
                tool: step.tool,
                column: step.column,
                row: step.row,
                height: step.height,
                portions,
            });
        }
        Some(Process {
            rows: self.rows,
            sources: Vec::new(),
            steps,
            heat,
        })
    }
}
