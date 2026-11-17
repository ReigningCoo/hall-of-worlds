//! `FruitDef`: what one fruit or nut is, as written in the table.

use serde::{Deserialize, Serialize};

use super::{Abundance, FruitLook, HarvestTier, PlantFamily};
use crate::climate::Biome;
use crate::herb::{Effect, Temperature};

/// Fruits, or nuts: the two sets of the table (owner, 2026-11-17).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FruitSet {
    Fruit,
    Nut,
}

/// What one item of a fruit is (owner, 2026-11-17): a unit under 25 kcal is a
/// handful (100 g of fruit, or 20 g of nut kernels), anything else one fruit,
/// and one over 300 kcal is picked whole and eaten in servings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    One,
    Handful,
    /// One whole fruit, cut into `FruitDef::servings` when eaten.
    Whole,
}

/// One fruit or nut. Saves and messages name it by `key`, never by its place in
/// the table.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FruitDef {
    /// Its id for good: lower case letters, digits and `_` (`sea_buckthorn`).
    pub key: String,
    /// What players call it.
    pub name: String,
    /// Its botanical name, for lore.
    pub latin: String,
    pub set: FruitSet,
    /// A herb of this key bears it (`herb::HerbDef::fruit`): these are its
    /// values when eaten, and it is never a fruit item of its own.
    #[serde(default)]
    pub herb: bool,
    /// Where it grows wild.
    pub biomes: Vec<Biome>,
    /// The yearly average temperatures it grows in.
    pub temperature: Temperature,
    pub unit: Unit,
    /// Food energy in one item (all of a whole fruit), which decides its healing.
    pub kcal: u32,
    /// How many servings a whole fruit is cut into: 1 for anything else.
    #[serde(default = "one")]
    pub servings: u8,
    /// What one item weighs, in grams (a whole fruit all of it).
    pub grams: u32,
    /// Exceptionally good for you: its healing lasts `EXCEPTIONAL` times as long.
    #[serde(default)]
    pub exceptional: bool,
    /// Harmful eaten raw (needs cooking, curing or roasting): no healing, only
    /// its harmful effects.
    #[serde(default)]
    pub harmful_raw: bool,
    /// What eating one does besides healing: 0 to 2 effects, ±1 or 2 each.
    pub effects: Vec<Effect>,
    pub abundance: Abundance,
    /// How many items picking a ripe plant gives, untrained.
    pub harvest: HarvestTier,
    /// The kind of plant it grows on, for the client's models.
    pub plant: PlantFamily,
    /// How the fruit looks, for the client's pictures.
    pub look: FruitLook,
}

fn one() -> u8 {
    1
}

impl FruitDef {
    /// Whether it makes sense: an error naming what is wrong.
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
        if self.biomes.is_empty() {
            return Err(format!("{key}: it grows nowhere"));
        }
        if self.temperature.min > self.temperature.max {
            return Err(format!(
                "{key}: its lowest temperature is above its highest"
            ));
        }
        if self.kcal == 0 || self.grams == 0 {
            return Err(format!("{key}: it has no food or weight"));
        }
        let whole = self.unit == Unit::Whole;
        if whole != (self.servings > 1) || self.servings == 0 {
            return Err(format!(
                "{key}: only a whole fruit has servings, more than one"
            ));
        }
        if self.effects.len() > 2
            || self
                .effects
                .iter()
                .any(|e| !(1..=2).contains(&e.strength()))
        {
            return Err(format!("{key}: at most 2 effects, each 1 or 2 either way"));
        }
        Ok(())
    }

    /// Whether it is a fruit item of its own (not a herb's fruit).
    pub fn is_item(&self) -> bool {
        !self.herb
    }
}
