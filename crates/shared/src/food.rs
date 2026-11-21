//! Food items (cooking plan PR 6, `docs/context/cooking.md`): what cooking
//! makes. A Dish (`ItemKind::Dish`) is real food in servings, eaten anywhere
//! for Well Fed; an intermediate (`ItemKind::Intermediate`: dough, noodles,
//! roux, stock, butter, fresh cheese, warm milk, yoghurt) is a row stopped
//! early, to start a later row from a source space. Both keep what went into
//! them and their steps (`FoodForm`). Intermediates rest on a clock run by
//! warmth (`RestClock`, `warmth_percent`): stable, proving (better, then
//! worse) or turning into another (fresh noodles drying, warm milk setting
//! into yoghurt, wild or with a culture: `Culture`), or ageing better (the
//! long processes: koji, moromi pressed into soy sauce, miso, cheese).

mod clock;
mod culture;
mod curve;
mod eat;
mod form;
mod give;
mod long;
mod make;
mod rest;
mod serving;
mod tint;
mod warmth;

pub use clock::{RestClock, Rested};
pub use culture::{Culture, culture_from, set_percent, turned_quality};
pub use curve::Curve;
pub use eat::{eating, food_poisons};
pub use form::FoodForm;
pub use give::dev_food;
pub use long::{SOY_GRAMS_PER_BOTTLE, soy_sauce_bottles};
pub use make::food_item;
pub use rest::{KOJI_GROWS, Rest, SALTED_PERCENT};
pub use serving::{SERVING_GRAMS, servings};
pub use tint::{accent, tint};
pub use warmth::{CARRIED_CELSIUS, warmth_percent};

#[cfg(test)]
mod long_tests;
#[cfg(test)]
mod tests;
