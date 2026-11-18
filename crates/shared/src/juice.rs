//! Juice and cordial (drinks plan PR 3, `docs/context/drinks.md`): fruit
//! pressed into juice with the oil press (**Press Juice**), how much juice a
//! fruit gives by its weight and kcal, cordial poured from juice and water,
//! and what drinking either does. The liquids themselves are
//! `potion::Liquid::Juice` and `Cordial`, held by casks, vessels and bottles.

mod carried;
mod drink;
mod mix;
mod pressing;
mod roll;

pub use carried::{JuiceChoice, with_juice};
pub use drink::{CORDIAL_SECONDS, JUICE_SECONDS, MANA_A_SECOND, drink_effect};
pub use mix::{Mixed, mix};
pub use pressing::{
    JUICE_PRACTICE, PRESSING_GRAMS, juicable, juice_fruits, kcal_per_litre, ml_from, press_ticks,
};
pub use roll::juice_fruit;

#[cfg(test)]
mod tests;
