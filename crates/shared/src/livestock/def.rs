//! `LivestockDef`: what one kept animal is, as written in the table.

use serde::{Deserialize, Serialize};

use super::{Class, Diet, Hide, Meat, Often, Place, Produce, Rarity, Stages, Warmth, Water};
use crate::climate::Biome;
use crate::creature::CreatureKind;

/// An animal's wild form, or its feral herds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wild {
    /// What it is called in the wild ("aurochs", "mouflon"); empty for feral
    /// herds of the kept animal.
    #[serde(default)]
    pub name: String,
    pub rarity: Rarity,
    /// Where it lives, its home first.
    #[serde(default)]
    pub biomes: Vec<Biome>,
    /// Where within them (mountains, by water, islands).
    #[serde(default)]
    pub note: String,
    /// Where within them it is found, for wild spawns: anywhere unless given.
    #[serde(default)]
    pub place: Place,
    /// How warm it must be there: any unless given.
    #[serde(default)]
    pub warmth: Warmth,
}

/// How often it is kept as a pet, and how.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pet {
    pub often: Often,
    #[serde(default)]
    pub note: String,
}

/// One kept animal. Saves and messages name it by `key`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivestockDef {
    /// Its id for good: lower case letters, digits and `_` (`water_buffalo`).
    pub key: String,
    pub name: String,
    pub latin: String,
    pub class: Class,
    /// Chews the cud (cattle to reindeer, camels and llamas too): has tripe.
    #[serde(default)]
    pub ruminant: bool,
    /// The creature kind that is this animal, once there is one.
    #[serde(default)]
    pub kind: Option<CreatureKind>,
    /// The animal this is a variant of (the Coo of cattle, the Muscovy of
    /// the duck; decision 13): its own row and kind all the same.
    #[serde(default)]
    pub variant_of: Option<String>,
    /// The biomes it is kept in, its home first.
    pub kept: Vec<Biome>,
    #[serde(default)]
    pub kept_note: String,
    /// Its wild form, if it has one (none for a hybrid or the silkworm).
    #[serde(default)]
    pub wild: Option<Wild>,
    pub feral: Wild,
    pub diet: Diet,
    /// What it likes best to eat, as words for now.
    pub favourites: Vec<String>,
    pub water: Water,
    /// Wallowing, dust baths, drinking.
    #[serde(default)]
    pub water_note: String,
    /// What it gives alive, at game rates; none for a pig or a mule.
    #[serde(default)]
    pub produce: Vec<Produce>,
    /// Work it does and what else it gives (dung, guarding, packs), as words.
    #[serde(default)]
    pub work: String,
    pub stages: Stages,
    pub meat: Meat,
    /// Its hide; none for birds and insects.
    #[serde(default)]
    pub hide: Option<Hide>,
    pub pet: Pet,
}

impl LivestockDef {
    /// Whether it makes sense: an error naming what is wrong.
    pub fn check(&self) -> Result<(), String> {
        let key = &self.key;
        let plain = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_';
        if key.is_empty() || !key.chars().all(plain) {
            return Err(format!(
                "{key:?}: a key is lower case letters, digits and _"
            ));
        }
        if self.name.trim().is_empty() || self.latin.trim().is_empty() {
            return Err(format!("{key}: it has no name"));
        }
        if self.kept.is_empty() || self.favourites.is_empty() {
            return Err(format!("{key}: it is kept nowhere or eats nothing"));
        }
        if let Some(wild) = &self.wild
            && (wild.name.trim().is_empty()
                || wild.biomes.is_empty()
                || wild.rarity == Rarity::Never)
        {
            return Err(format!("{key}: its wild form lives nowhere"));
        }
        if self.feral.rarity != Rarity::Never && self.feral.note.trim().is_empty() {
            return Err(format!("{key}: its feral herds aren't described"));
        }
        if self.ruminant && self.class != Class::Mammal {
            return Err(format!("{key}: only a mammal chews the cud"));
        }
        if self.hide.is_some() && self.class != Class::Mammal {
            return Err(format!("{key}: only a mammal has a hide"));
        }
        let error = |what: &str, error: String| format!("{key}: {what}: {error}");
        for produce in &self.produce {
            produce.check().map_err(|e| error("produce", e))?;
        }
        self.stages.check().map_err(|e| error("stages", e))?;
        self.meat.check().map_err(|e| error("meat", e))?;
        if let Some(hide) = &self.hide {
            hide.check().map_err(|e| error("hide", e))?;
        }
        Ok(())
    }
}
