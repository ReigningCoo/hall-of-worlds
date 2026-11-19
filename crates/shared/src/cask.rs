//! Casks (drinks plan PR 1, `docs/context/drinks.md`): wooden casks in eight
//! sizes from the English brewers' (Pin to Tun) and five woods, holding one
//! liquid (`shared::potion`'s) by the litre. One item kind, `ItemKind::Cask`,
//! not counted; its form says its size, wood and what is in it. Collect Water
//! fills one by the litre once no empty bottle is left (`fill.rs`).
//! See `docs/context/shared/cask.md`.

mod fill;
mod form;
mod roll;
mod size;
mod time;

pub use fill::{CASK_FILL_SECONDS, CASK_PRACTICE, cask_fill_ticks, cask_litres, fill_cask_with};
pub use form::{CaskForm, CaskLook, WOODS};
pub use roll::{cask_to, cask_wood_weights};
pub use size::CaskSize;
pub use time::CaskTime;

#[cfg(test)]
mod tests;
