//! What a recipe book's entry shows: its recipe, the dish's name, and where
//! the dish is from (a real dish's) or nothing (the reader's own).

use shared::cooking::{Dishes, Recipe, dish_recipe};
use shared::recipe::{Craft, Entry, Own};

/// A recipe as the panel shows it.
pub(in crate::cooking) struct Shown {
    pub(in crate::cooking) recipe: Recipe,
    /// The dish's name: "Naan", "Fried Carrot".
    pub(in crate::cooking) dish: String,
    /// Where a real dish is from ("Indian"); `None` for the reader's own.
    pub(in crate::cooking) cuisine: Option<String>,
}

impl Shown {
    /// `None` for a recipe this panel can't show (Alchemy's, later).
    pub(in crate::cooking) fn of(entry: &Entry) -> Option<Self> {
        let dishes = Dishes::built_in();
        match entry {
            Entry::Learnt {
                craft: Craft::Cooking,
                dish,
                ..
            } => {
                let def = dishes.get(dish)?;
                Some(Self {
                    recipe: dish_recipe(dish)?.clone(),
                    dish: def.name.clone(),
                    cuisine: Some(def.cuisine.clone()),
                })
            }
            Entry::Own(Own::Cooking { recipe, name }) => Some(Self {
                recipe: recipe.clone(),
                dish: name.name(dishes),
                cuisine: None,
            }),
            _ => None,
        }
    }
}
