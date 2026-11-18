//! Vegetables, pulses and grains: what each one is (`VegetableDef`), the table
//! of them (`Vegetables`, built in), vegetables as items (`VegetableForm`; a
//! whole big one is cut into servings when eaten, `cut`), and what eating one
//! raw does (`eating`). Cooking and milling come later; their figures are kept
//! (`cooked_ticks`). Each has a look (`VegetableLook`: colours, size) for its
//! pictures. Where they grow (wild, `wilderness::vegetables`; farms' fields,
//! `plant_fields`), how they grow (`VegetablePlant`) and what picking gives
//! (`harvest`); each plant's look (`VegetablePlantLook`) for the client's models. Research and the owner's decisions:
//! `docs/context/research-vegetables.md`.

mod cut;
mod def;
mod dev;
mod eating;
mod effect;
mod fields;
mod form;
mod harvest;
mod look;
mod place;
mod plant;
mod plant_look;
mod table;
mod tag;
mod traits;

pub use cut::cut;
pub use def::{Unit, VegetableDef, VegetableSet};
pub use dev::dev_vegetable;
pub use eating::{RAW_SHARE, cooked_ticks, eating, effect_ticks, healing, raw_ticks};
pub use effect::{Effect, When};
pub use fields::plant_fields;
pub use form::{VegetableForm, new_item};
pub use harvest::{CROPS, harvest, harvest_range};
pub use look::{MOST_COLOURS, VegetableLook};
pub use place::{choose_wild, field_crops, wild_crops, wild_only_vegetables};
pub use plant::VegetablePlant;
pub use plant_look::VegetablePlantLook;
pub use table::Vegetables;
pub use tag::VegetableTag;
pub use traits::{Abundance, Part, PlantShape, Raw};

#[cfg(test)]
mod plant_tests;
#[cfg(test)]
mod tests;
