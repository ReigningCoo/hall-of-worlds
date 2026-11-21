//! Cooking: tools placed in rows of steps, a step able to join two rows, into
//! dishes (plan: `docs/context/cooking.md`). So far only the culinary data of
//! every raw food (`Ingredients`, built in from `ingredients.ron`): its roles
//! in a dish, how it is cooked, its texture, what it does to a dish, how it
//! tastes, the most of a dish it should be and its diet flags. Research:
//! `docs/context/research-cooking.md`.

mod def;
mod table;
mod traits;

pub use def::{IngredientDef, MAX_STRENGTH};
pub use table::Ingredients;
pub use traits::{CookingNeed, Flavour, Function, Method, Role, Source, Texture, Usage};

#[cfg(test)]
mod tests;
