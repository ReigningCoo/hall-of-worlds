//! Herbs: what each one is (`HerbDef`), the table of them (`Herbs`: the ones
//! built in, plus any a server's mods add or replace), herbs as items
//! (`HerbForm`, as good as the plant is rare), how many picking gives
//! (`harvest`), how much of its quality a user draws out (`potency`), and what a character knows of each
//! (`HerbKnowledge`), and the weak dose of eating one raw (`raw_dose`). Research and the owner's decisions:
//! `docs/context/research-herbs.md`.

mod beds;
mod def;
mod dev;
mod eating;
mod effect;
mod form;
mod growth;
mod harvest;
mod knowledge;
mod patch;
mod place;
mod plant;
mod potency;
mod raw;
mod settle;
mod skills;
mod table;
mod traits;

pub use beds::plant_beds;
pub use def::{EFFECTS, HerbDef};
pub use dev::dev_herb;
pub use eating::{RAW_DOSE_WITH_FRUIT, eating_herb};
pub use effect::{Effect, Stat};
pub use form::{HerbForm, new_item};
pub use growth::{EARLY_STAGE, Early, Growth, Regrow, STAGES};
pub use harvest::{FULL_HARVEST, Harvest, harvest, harvest_range};
pub use knowledge::{HerbKnowledge, KnownHerb};
pub use patch::{HerbPatch, MINUTES_PER_DAY, grown_stage, growth_stage_minutes, stage_minutes};
pub use place::{MARSH_STAND_INS, bed_herbs, choose, wild_herbs};
pub use plant::{PartSize, PlantLook, PlantShape, Rgb};
pub use potency::{HERBALISM, potency, quality_percent, skill_cap};
pub use raw::{RAW_ATTRIBUTE, RAW_TICKS, RAW_VITAL, raw_dose};
pub use settle::settle_herbs;
pub use skills::{FORAGING, HERBS, cube_root, skill_percent, skill_value};
pub use table::Herbs;
pub use traits::{Abundance, Part, Temperature};

#[cfg(test)]
pub(crate) mod climate_tests;
#[cfg(test)]
mod raw_tests;
#[cfg(test)]
mod tests;
