//! Cooking: tools placed in rows of steps, a step able to join two rows, into
//! dishes (plan: `docs/context/cooking.md`). The culinary data of every raw
//! food (`Ingredients`, built in from `ingredients.ron`): its roles in a
//! dish, how it is cooked, its texture, what it does to a dish, how it
//! tastes, the most of a dish it should be and its diet flags. And the rules
//! of a process (plan PR 4): the grid of rows and steps, tools and their
//! sub-skills, chance, balance, quality and heat, as whole numbers so the
//! client's preview matches the server. And what it makes (plan PR 5): each
//! output named after the closest real dish (`Dishes`, built in from
//! `dishes.ron`) or generically, its diet labels, Well Fed, and the poisons
//! fungi carry into it. Research:
//! `docs/context/research-cooking.md`.

mod balance;
mod chance;
mod classify;
mod def;
mod diet;
mod dish;
mod dishes;
mod generic;
mod grid;
mod mix;
mod names;
mod order;
mod order_check;
mod outcome;
mod plan;
mod poisons;
mod process;
mod quality;
mod run;
mod shape;
mod skills;
mod table;
mod tool;
mod traits;
mod well_fed;

pub use balance::{BALANCED_ROLES, EXCESS_STEP, ROLES_COUNTED, Wants, balance, is_seasoning};
pub use chance::chance;
pub use classify::{Dish, DishName, NAMED_FOODS, classify};
pub use def::{IngredientDef, MAX_STRENGTH};
pub use diet::Diet;
pub use dish::{DishDef, DishForm, Wanted, Wetness};
pub use dishes::Dishes;
pub use generic::{DOUGH_PERCENT, Generic};
pub use mix::{ROLE_PERCENT, SEASONING_PERCENT};
pub use names::food_name;
pub use order::{CookOrder, Kitchen, MOST_REPEATS, OrderInput, OrderRefused, OrderStep};
pub use order_check::Checked;
pub use outcome::{Outcome, outcome};
pub use plan::{Output, Planned, plan};
pub use poisons::poisons;
pub use process::{
    Carried, Heat, History, MOST_COLUMNS, MOST_ROWS, Portion, Process, Refused, Step,
};
pub use quality::{AWAY_CAP, capped, ingredient_quality, least_quality, skill_steps};
pub use run::{COOK_PRACTICE, drawn, heat_seconds, run_seconds, run_ticks};
pub use shape::Shape;
pub use skills::CookingSkills;
pub use table::Ingredients;
pub use tool::{BAKING, BOILING, COOKING, CUTTING, FRYING, MIXING, POUNDING, STRAINING, Tool};
pub use traits::{CookingNeed, Flavour, Function, Method, Role, Source, Texture, Usage};
pub use well_fed::{
    MOST_CHANGE, WELL_FED_MINUTES, WELL_FED_RESTORE, WELL_FED_STEP_MINUTES, WellFed, well_fed_ticks,
};

#[cfg(test)]
mod dish_tests;
#[cfg(test)]
mod grid_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod rules_tests;
#[cfg(test)]
mod tests;
