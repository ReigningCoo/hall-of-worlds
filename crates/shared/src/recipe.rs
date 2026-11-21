//! Recipes (recipes plan, `docs/context/recipes.md`): recipe scrolls, one
//! for every real dish, and each character's recipe book holding the
//! recipes read from scrolls (learnt) and every process they cooked well
//! (their own). Craft-agnostic: Alchemy's recipes come later (`Craft`).
//! A cooking recipe itself is `cooking::Recipe`.

mod book;
mod craft;
mod form;

pub use book::{Entry, MOST_OWN, Own, RecipeBook, Refused, Removed};
pub use craft::Craft;
pub use form::{RECIPE_GRAMS, RecipeForm, dev_recipe, new_item};

#[cfg(test)]
mod tests;
