//! Organs, fats and other parts butchering gives, each naming the animals it
//! comes from (`Animals`) and any condition (`Only`).

use serde::{Deserialize, Serialize};

use super::{Class, LivestockDef};
use crate::herb;

/// Animals a part comes from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Animals {
    /// Every mammal and bird (never an insect).
    All,
    Mammals,
    Birds,
    /// Those that chew the cud (`LivestockDef::ruminant`).
    Ruminants,
    /// One animal, by key.
    Animal(String),
}

impl Animals {
    pub fn includes(&self, def: &LivestockDef) -> bool {
        match self {
            Self::All => def.class != Class::Insect,
            Self::Mammals => def.class == Class::Mammal,
            Self::Birds => def.class == Class::Bird,
            Self::Ruminants => def.ruminant,
            Self::Animal(key) => *key == def.key,
        }
    }
}

/// Only some of those animals give it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Only {
    /// A young animal (calf, lamb, kid).
    Young,
    /// An old one.
    Old,
    /// An uncastrated male.
    IntactMale,
    /// One fattened for it (foie gras).
    Fattened,
    /// While its antlers grow.
    GrowingAntlers,
}

/// Whether any of `from` and none of `except` is this animal.
fn comes_from(from: &[Animals], except: &[String], def: &LivestockDef) -> bool {
    from.iter().any(|from| from.includes(def)) && !except.contains(&def.key)
}

/// An organ, with what it is good for and how likely butchering gives it
/// (`organ_chance`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organ {
    pub key: String,
    pub name: String,
    pub from: Vec<Animals>,
    #[serde(default)]
    pub except: Vec<String>,
    #[serde(default)]
    pub only: Option<Only>,
    /// As food, in words.
    #[serde(default)]
    pub food: String,
    /// What it would do in a potion, proposed (decision 10: Alchemy's use of
    /// organs is still to be decided).
    #[serde(default)]
    pub effects: Vec<herb::Effect>,
    /// Other uses, in words.
    #[serde(default)]
    pub other: String,
    /// The base percent butchering gives it.
    pub base: u8,
    /// A rare stone, staying rare (`organ_chance`).
    #[serde(default)]
    pub stone: bool,
    /// Taken only into an empty bottle (blood, decision 12).
    #[serde(default)]
    pub bottled: bool,
    /// What quests might want it for.
    #[serde(default)]
    pub quest: String,
}

impl Organ {
    pub fn comes_from(&self, def: &LivestockDef) -> bool {
        comes_from(&self.from, &self.except, def)
    }
}

/// A fat. Every animal fat can stand in for an oil as an ointment base
/// (decision 11).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fat {
    pub key: String,
    pub name: String,
    pub from: Vec<Animals>,
    /// The fat it is rendered from (tallow from suet), if not taken as it is.
    #[serde(default)]
    pub rendered_from: Option<String>,
    #[serde(default)]
    pub note: String,
}

impl Fat {
    pub fn comes_from(&self, def: &LivestockDef) -> bool {
        comes_from(&self.from, &[], def)
    }
}

/// Bones, horn, feathers and the rest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Part {
    pub key: String,
    pub name: String,
    pub from: Vec<Animals>,
    #[serde(default)]
    pub only: Option<Only>,
    /// What it is used for, in words.
    pub uses: String,
}

impl Part {
    pub fn comes_from(&self, def: &LivestockDef) -> bool {
        comes_from(&self.from, &[], def)
    }
}
