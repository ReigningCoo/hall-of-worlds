//! The brewing rules (brewing plan, owner, 2026-11-17: `docs/context/brewing.md`,
//! decisions 6 to 16 and 18): herbs and an oil ground into an ointment, an
//! ointment and a water brewed into a potion, a potion's quality and how long
//! it lasts. Pure functions on whole numbers, so client and server agree. Also
//! Grind's pieces (PR 6): the ointment item, the order checked against what is
//! carried, ointment on the skin and the recipes a character remembers; and
//! Brew's (PR 7): the order, the bottle turned into a potion, what it teaches;
//! and drinking's (PR 8): a potion's effects and the cooldowns.

mod boost;
mod brew;
mod brew_order;
mod cooldown;
mod cut;
mod drink;
mod grind;
mod lesson;
mod ointment;
mod order;
mod quality;
mod recipe;
mod skill;
mod skin;

pub use boost::{boosted, oil_suits, stat_row};
pub use brew::{KEPT_FROM, POTION_AT, brew, watered};
pub use brew_order::{
    BREW_HERB_PRACTICE, BREW_PRACTICE, BrewOrder, BrewRefused, Brewed, OintmentStack, WaterStack,
    brew_seconds, brew_ticks, brewed_into,
};
pub use cooldown::{Combination, Cooldowns, DRINK_COOLDOWN_SECONDS, DRINK_COOLDOWN_TICKS};
pub use cut::cut_unwanted;
pub use drink::{Drunk, drunk};
pub use grind::{GroundHerb, Ointment, Refused, grind, herb_steps};
pub use lesson::learn_from_potion;
pub use ointment::{OINTMENT_GRAMS, OintmentForm, new_ointment};
pub use order::{
    GRIND_PRACTICE, GrindOrder, GrindRefused, HERB_PRACTICE, HerbStack, OilStack, grind_seconds,
    grind_ticks,
};
pub use quality::{
    COMMON_SECONDS, ON_SKIN_SHARE, SECONDS_PER_QUALITY, duration_seconds, duration_ticks, on_skin,
    quality,
};
pub use recipe::{BrewedWith, MOST_RECIPES, Recipe, Recipes};
pub use skill::{
    BrewSkills, cut_steps, most_doses, most_kinds, per_herb_cap, square_root, stat_cap,
};
pub use skin::{ATTRIBUTE_PER_STEP, Applied, VITAL_PER_STEP, applied, skin_effect};

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
