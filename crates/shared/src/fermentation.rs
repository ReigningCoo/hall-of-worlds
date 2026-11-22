//! The fermentation rules (drinks plan PR 5, `docs/context/drinks.md`,
//! owner's decisions 16 to 18): pure functions, whole numbers, no clocks or
//! items of their own. What ferments (`Mash`: juice, cordial, fruit and water,
//! or grain and water) and what it becomes (`Base`, `DrinkKind`: ale, cider,
//! perry, wine, "<fruit> wine"); how strong and sweet it ends up for the yeast
//! used; how long it takes in a cask; its quality from the yeast, what went in
//! and the wood, and ageing in the cask; and the opened cask's best-before
//! until it turns to vinegar. Ferment itself (PR 6) applies them.
//!
//! Strengths are in tenths of a percent of alcohol by volume (`55` is 5.5%),
//! times in real seconds (one game day is one real hour, `GAME_DAY_SECS`).

mod ageing;
mod base;
mod drink;
mod mash;
mod quality;
mod roll;
mod spoil;
mod strength;
mod time;

pub use ageing::{aged, ageing_cap, ageing_step_secs, glass_aged, glass_cap, glass_step_secs};
pub use base::{Base, DrinkKind};
pub use drink::Drink;
pub use mash::{FRUIT_SUGAR, GRAIN_SUGAR, Mash, MashSource};
pub use quality::{drink_quality, mean_quality};
pub use roll::{rolled_drink, rolled_vinegar};
pub use spoil::{
    JUICE_TURNS_SECS, KEEPS_FROM, best_before_secs, notice_due, vinegar_share, wild_cider,
};
pub use strength::{
    Fermented, KCAL_PER_PERCENT, LEAST_STRENGTH, Sweetness, WILD_TOLERANCE, best_tolerance,
    ferment, tolerance,
};
pub use time::{GAME_DAY_SECS, PITCH_ML, base_secs, ferment_secs};

#[cfg(test)]
mod tests;
