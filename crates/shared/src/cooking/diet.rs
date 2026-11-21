//! Diet labels shown on food (owner, 2026-11-21), worked out from its
//! ingredients' flags. Water and foods with no culinary data add nothing.

use serde::{Deserialize, Serialize};

use super::mix::Foods;
use super::{Ingredients, Portion};

/// Which labels a dish earns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Diet {
    /// No animal product at all (`animal_product` is true for meat too).
    pub vegan: bool,
    /// No meat.
    pub vegetarian: bool,
    pub gluten_free: bool,
    pub lactose_free: bool,
    /// No tree nut (chestnut and acorn count, coconut doesn't).
    pub nut_free: bool,
}

impl Diet {
    /// The labels of what these portions hold.
    pub fn of(portions: &[Portion], ingredients: &Ingredients) -> Self {
        let foods = Foods::of(portions, ingredients);
        let none = |flag: fn(&super::IngredientDef) -> bool| !foods.0.iter().any(|(d, _)| flag(d));
        Self {
            vegan: none(|def| def.animal_product),
            vegetarian: none(|def| def.meat),
            gluten_free: none(|def| def.gluten),
            lactose_free: none(|def| def.dairy),
            nut_free: none(|def| def.nut_allergen),
        }
    }

    /// Its labels' names in English, in this order, for the popup.
    pub fn labels(self) -> Vec<&'static str> {
        [
            (self.vegan, "Vegan"),
            (self.vegetarian, "Vegetarian"),
            (self.gluten_free, "Gluten free"),
            (self.lactose_free, "Lactose free"),
            (self.nut_free, "Nut free"),
        ]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
        .collect()
    }
}
