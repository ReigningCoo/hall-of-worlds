//! What a cooked food's model shows: its model by form or by kind of
//! intermediate, and its colours from what went into it.

use serde::Deserialize;
use shared::cooking::{DishForm, Ingredients};
use shared::food::{Rest, accent, tint};
use shared::herb::Rgb;
use shared::item::Item;

/// Which model a cooked food has.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Model {
    /// A Dish, by what it is served as.
    Dish(DishForm),
    /// An intermediate, by its kind.
    Rest(Rest),
}

/// How a cooked food looks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DishLook {
    pub model: Model,
    /// Its foods' colour (`food::tint`).
    pub tint: Rgb,
    /// The second colour, for pieces, toppings and fillings (`food::accent`),
    /// or the tint again when its foods share one colour.
    pub accent: Rgb,
    /// Its cells, as `FoodForm::shape` has them.
    pub cells: (u8, u8),
}

impl DishLook {
    /// A Dish's or an intermediate's look, or `None` for anything else.
    pub fn of(item: &Item) -> Option<Self> {
        let food = item.food()?;
        let ingredients = Ingredients::built_in();
        let main = tint(&food.portions, &food.history, ingredients);
        let second = accent(&food.portions, &food.history, ingredients).unwrap_or(main);
        let model = match &food.rest {
            Some(clock) => Model::Rest(clock.kind),
            None => Model::Dish(food.dish.form),
        };
        let shape = food.shape();
        Some(Self {
            model,
            tint: Rgb(main[0], main[1], main[2]),
            accent: Rgb(second[0], second[1], second[2]),
            cells: (shape.width, shape.height),
        })
    }

    /// A look in these colours, for samples and tests: its cells as
    /// `FoodForm::shape` gives that model.
    #[cfg_attr(not(any(test, feature = "debug-api")), allow(dead_code))]
    pub fn sample(model: Model, tint: Rgb, accent: Rgb) -> Self {
        let cells = match model {
            Model::Rest(_) => (1, 1),
            Model::Dish(DishForm::Bowl | DishForm::Plate | DishForm::Pie | DishForm::Cake) => {
                (2, 2)
            }
            Model::Dish(
                DishForm::Loaf | DishForm::Flatbread | DishForm::Strips | DishForm::Sausage,
            ) => (2, 1),
            Model::Dish(_) => (1, 1),
        };
        Self {
            model,
            tint,
            accent,
            cells,
        }
    }
}
