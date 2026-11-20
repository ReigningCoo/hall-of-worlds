//! `FungusDef`: what one mushroom, fungus or lichen is, as written in the table.

use serde::{Deserialize, Serialize};

use super::{FungusLook, Group, Niche, Poison, Where};
use crate::climate::Biome;
use crate::herb::{self, Growth, Regrow, Rgb, Temperature};
use crate::vegetable::{Abundance, Effect, Raw};

/// What one item of it is (owner, 2026-11-19, decision 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    /// One mushroom, truffle or bracket.
    One,
    /// Small ones, jelly or lichen by the 100 g handful.
    Handful,
    /// A large one (a giant puffball, poria), cut into `FungusDef::servings`
    /// when eaten.
    Piece,
}

/// One fungus. Saves and messages name it by `key`, never by its place in the
/// table.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FungusDef {
    /// Its id for good: lower case letters, digits and `_` (`death_cap`).
    pub key: String,
    /// What players call it.
    pub name: String,
    /// Its scientific name, for lore; empty for an invented one.
    #[serde(default)]
    pub latin: String,
    /// Made up for the game (owner: some invented kinds fill the underground).
    #[serde(default)]
    pub invented: bool,
    pub group: Group,
    #[serde(rename = "where")]
    pub grows: Where,
    /// What it grows on or by, at least one.
    pub niches: Vec<Niche>,
    /// The biomes above it grows in (or whose land it lies under), its home
    /// first: needed above ground, none for invented cave kinds.
    #[serde(default)]
    pub biomes: Vec<Biome>,
    /// The yearly average temperatures it grows in; a cave is at its land's.
    pub temperature: Temperature,
    pub abundance: Abundance,
    pub unit: Unit,
    /// How many servings a piece is cut into: 1 for anything else.
    #[serde(default = "one")]
    pub servings: u8,
    /// What one item weighs, in grams.
    pub grams: u32,
    /// Food energy in one item (all of a piece): its healing. 0 for what
    /// isn't food at all (tinder fungus).
    pub kcal: u32,
    /// How often it is eaten raw: whether it heals raw.
    pub raw: Raw,
    /// What eating one does besides healing: 0 to 2 effects, ±1 to 3, each raw,
    /// cooked or both (as vegetables').
    #[serde(default)]
    pub food: Vec<Effect>,
    /// Its four effects used like a herb (owner, decision 7), or none.
    #[serde(default)]
    pub herb: Vec<herb::Effect>,
    #[serde(default)]
    pub poison: Option<Poison>,
    pub growth: Growth,
    /// What picking does to it.
    pub regrow: Regrow,
    /// The colour of its light, if it glows (fungi plan PR 5).
    #[serde(default)]
    pub glow: Option<Rgb>,
    /// The keys of fungi it is mistaken for (fungi plan PR 7).
    #[serde(default)]
    pub lookalikes: Vec<String>,
    /// What one looks like.
    pub look: FungusLook,
}

fn one() -> u8 {
    1
}

impl FungusDef {
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
        if self.niches.is_empty() || (self.grows == Where::Surface && self.biomes.is_empty()) {
            return Err(format!("{key}: it grows nowhere"));
        }
        if self.temperature.min > self.temperature.max {
            return Err(format!(
                "{key}: its lowest temperature is above its highest"
            ));
        }
        if self.grams == 0 {
            return Err(format!("{key}: it weighs nothing"));
        }
        if self.servings == 0 || (self.servings > 1 && self.unit != Unit::Piece) {
            return Err(format!("{key}: only a piece has servings, more than one"));
        }
        let wrong = |change: i8| !(1..=3).contains(&change.unsigned_abs());
        if self.food.len() > 2 || self.food.iter().any(|e| wrong(e.change)) {
            return Err(format!(
                "{key}: at most 2 food effects, each 1 to 3 either way"
            ));
        }
        let herb = self.herb.len();
        if (herb != 0 && herb != herb::EFFECTS) || self.herb.iter().any(|e| wrong(e.change)) {
            return Err(format!(
                "{key}: none or {} herb effects, each 1 to 3 either way",
                herb::EFFECTS
            ));
        }
        if self.lookalikes.contains(key) {
            return Err(format!("{key}: it looks like itself"));
        }
        if let Some(poison) = &self.poison {
            poison.check().map_err(|error| format!("{key}: {error}"))?;
        }
        self.look.check().map_err(|error| format!("{key}: {error}"))
    }
}
