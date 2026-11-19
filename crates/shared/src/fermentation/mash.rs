//! What goes into a cask to ferment (`Mash`): its sugar as kcal and its
//! volume (owner: the kcal of juice when it is juice, else the fruit's or
//! grain's with the water added, so one apple in a butt does nothing).

use crate::fruit::{FruitDef, FruitSet};
use crate::juice::{kcal_per_litre, ml_from};
use crate::vegetable::{VegetableDef, VegetableSet};

use super::Base;

/// The share of a whole fruit's kcal that ferments when it is mashed in
/// water, in thousandths: as much as pressing puts in its juice.
pub const FRUIT_SUGAR: u64 = 600;

/// The share of a grain's kcal that a mash in water turns to sugar, in
/// thousandths (a guess: about 190 g of grain a litre for a 5% ale).
pub const GRAIN_SUGAR: u64 = 500;

/// How the mash was made, which bears on the drink's quality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MashSource {
    Juice,
    Cordial,
    FruitInWater,
    Grain,
}

/// A cask's worth (or any amount) of something to ferment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mash {
    pub base: Base,
    pub source: MashSource,
    /// The sugar in it all, as kcal.
    pub kcal: u64,
    /// Its volume in millilitres.
    pub ml: u32,
}

impl Mash {
    /// `ml` of `def`'s juice; none if it can't be juiced or `ml` is 0.
    pub fn juice(def: &FruitDef, ml: u32) -> Option<Self> {
        let kcal = u64::from(kcal_per_litre(def)) * u64::from(ml) / 1000;
        Self::new(Base::Fruit(def.key.clone()), MashSource::Juice, kcal, ml)
    }

    /// `ml` of `def`'s cordial. How much water went in isn't kept, so it is
    /// taken as half juice, half water (the most water `juice::mix` allows).
    pub fn cordial(def: &FruitDef, ml: u32) -> Option<Self> {
        let kcal = u64::from(kcal_per_litre(def)) * u64::from(ml) / 2000;
        Self::new(Base::Fruit(def.key.clone()), MashSource::Cordial, kcal, ml)
    }

    /// `grams` of whole `def` mashed into `water_ml` of water: `FRUIT_SUGAR`
    /// of its kcal, in the water and the juice it gives. Any fruit (not a nut
    /// or a herb's fruit) safe raw.
    pub fn fruit(def: &FruitDef, grams: u32, water_ml: u32) -> Option<Self> {
        if !def.is_item() || def.set != FruitSet::Fruit || def.harmful_raw || def.grams == 0 {
            return None;
        }
        let kcal = u64::from(def.kcal) * u64::from(grams) / u64::from(def.grams);
        let ml = water_ml.saturating_add(ml_from(def, grams));
        let base = Base::Fruit(def.key.clone());
        Self::new(
            base,
            MashSource::FruitInWater,
            kcal * FRUIT_SUGAR / 1000,
            ml,
        )
    }

    /// `sheaves` of the grain `def` mashed into `water_ml` of water (grains
    /// ferment directly, owner): `GRAIN_SUGAR` of their kcal.
    pub fn grain(def: &VegetableDef, sheaves: u32, water_ml: u32) -> Option<Self> {
        if def.set != VegetableSet::Grain {
            return None;
        }
        let kcal = u64::from(def.kcal) * u64::from(sheaves) * GRAIN_SUGAR / 1000;
        Self::new(
            Base::Grain(def.key.clone()),
            MashSource::Grain,
            kcal,
            water_ml,
        )
    }

    fn new(base: Base, source: MashSource, kcal: u64, ml: u32) -> Option<Self> {
        (ml > 0).then_some(Self {
            base,
            source,
            kcal,
            ml,
        })
    }

    /// Its sugar as kcal a litre.
    pub fn kcal_per_litre(&self) -> u64 {
        self.kcal * 1000 / u64::from(self.ml.max(1))
    }
}
