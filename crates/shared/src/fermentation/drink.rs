//! A fermented drink as a liquid (`potion::Liquid::Drink`, drinks plan PR 6):
//! what it was made from, how strong and sweet it is, its quality, and how
//! much vinegar has been added to it; and what it turns into, vinegar
//! (`Liquid::Vinegar` of its base).

use serde::{Deserialize, Serialize};

use crate::item::Quality;

use super::{Base, Fermented, Sweetness};

/// A drink in a cask, vessel or bottle.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Drink {
    pub base: Base,
    /// Tenths of a percent of alcohol.
    pub strength: u32,
    pub sweetness: Sweetness,
    /// Its quality now: as fermented, then aged in a sealed cask.
    pub quality: Quality,
    /// How much of it is vinegar added, in thousandths (`vinegar_share`):
    /// it turns sooner (`best_before_secs`).
    pub vinegar: u32,
}

impl Drink {
    /// What `base` fermented into, at `quality`, with no vinegar.
    pub fn new(base: Base, fermented: &Fermented, quality: Quality) -> Self {
        Self {
            base,
            strength: fermented.strength,
            sweetness: fermented.sweetness,
            quality,
            vinegar: 0,
        }
    }

    /// "Cider", "Plum Wine", "Ale".
    pub fn name(&self) -> String {
        self.base.name()
    }

    /// Its strength in words: "5.8%".
    pub fn strength_words(&self) -> String {
        format!("{}.{}%", self.strength / 10, self.strength % 10)
    }

    /// The same with `vinegar_ml` of vinegar poured into `ml` of it.
    pub fn with_vinegar(self, ml: u32, vinegar_ml: u32) -> Self {
        let had = u64::from(self.vinegar) * u64::from(ml) / 1000;
        let total = ml.saturating_add(vinegar_ml);
        let vinegar = super::vinegar_share((had as u32).saturating_add(vinegar_ml), total);
        Self { vinegar, ..self }
    }
}

impl Base {
    /// What its vinegar is called: "Cider Vinegar", "Wine Vinegar" (any
    /// fruit's), "Malt Vinegar" (any grain's).
    pub fn vinegar_name(&self) -> &'static str {
        match self.kind() {
            super::DrinkKind::Cider => "Cider Vinegar",
            super::DrinkKind::Ale => "Malt Vinegar",
            _ => "Wine Vinegar",
        }
    }
}
