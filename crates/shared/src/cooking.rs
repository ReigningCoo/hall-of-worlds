//! Cooking: tools placed in rows of steps, a step able to join two rows, into
//! dishes (plan: `docs/context/cooking.md`). The culinary data of every raw
//! food (`Ingredients`, built in from `ingredients.ron`): its roles in a
//! dish, how it is cooked, its texture, what it does to a dish, how it
//! tastes, the most of a dish it should be and its diet flags. And the rules
//! of a process (plan PR 4): the grid of rows and steps, tools and their
//! sub-skills, chance, balance, quality and heat, as whole numbers so the
//! client's preview matches the server. Research:
//! `docs/context/research-cooking.md`.

mod balance;
mod chance;
mod def;
mod grid;
mod outcome;
mod plan;
mod process;
mod quality;
mod skills;
mod table;
mod tool;
mod traits;

pub use balance::{
    BALANCED_ROLES, EXCESS_STEP, ROLE_PERCENT, ROLES_COUNTED, SEASONING_PERCENT, balance,
    is_seasoning,
};
pub use chance::chance;
pub use def::{IngredientDef, MAX_STRENGTH};
pub use outcome::{Outcome, outcome};
pub use plan::{Output, Planned, plan};
pub use process::{
    Carried, Heat, History, MOST_COLUMNS, MOST_ROWS, Portion, Process, Refused, Step,
};
pub use quality::{AWAY_CAP, capped, ingredient_quality, least_quality, skill_steps};
pub use skills::CookingSkills;
pub use table::Ingredients;
pub use tool::{BAKING, BOILING, COOKING, CUTTING, FRYING, MIXING, POUNDING, STRAINING, Tool};
pub use traits::{CookingNeed, Flavour, Function, Method, Role, Source, Texture, Usage};

#[cfg(test)]
mod grid_tests;
#[cfg(test)]
mod rules_tests;
#[cfg(test)]
mod tests;
