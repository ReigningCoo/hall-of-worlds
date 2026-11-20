//! Livestock: the 26 animals people keep (owner's research, 2026-11-19), as a
//! table of data with lookups and no gameplay yet. Each row (`LivestockDef`)
//! says where it is kept, its wild form and feral herds, what it eats and how
//! it feels about water, what it gives while alive at game rates (`Produce`),
//! its life stages (`Stages`), its meat by quality (`Meat`, cut names chosen
//! by hash, carcass shares), its hide (`Hide`) and whether it is a pet. Organs,
//! fats and other parts (`Parts`) come from their own table, naming the
//! animals they come from. The plan: livestock PRs 1 to 8 (this is PR 1);
//! research and the owner's decisions: `docs/context/research-livestock.md`.

mod def;
mod drop;
mod hide;
mod meat;
mod parts;
mod parts_table;
mod produce;
mod stages;
mod table;
mod traits;

pub use def::{LivestockDef, Pet, Wild};
pub use drop::{OUT_OF, organ_chance};
pub use hide::{Cure, HIDE_ROTS_AFTER_SECONDS, Hide, YoungHide};
pub use meat::{ByQuality, Meat, SHARES, YOUNG_YIELD_PERCENT};
pub use parts::{Animals, Fat, Only, Organ, Part};
pub use parts_table::Parts;
pub use produce::{Per, Produce, Product};
pub use stages::{Adult, GAME_HOURS_A_MONTH, Sexed, Stage, Stages};
pub use table::Livestock;
pub use traits::{Class, Diet, Often, Rarity, Water};

#[cfg(test)]
mod tests;
