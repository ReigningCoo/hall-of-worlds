//! Bottles (owner, 2026-11-17): one item kind, `ItemKind::Bottle`, empty or
//! holding a `Liquid`: a water, an oil, or a potion's `Contents` (effects on
//! `herb::Stat`s, each a signed step); with a look seed the client picks the
//! bottle's shape, glass and stopper from. Bottles stack by liquid, potions
//! also by quality, whatever they look like. Drinking and brewing are still
//! to come (`docs/context/brewing.md`). See `docs/context/shared/potion.md`.

mod contents;
mod form;
mod liquid;
mod roll;

pub use contents::{Contents, Dose};
pub use form::{BottleForm, EMPTY_GRAMS, POTION_GRAMS};
pub use liquid::{Liquid, LiquidKind, Oil, Water};
pub use roll::{MOST_EFFECTS, bottle_to, roll_contents};

#[cfg(test)]
mod liquid_tests;
#[cfg(test)]
mod tests;
