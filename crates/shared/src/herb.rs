//! Herbs: what each one is (`HerbDef`), the table of them (`Herbs`: the ones
//! built in, plus any a server's mods add or replace), herbs as items
//! (`HerbForm`), the quality picking one gives (`pick_quality`), how much of
//! that a user draws out (`potency`), and what a character knows of each
//! (`HerbKnowledge`). Research and the owner's decisions:
//! `docs/context/research-herbs.md`.

mod def;
mod effect;
mod form;
mod growth;
mod knowledge;
mod plant;
mod potency;
mod quality;
mod table;
mod traits;

pub use def::{EFFECTS, HerbDef};
pub use effect::{Effect, Stat};
pub use form::{HerbForm, new_item};
pub use growth::{EARLY_STAGE, Early, Growth, Regrow, STAGES};
pub use knowledge::{HerbKnowledge, KnownHerb};
pub use plant::{PartSize, PlantLook, PlantShape, Rgb};
pub use potency::{HERBALISM, potency, quality_percent, skill_cap};
pub use quality::{FORAGING, HERB_ODDS, pick_odds, pick_quality, skill_value};
pub use table::Herbs;
pub use traits::{Abundance, Part, Temperature};

#[cfg(test)]
mod tests;
