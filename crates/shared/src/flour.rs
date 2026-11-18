//! Flour: a grain's sheaf milled into a sack of flour (`FlourForm`), one flour
//! per grain, weighing the sheaf's grams times the grain's `flour_yield`. The
//! sack is only its picture and name: the whole item is used up together.
//! Counted, no quality, and it never goes stale (owner, 2026-11-18). Milling is
//! trade with a town's miller: `docs/context/milling.md`.

mod dev;
mod form;

pub use dev::{dev_flour, grains};
pub use form::{FlourForm, new_item};

#[cfg(test)]
mod tests;
