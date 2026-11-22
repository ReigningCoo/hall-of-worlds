//! The lantern (owner, 2026-11-21): a fount of oil burning by playtime while
//! held in a hand or tied to the belt, lighting further than the light every
//! player carries. The rules both sides share: the fuel kept on the item, how
//! it burns, refilling from a bottle of oil, and how far a carrier's light
//! reaches. The server keeps the clock and tells viewers (lantern PR 5); the
//! client draws it (PR 6). See `docs/context/shared/lantern.md`.

mod burn;
mod fill;
mod form;
mod light;

pub use burn::Burnt;
pub use fill::{FillRefused, Filled, fill};
pub use form::{FOUNT_ML, LanternForm, SECONDS_PER_ML};
pub use light::LANTERN_LIGHT;

#[cfg(test)]
mod tests;
