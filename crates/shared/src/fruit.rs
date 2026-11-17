//! Fruits and nuts: what each one is (`FruitDef`), the table of them
//! (`Fruits`, built in), fruits as items (`FruitForm`; a whole big fruit is cut
//! into servings when eaten, `cut`), and what eating one does (`eating`).
//! Where they grow, growth and picking, and their pictures and models come in
//! later PRs. Research and the owner's decisions: `docs/context/research-fruit.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod form;
mod look;
mod table;
mod tag;
mod traits;

pub use cut::cut;
pub use def::{FruitDef, FruitSet, Unit};
pub use dev::dev_fruit;
pub use eating::{ATTRIBUTE, EXCEPTIONAL, RESTORE, VITAL, eating, healing, ticks};
pub use form::{APPLE, FruitForm, new_item};
pub use look::{FruitLook, FruitShape, Skin};
pub use table::Fruits;
pub use tag::FruitTag;
pub use traits::{Abundance, HarvestTier, PlantFamily};

#[cfg(test)]
mod climate_tests;
#[cfg(test)]
mod tests;
