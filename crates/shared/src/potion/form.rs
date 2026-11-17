//! `BottleForm`: what one bottle is, empty or holding a liquid, and how it looks.

use serde::{Deserialize, Serialize};

use super::{Contents, Liquid};
use crate::item::Item;
use crate::item_form::ItemForm;

/// What an empty bottle weighs, in grams: small, thick glass and a stopper.
pub const EMPTY_GRAMS: u32 = 120;
/// What a full one weighs, in grams, whatever it holds: about 100 ml more.
pub const POTION_GRAMS: u32 = 220;

/// A bottle (`ItemKind::Bottle`): what it holds and a seed for how it looks.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct BottleForm {
    /// What is in it, none when empty; the liquid's look comes from this alone.
    pub liquid: Option<Liquid>,
    /// A seed the client picks the bottle's shape, glass and stopper from. A
    /// stack shows its bottom one's; stacks don't compare it.
    pub look: u64,
}

impl BottleForm {
    /// One holding this, or empty; a potion with no doses is an empty bottle.
    pub fn new(liquid: Option<Liquid>, look: u64) -> Self {
        let liquid = liquid.filter(|liquid| liquid.contents().is_none_or(|it| !it.is_empty()));
        Self { liquid, look }
    }

    /// Whether nothing is in it.
    pub fn is_empty(&self) -> bool {
        self.liquid.is_none()
    }

    /// A potion's contents: empty for anything else.
    pub fn contents(&self) -> &Contents {
        const EMPTY: &Contents = &Contents::EMPTY;
        self.liquid
            .as_ref()
            .and_then(Liquid::contents)
            .unwrap_or(EMPTY)
    }

    /// Whether it holds a potion.
    pub fn is_potion(&self) -> bool {
        matches!(self.liquid, Some(Liquid::Potion(_)))
    }

    /// "Empty Bottle", "Potion", or "Bottle of" its liquid ("Bottle of Moon Water").
    pub fn name(&self) -> String {
        match &self.liquid {
            None => "Empty Bottle".to_owned(),
            Some(Liquid::Potion(_)) => "Potion".to_owned(),
            Some(liquid) => format!("Bottle of {}", liquid.name()),
        }
    }

    /// What one weighs.
    pub fn grams(&self) -> u32 {
        match self.is_empty() {
            true => EMPTY_GRAMS,
            false => POTION_GRAMS,
        }
    }
}

impl<'de> Deserialize<'de> for BottleForm {
    /// Messages (postcard) carry exactly what is written; saves (RON) may be
    /// from before liquids (2026-11-17), with only `contents`: a potion, or an
    /// empty bottle when it had none.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if !deserializer.is_human_readable() {
            let wire = Wire::deserialize(deserializer)?;
            return Ok(Self::new(wire.liquid, wire.look));
        }
        let saved = Saved::deserialize(deserializer)?;
        let liquid = saved.liquid.or(Some(Liquid::Potion(saved.contents)));
        Ok(Self::new(liquid, saved.look))
    }
}

/// A bottle as messages carry it.
#[derive(Deserialize)]
#[serde(rename = "BottleForm")]
struct Wire {
    liquid: Option<Liquid>,
    look: u64,
}

/// A bottle as saved, now or before liquids.
#[derive(Deserialize)]
#[serde(rename = "BottleForm")]
struct Saved {
    #[serde(default)]
    liquid: Option<Liquid>,
    #[serde(default)]
    contents: Contents,
    look: u64,
}

impl Item {
    /// Its bottle, if it is one.
    pub fn bottle(&self) -> Option<&BottleForm> {
        match &self.form {
            Some(ItemForm::Bottle(bottle)) => Some(bottle),
            _ => None,
        }
    }
}
