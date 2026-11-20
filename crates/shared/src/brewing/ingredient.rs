//! What Alchemy's Mortar grinds: herbs, and the fungi used like herbs (fungi
//! plan decision 7), each with four effects. `Ingredient` says which table a
//! key is from, so a herb's and a fungus's keys never clash in what is known,
//! what a run teaches or the same kind twice.

use serde::{Deserialize, Serialize};

use crate::fungus::FungusDef;
use crate::herb::{Effect, HerbDef};
use crate::item::Quality;

/// Which table a Mortar's ingredient is from.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Ingredient {
    #[default]
    Herb,
    Fungus,
}

/// One kind of herb (or fungus used like one) in a mix, at the quality of the
/// ones used.
#[derive(Clone, Copy, Debug)]
pub struct GroundHerb<'a> {
    pub ingredient: Ingredient,
    /// Its key in its table.
    pub key: &'a str,
    /// Its four effects.
    pub effects: &'a [Effect],
    pub quality: Quality,
}

impl<'a> GroundHerb<'a> {
    /// A herb at `quality`.
    pub fn herb(def: &'a HerbDef, quality: Quality) -> Self {
        Self {
            ingredient: Ingredient::Herb,
            key: &def.key,
            effects: &def.effects,
            quality,
        }
    }

    /// A fungus used like a herb. Fungi have no quality, so they count as
    /// Common.
    pub fn fungus(def: &'a FungusDef) -> Self {
        Self {
            ingredient: Ingredient::Fungus,
            key: &def.key,
            effects: &def.herb,
            quality: Quality::Common,
        }
    }

    /// Whether it is the same kind as `other`: the same key in the same table.
    pub fn same_kind(&self, other: &GroundHerb) -> bool {
        self.ingredient == other.ingredient && self.key == other.key
    }
}
