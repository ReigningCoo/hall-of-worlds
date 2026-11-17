//! `HerbDef`: what one herb is, as written in the table and in mods.

use serde::{Deserialize, Serialize};

use super::{Abundance, Early, Effect, Growth, Part, PlantLook, Regrow, Temperature};
use crate::climate::Biome;

/// How many effects every herb has.
pub const EFFECTS: usize = 4;

/// One herb. Saves and messages name it by `key`, never by its place in the
/// table, so mods can add and replace herbs freely.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HerbDef {
    /// Its id for good: lower case letters, digits and `_` (`roseroot`).
    pub key: String,
    /// What players call it.
    pub name: String,
    /// Its botanical name, for lore.
    pub latin: String,
    /// Where it grows wild.
    pub biomes: Vec<Biome>,
    /// It also grows in marshes, which the climate doesn't have yet: kept for then.
    #[serde(default)]
    pub marsh: bool,
    pub abundance: Abundance,
    /// The yearly average temperatures it grows in.
    pub temperature: Temperature,
    /// The part of it that is used: decides the item's picture.
    pub part: Part,
    /// That part as written ("Flowering tops").
    pub part_name: String,
    pub growth: Growth,
    /// What harvesting does to the plant.
    pub regrow: Regrow,
    /// What harvesting at `EARLY_STAGE` gives.
    pub early: Early,
    /// What eating it does: four effects (`EFFECTS`), each found out on its own.
    pub effects: Vec<Effect>,
    /// What one weighs, in grams.
    pub grams: u32,
    /// How its plant looks, for the client's models.
    pub plant: PlantLook,
}

impl HerbDef {
    /// Whether it makes sense: an error naming what is wrong, for a mod's author.
    pub fn check(&self) -> Result<(), String> {
        let key = &self.key;
        let plain = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_';
        if key.is_empty() || !key.chars().all(plain) {
            return Err(format!(
                "{key:?}: a key is lower case letters, digits and _"
            ));
        }
        if self.name.trim().is_empty() {
            return Err(format!("{key}: it has no name"));
        }
        if self.biomes.is_empty() && !self.marsh {
            return Err(format!("{key}: it grows nowhere"));
        }
        if self.temperature.min > self.temperature.max {
            return Err(format!(
                "{key}: its lowest temperature is above its highest"
            ));
        }
        if self.effects.len() != EFFECTS {
            return Err(format!(
                "{key}: it has {} effects, not {EFFECTS}",
                self.effects.len()
            ));
        }
        let wrong = self
            .effects
            .iter()
            .find(|e| !(1..=3).contains(&e.strength()));
        if let Some(effect) = wrong {
            return Err(format!(
                "{key}: {:?} changes by {}, not 1 to 3 either way",
                effect.stat, effect.change
            ));
        }
        if self.grams == 0 {
            return Err(format!("{key}: it weighs nothing"));
        }
        if self.early.taken > 100 || self.early.strength > 100 {
            return Err(format!("{key}: early harvests are percentages up to 100"));
        }
        Ok(())
    }
}
