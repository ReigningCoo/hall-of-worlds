//! Vegetables, pulses and grains: what each one is (`VegetableDef`), the table
//! of them (`Vegetables`, built in), vegetables as items (`VegetableForm`; a
//! whole big one is cut into servings when eaten, `cut`), and what eating one
//! raw does (`eating`). Cooking and milling come later; their figures are kept
//! (`cooked_ticks`). Where they grow, their pictures and plant models are later
//! PRs of the vegetable plan. Research and the owner's decisions:
//! `docs/context/research-vegetables.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod effect;
mod form;
mod table;
mod tag;
mod traits;

pub use cut::cut;
pub use def::{Unit, VegetableDef, VegetableSet};
pub use dev::dev_vegetable;
pub use eating::{RAW_SHARE, cooked_ticks, eating, effect_ticks, healing, raw_ticks};
pub use effect::{Effect, When};
pub use form::{VegetableForm, new_item};
pub use table::Vegetables;
pub use tag::VegetableTag;
pub use traits::{Abundance, Part, PlantShape, Raw};

#[cfg(test)]
mod tests;
