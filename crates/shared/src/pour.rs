//! Pouring (drinks plan PR 2, `docs/context/drinks.md`): one rule for every
//! container of a liquid, casks (`shared::cask`), vessels (`shared::vessel`)
//! and bottles (`shared::potion`), carried or lying in reach. As much as there
//! is goes, up to what the other takes; never two liquids together, never a
//! potion; drawing from a cask needs a tap carried. A cask opens when poured
//! from or into; a wine bottle or bottle filled from a cask starts sealed.
//! The server decides (`ClientMessage::Pour`); the client aims by the same rule.
//! See `docs/context/shared/pour.md`.

mod held;
mod in_place;
mod rule;

pub use held::{BOTTLE_ML, Held};
pub use rule::{PourRefused, Poured, pour, pour_cost};

#[cfg(test)]
mod tests;
