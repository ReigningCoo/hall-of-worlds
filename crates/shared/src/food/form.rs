//! A food item's form: what it is (`cooking::Dish`), what went into it and
//! its steps, and an intermediate's rest clock.

use serde::{Deserialize, Serialize};

use super::RestClock;
use crate::cooking::{Carried, Diet, Dish, DishForm, Dishes, History, Ingredients, Portion};
use crate::item::{Item, Shape};
use crate::item_form::ItemForm;

/// A Dish's or an intermediate's form.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoodForm {
    /// What it was named and served as when made.
    pub dish: Dish,
    /// What went into it: one serving's for a Dish, the whole of an
    /// intermediate.
    pub portions: Vec<Portion>,
    /// Its steps and the cooks' sub-skills at them.
    pub history: History,
    /// An intermediate's clock; `None` for a Dish.
    #[serde(default)]
    pub rest: Option<RestClock>,
}

impl FoodForm {
    /// What it is called: a Dish by its dish ("Fried Egg", "Mutton and
    /// Turnip Stew"), an intermediate by its kind, dough and noodles with
    /// their biggest food ("Wheat Dough").
    pub fn name(&self) -> String {
        let Some(rest) = &self.rest else {
            return self.dish.name(Dishes::built_in());
        };
        use super::Rest::{Dough, DriedNoodles, FreshNoodles};
        let food = (matches!(rest.kind, Dough | FreshNoodles | DriedNoodles))
            .then(|| self.biggest_food())
            .flatten();
        match food {
            Some(food) => format!("{food} {}", rest.kind.name()),
            None => rest.kind.name().to_owned(),
        }
    }

    fn biggest_food(&self) -> Option<String> {
        let (_, source, key) = (self.portions.iter())
            .filter_map(|portion| match portion {
                Portion::Food {
                    source, key, grams, ..
                } => Some((*grams, *source, key)),
                Portion::Water { .. } => None,
            })
            .max_by_key(|(grams, ..)| *grams)?;
        Some(crate::cooking::food_name(source, key))
    }

    /// What one weighs: its portions' grams, water a gram a ml.
    pub fn grams(&self) -> u32 {
        (self.portions.iter())
            .map(|portion| match portion {
                Portion::Food { grams, .. } => *grams,
                Portion::Water { ml } => *ml,
            })
            .sum::<u32>()
            .max(1)
    }

    /// The cells one takes up: plates, bowls, pies and cakes 2 × 2, loaves
    /// and other long things 2 × 1, the rest (intermediates too) 1 × 1.
    pub fn shape(&self) -> Shape {
        if self.rest.is_some() {
            return Shape::rectangle(1, 1);
        }
        match self.dish.form {
            DishForm::Bowl | DishForm::Plate | DishForm::Pie | DishForm::Cake => {
                Shape::rectangle(2, 2)
            }
            DishForm::Loaf | DishForm::Flatbread | DishForm::Strips | DishForm::Sausage => {
                Shape::rectangle(2, 1)
            }
            _ => Shape::rectangle(1, 1),
        }
    }

    /// Its diet labels, from what went into it.
    pub fn diet(&self, ingredients: &Ingredients) -> Diet {
        Diet::of(&self.portions, ingredients)
    }

    /// What it brings to a row's source space when a later row starts from
    /// it.
    pub fn carried(&self) -> Carried {
        let mut history = self.history.clone();
        history
            .rests
            .extend(self.rest.as_ref().map(|rest| rest.kind));
        Carried {
            portions: self.portions.clone(),
            history,
        }
    }
}

impl Item {
    /// What it is, if it is a Dish or an intermediate.
    pub fn food(&self) -> Option<&FoodForm> {
        match &self.form {
            Some(ItemForm::Food(food)) => Some(food),
            _ => None,
        }
    }

    /// The same, to change.
    pub fn food_mut(&mut self) -> Option<&mut FoodForm> {
        match &mut self.form {
            Some(ItemForm::Food(food)) => Some(food),
            _ => None,
        }
    }
}
