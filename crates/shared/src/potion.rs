//! Potions in bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`,
//! empty or holding `Contents` (effects on `herb::Stat`s, each a signed step),
//! with a look seed the client picks the bottle's shape, glass and stopper
//! from. Bottles stack by contents and quality, whatever they look like.
//! Drinking and brewing are still to come. See `docs/context/shared/potion.md`.

mod contents;
mod form;
mod roll;

pub use contents::{Contents, Dose};
pub use form::{BottleForm, EMPTY_GRAMS, POTION_GRAMS};
pub use roll::{MOST_EFFECTS, roll_contents};

#[cfg(test)]
mod tests;
