//! `VegetableDef`: what one vegetable, pulse or grain is, as written in the table.

use serde::{Deserialize, Serialize};

use super::{Abundance, Effect, Part, PlantShape, Raw, VegetableLook, VegetablePlantLook};
use crate::climate::Biome;
use crate::fruit::HarvestTier;
use crate::herb::{Growth, Regrow};

/// The three sets of the table (owner, 2026-11-17).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VegetableSet {
    Vegetable,
    Pulse,
    /// Picked as a sheaf, which heals nothing until milled (later).
    Grain,
}

/// What one item of it is (owner, 2026-11-17).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    /// One root, bulb, head or fruit.
    One,
    /// Leaves, shoots, small roots or fresh pods by the 100 g handful; dry
    /// pulses by the 30 g handful.
    Handful,
    /// A sheaf of grain: 30 g of grain.
    Sheaf,
    /// One whole big one, cut into `VegetableDef::servings` when eaten.
    Whole,
    /// A piece cut from a huge one (agave heart, yam), cut again into
    /// `VegetableDef::servings` when eaten.
    Piece,
    /// One cluster of roots (skirret).
    Cluster,
}

/// One vegetable, pulse or grain. Saves and messages name it by `key`, never
/// by its place in the table.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VegetableDef {
    /// Its id for good: lower case letters, digits and `_` (`sweet_potato`).
    pub key: String,
    /// What players call it.
    pub name: String,
    /// Its botanical name, for lore.
    pub latin: String,
    pub set: VegetableSet,
    /// Only ever gathered wild, never farmed.
    #[serde(default)]
    pub wild_only: bool,
    /// Where it grows, its home first.
    pub biomes: Vec<Biome>,
    pub unit: Unit,
    /// Food energy in one item (all of a whole one), which decides its healing.
    pub kcal: u32,
    /// How many servings a whole one or piece is cut into: 1 for anything else.
    #[serde(default = "one")]
    pub servings: u8,
    /// What one item weighs, in grams.
    pub grams: u32,
    /// How often it is eaten raw: whether it heals raw.
    pub raw: Raw,
    /// Exceptionally good for you: cooked (later), it heals
    /// `fruit::EXCEPTIONAL` times as long (`cooked_ticks`). Not raw.
    #[serde(default)]
    pub exceptional: bool,
    /// What eating one does besides healing: 0 to 2 effects, ±1 to 3, each raw,
    /// cooked or both.
    pub effects: Vec<Effect>,
    /// How common it is wild.
    pub abundance: Abundance,
    pub growth: Growth,
    /// What picking does to the plant.
    pub regrow: Regrow,
    /// How many items picking a ripe plant gives, untrained.
    pub harvest: HarvestTier,
    /// The kind of plant it grows on, for the client's models.
    pub plant: PlantShape,
    /// How its plant looks, for the client's models.
    pub plant_look: VegetablePlantLook,
    /// The part picked, for the client's pictures.
    pub part: Part,
    /// What one item of it looks like.
    pub look: VegetableLook,
}

fn one() -> u8 {
    1
}

impl VegetableDef {
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
        if self.kcal == 0 || self.grams == 0 {
            return Err(format!("{key}: it has no food or weight"));
        }
        let cut = matches!(self.unit, Unit::Whole | Unit::Piece);
        if self.servings == 0 || (self.servings > 1 && !cut) {
            return Err(format!(
                "{key}: only a whole one or a piece has servings, more than one"
            ));
        }
        if (self.set == VegetableSet::Grain) != (self.unit == Unit::Sheaf) {
            return Err(format!("{key}: grain comes as a sheaf, and only grain"));
        }
        if self.effects.len() > 2
            || self
                .effects
                .iter()
                .any(|e| !(1..=3).contains(&e.change.unsigned_abs()))
        {
            return Err(format!("{key}: at most 2 effects, each 1 to 3 either way"));
        }
        self.plant_look
            .check()
            .map_err(|error| format!("{key}: {error}"))?;
        self.look.check().map_err(|error| format!("{key}: {error}"))
    }
}
