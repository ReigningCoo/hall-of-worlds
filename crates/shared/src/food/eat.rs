//! Eating a serving of a Dish: Well Fed (`cooking::WellFed`), one at a time,
//! and the poisons any fungi in it carry (the server's, never shown first).

use super::FoodForm;
use crate::bonus::Effect;
use crate::cooking::{Ingredients, WellFed, poisons};
use crate::fungus::{Fungi, FungusDef};
use crate::item::Quality;

/// Well Fed from eating one serving of `form` at `quality`, from `now`.
pub fn eating(form: &FoodForm, quality: Quality, now: u64, ingredients: &Ingredients) -> Effect {
    let well_fed = WellFed::of(&form.portions, form.history.cooked(), ingredients);
    well_fed.effect(quality, now)
}

/// The fungi whose poison eating `form` brings on (`cooking::poisons`).
pub fn food_poisons<'a>(form: &FoodForm, fungi: &'a Fungi) -> Vec<&'a FungusDef> {
    poisons(&form.portions, form.history.cooked(), fungi)
}
