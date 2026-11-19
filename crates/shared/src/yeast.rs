//! Yeast (drinks plan, PR 4; owner's decisions 26 to 31 in
//! `docs/context/drinks.md`): bought from shops, nearly always Common. Each
//! yeast is its own item (`ItemKind::Yeast`, not counted), so a stack keeps
//! each one's clock and progress. Unfed, it loses a quality step every 24
//! hours (Poor stays Poor), counted in playtime while carried and in real time
//! while it lies on the ground. Feeding (PR 9) resets the clock; between feeds
//! a yeast can be used in one thing (PR 6) without being used up.

mod clock;
mod form;
mod shop;

pub use clock::{SECONDS_PER_HOUR, Settled};
pub use form::{FEED_AFTER, WEAKENS_EVERY, YeastForm, new_item};
pub use shop::shop_quality;

#[cfg(test)]
mod tests;
