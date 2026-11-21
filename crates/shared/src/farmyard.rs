//! Milk, eggs and honey as items (cooking plan PR 3, `docs/context/cooking.md`):
//! one of each for every kept animal that gives it (`livestock`'s produce).
//! Milk and honey are liquids (`potion::Liquid::Milk`, `Honey`), held by the
//! millilitre in bottles, vessels and casks as water is; eggs are counted
//! items stacking by animal. Nothing gives them yet but shops and Give Item:
//! livestock production comes later (owner, 2026-11-21).

mod animals;
mod egg;
mod names;
mod place;

pub use animals::{dev_animal, givers, rolled_honey, rolled_milk};
pub use egg::{EggForm, new_egg};
pub use names::{egg_name, honey_name, milk_name};
pub use place::animal_for;

#[cfg(test)]
mod tests;
