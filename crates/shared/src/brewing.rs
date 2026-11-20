//! The brewing rules (brewing plan, owner, 2026-11-17: `docs/context/brewing.md`,
//! decisions 6 to 16 and 18): herbs and an oil ground into an ointment, an
//! ointment and a water brewed into a potion, a potion's quality and how long
//! it lasts. Pure functions on whole numbers, so client and server agree. Also
//! the ointment item, the carried stacks Alchemy offers, ointment on the skin
//! and the recipes a character remembers; the bottle turned into a potion and
//! what it teaches; and drinking's (PR 8): a potion's effects and the
//! cooldowns. Grind and Brew themselves gave way to `shared::alchemy`.

mod boost;
mod bottles;
mod brew;
mod cooldown;
mod cut;
mod drink;
mod grind;
mod ingredient;
mod lesson;
mod ointment;
mod quality;
mod recipe;
mod skill;
mod skin;
mod stacks;

pub use boost::{boosted, oil_suits, stat_row};
pub use bottles::{OintmentStack, WaterStack, brewed_into};
pub use brew::{KEPT_FROM, POTION_AT, brew, watered};
pub use cooldown::{Combination, Cooldowns, DRINK_COOLDOWN_SECONDS, DRINK_COOLDOWN_TICKS};
pub use cut::cut_unwanted;
pub use drink::{Drunk, drunk};
pub use grind::{Ointment, Refused, grind, ground_doses, herb_steps};
pub use ingredient::{GroundHerb, Ingredient};
pub use lesson::learn_from_potion;
pub use ointment::{OINTMENT_GRAMS, OintmentForm, new_ointment};
pub use quality::{
    ON_SKIN_SHARE, POOR_SECONDS, duration_seconds, duration_ticks, on_skin, purity, quality,
    quality_of_purity,
};
pub use recipe::{BrewedWith, MOST_RECIPES, Recipe, Recipes};
pub use skill::{
    BrewSkills, cut_steps, most_doses, most_kinds, per_herb_cap, square_root, stat_cap,
};
pub use skin::{ATTRIBUTE_PER_STEP, Applied, VITAL_PER_STEP, applied, skin_effect};
pub use stacks::{HerbStack, OilStack};

#[cfg(test)]
mod brew_item_tests;
#[cfg(test)]
mod brew_tests;
#[cfg(test)]
mod drink_tests;
#[cfg(test)]
mod grind_item_tests;
#[cfg(test)]
mod tests;
