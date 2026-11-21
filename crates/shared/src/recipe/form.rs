//! Recipe scrolls as items: `ItemKind::Recipe` with a `RecipeForm` saying
//! which craft and dish it teaches. One cell, no quality, counted: a stack
//! keys on its craft and dish alone.

use serde::{Deserialize, Serialize};

use super::Craft;
use crate::cooking::{Dishes, dish_recipe};
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::rng::Seed;

/// What one scroll weighs, in grams: a rolled sheet of parchment.
pub const RECIPE_GRAMS: u32 = 20;

/// What a recipe scroll teaches: a craft's dish, by its key (for Cooking,
/// `cooking::Dishes`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeForm {
    pub craft: Craft,
    pub dish: String,
}

impl RecipeForm {
    /// A Cooking scroll of this dish.
    pub fn cooking(dish: impl Into<String>) -> Self {
        Self {
            craft: Craft::Cooking,
            dish: dish.into(),
        }
    }

    /// What it is called: "Recipe for Naan", or "Recipe" for a dish the
    /// table lacks.
    pub fn name(&self) -> String {
        match self.craft {
            Craft::Cooking => match Dishes::built_in().get(&self.dish) {
                Some(def) => format!("Recipe for {}", def.name),
                None => "Recipe".to_owned(),
            },
            Craft::Alchemy => "Recipe".to_owned(),
        }
    }

    /// Whether it teaches something there is a recipe for.
    pub fn is_known(&self) -> bool {
        match self.craft {
            Craft::Cooking => dish_recipe(&self.dish).is_some(),
            Craft::Alchemy => false,
        }
    }

    /// The cells one takes up.
    pub fn shape(&self) -> Shape {
        Shape::rectangle(1, 1)
    }
}

/// `count` scrolls of `form`, as one new item with id `id`: never magical,
/// blessed or cursed, and without quality.
pub fn new_item(form: RecipeForm, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Recipe);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Recipe(form));
    item
}

/// Give Item's scroll: `dish` if it is a real dish, else one rolled from the
/// seed; `None` if no dish has a recipe.
pub fn dev_recipe(dish: Option<&str>, seed: Seed) -> Option<RecipeForm> {
    let dishes = Dishes::built_in();
    if let Some(key) = dish.filter(|key| dish_recipe(key).is_some()) {
        return Some(RecipeForm::cooking(key));
    }
    let keys: Vec<&str> = (dishes.iter())
        .map(|def| def.key.as_str())
        .filter(|key| dish_recipe(key).is_some())
        .collect();
    let at = seed.derive("recipe").value() % (keys.len().max(1) as u64);
    keys.get(at as usize).map(|key| RecipeForm::cooking(*key))
}

impl Item {
    /// Which recipe it is, if it is a recipe scroll.
    pub fn recipe(&self) -> Option<&RecipeForm> {
        match &self.form {
            Some(ItemForm::Recipe(recipe)) => Some(recipe),
            _ => None,
        }
    }
}
