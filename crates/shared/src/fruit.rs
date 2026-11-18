//! Fruits and nuts: what each one is (`FruitDef`), the table of them
//! (`Fruits`, built in), fruits as items (`FruitForm`; a whole big fruit is cut
//! into servings when eaten, `cut`), and what eating one does (`eating`).
//! Fruit plants (`FruitPlant`): where they grow (wild, very rarely, and in
//! farms' orchards), how they grow and what picking gives. Their models are
//! the client's. Research and the owner's decisions: `docs/context/research-fruit.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod form;
mod growth;
mod harvest;
mod look;
mod orchard;
mod place;
mod plant;
mod plant_look;
mod table;
mod tag;
mod traits;

pub use cut::cut;
pub use def::{FruitDef, FruitSet, Unit};
pub use dev::dev_fruit;
pub(crate) use eating::bonus_of;
pub use eating::{ATTRIBUTE, EXCEPTIONAL, RESTORE, VITAL, eating, healing, ticks};
pub use form::{APPLE, FruitForm, new_item};
pub use growth::GIANTS;
pub use harvest::{ORCHARDS, harvest, harvest_range};
pub use look::{FruitLook, FruitShape, Skin};
pub use orchard::{ROW_GAP, plant_orchards};
pub use place::{choose, orchard_fruits, wild_fruits};
pub use plant::{FruitPlant, FruitStage, LIFE_ROUNDS, round_minutes};
pub use plant_look::{FruitPlantLook, FruitPosition};
pub use table::Fruits;
pub use tag::FruitTag;
pub use traits::{Abundance, HarvestTier, PlantFamily};

#[cfg(test)]
mod climate_tests;
#[cfg(test)]
mod plant_tests;
#[cfg(test)]
mod tests;
