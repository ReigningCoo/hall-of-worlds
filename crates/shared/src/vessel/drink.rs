//! Drinking from a vessel (`ClientMessage::Consume`): a serving at a time of
//! water (which does nothing), juice or cordial (Mana: `juice::drink_effect`).
//! Alcohol comes with drinks plan PR 7.

use super::VesselForm;
use crate::item::Item;
use crate::potion::Liquid;

/// How much one drink takes, in millilitres: a cup's worth, or what is left.
pub const SERVING_ML: u32 = 250;

impl VesselForm {
    /// Whether it can be drunk from: it holds a water, a juice or a cordial
    /// (`Liquid::is_drink`; oil can't be drunk).
    pub fn drinkable(&self) -> bool {
        self.liquid.as_ref().is_some_and(Liquid::is_drink)
    }

    /// The same after one drink, and how many millilitres were drunk: none if
    /// it can't be drunk from. Drinking from it opens it.
    pub fn drunk(self) -> (Self, u32) {
        if !self.drinkable() {
            return (self, 0);
        }
        let ml = self.ml.min(SERVING_ML);
        let left = self.ml - ml;
        let liquid = self.liquid.clone();
        (self.holding(liquid, left), ml)
    }
}

impl Item {
    /// Whether this is a vessel that can be drunk from.
    pub fn is_drinkable_vessel(&self) -> bool {
        self.vessel().is_some_and(VesselForm::drinkable)
    }
}
