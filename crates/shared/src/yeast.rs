//! Yeast (drinks plan, PR 4; owner's decisions 26 to 31 in
//! `docs/context/drinks.md`): bought from shops, nearly always Common. Each
//! yeast is its own item (`ItemKind::Yeast`, not counted), so a stack keeps
//! each one's clock and progress. Unfed, it loses a quality step every 24
//! hours (Poor stays Poor), counted in playtime while carried and in real time
//! while it lies on the ground. Feeding with flour and water (PR 8) resets the
//! clock: early it splits, on time it gains progress towards a better quality;
//! between feeds a yeast can be used in one thing (PR 6) without being used
//! up. A starter can be begun from flour and a bottle of fresh water. Cultures
//! for cooking are strains of it (`Strain`: yoghurt, koji).

mod clock;
mod feed;
mod feed_stack;
mod form;
mod shop;
mod starter;
mod strain;
mod supplies;

pub use clock::{SECONDS_PER_HOUR, Settled};
pub use feed::{BEST_BY_FEEDING, FEED_PRACTICE, Fed, Window, feed, feed_points, step_up_chance};
pub use feed_stack::{FeedPlan, FeedRefused, Supplies, plan_feeding};
pub use form::{FEED_AFTER, WEAKENS_EVERY, YeastForm, new_item};
pub use shop::shop_quality;
pub use starter::{STARTER_PRACTICE, STARTS_IN, StartRefused, can_start, new_starter};
pub use strain::Strain;
pub use supplies::{WATER_ML, drawn, fresh, water_portions};

#[cfg(test)]
mod feed_tests;
#[cfg(test)]
mod tests;
