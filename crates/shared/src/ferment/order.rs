//! Ferment's order (owner: a cask, the fruit or grain, water or juice, and
//! the yeasts, several from a stack for a large cask) and what it starts: the
//! cask sealed with the drink it will be, fermenting for `ferment_secs`.

use serde::{Deserialize, Serialize};

use crate::cask::{CaskForm, CaskTime};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::fermentation::{Drink, Mash, best_tolerance, drink_quality, ferment, ferment_secs};
use crate::fruit::Fruits;
use crate::id::Id;
use crate::item::{Item, Quality};
use crate::potion::{Liquid, Water};
use crate::vegetable::Vegetables;

/// The Fermenting skill's path: Ferment comes from it.
pub const FERMENTING: &str = "crafting/fermenting";

/// Fermenting practice for starting a cask.
pub const FERMENT_PRACTICE: u32 = 8;

/// How long setting a cask going takes, in seconds.
const FERMENT_SECONDS: u64 = 5;

/// The wind-up in action ticks.
pub fn ferment_ticks() -> u64 {
    FERMENT_SECONDS * ACTION_TICKS_PER_SECOND as u64
}

/// What the client asks Ferment to do.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FermentOrder {
    /// The cask, carried or lying in reach: holding juice, cordial or water.
    pub cask: Id,
    /// The fruit or grain stack (carried) to mash into a cask of water: all of
    /// it goes in. None for juice or cordial.
    pub add: Option<Id>,
    /// The yeast stack (carried).
    pub yeast: Id,
    /// How many of its usable yeasts to pitch, 1 or more.
    pub yeasts: u32,
}

/// Why a cask can't be set fermenting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FermentRefused {
    NotACask,
    /// It is fermenting already.
    Fermenting,
    /// It holds nothing that ferments: juice, cordial, or water to mash into.
    Nothing,
    /// Water needs fruit or grain mashed in.
    NeedsAdding,
    /// Juice and cordial take nothing more.
    TakesNothing,
    /// That fruit or grain doesn't ferment (or isn't fruit or grain).
    WontFerment,
    /// No usable yeast: each yeast is used once between feeds.
    NoYeast,
    /// It would be weaker than 1%.
    TooWeak,
}

impl FermentRefused {
    pub fn words(self) -> &'static str {
        match self {
            Self::NotACask => "Only a cask can be fermented in.",
            Self::Fermenting => "It is fermenting already.",
            Self::Nothing => "It holds nothing that ferments: juice, cordial or water.",
            Self::NeedsAdding => "Water needs fruit or grain mashed into it.",
            Self::TakesNothing => "Juice and cordial ferment as they are.",
            Self::WontFerment => "That won't ferment.",
            Self::NoYeast => "It needs yeast not used since it was fed.",
            Self::TooWeak => "There is too little sugar in it to ferment.",
        }
    }
}

/// What Ferment would start.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FermentPlan {
    /// The drink it will be, at the quality it ferments to.
    pub drink: Drink,
    /// How much of it: the mash, no more than the cask holds.
    pub ml: u32,
    /// How long it ferments, in real seconds.
    pub secs: u64,
}

impl FermentPlan {
    /// `cask` once started: holding the drink, sealed, fermenting. One lying
    /// on the ground keeps its stamp, so its clock goes on by world time.
    pub fn started(&self, cask: &CaskForm) -> CaskForm {
        let liquid = Some(Liquid::Drink(Box::new(self.drink.clone())));
        CaskForm {
            opened: false,
            time: Box::new(CaskTime {
                fermenting: self.secs.max(1),
                made: Some(self.drink.quality),
                lying_since: cask.time.lying_since,
                ..CaskTime::default()
            }),
            ..cask.clone().holding(liquid, self.ml)
        }
    }
}

/// What fermenting `cask` (one item) with `add` (a fruit or grain stack, all
/// of it, for water) and these yeasts would make.
pub fn plan(
    cask: &Item,
    add: Option<&Item>,
    yeasts: &[Quality],
) -> Result<FermentPlan, FermentRefused> {
    let form = cask.cask().ok_or(FermentRefused::NotACask)?;
    if form.time.is_fermenting() {
        return Err(FermentRefused::Fermenting);
    }
    let mash = mash(form, add)?;
    if yeasts.is_empty() {
        return Err(FermentRefused::NoYeast);
    }
    let made = ferment(&mash, best_tolerance(yeasts)).ok_or(FermentRefused::TooWeak)?;
    let ml = mash.ml.min(form.most_ml());
    let quality = drink_quality(yeasts, mash.source, form.wood);
    Ok(FermentPlan {
        secs: ferment_secs(made.kind, form.size, ml, yeasts),
        drink: Drink::new(mash.base, &made, quality),
        ml,
    })
}

/// The mash: the cask's juice or cordial, or its water with `add` in it.
fn mash(form: &CaskForm, add: Option<&Item>) -> Result<Mash, FermentRefused> {
    let fruits = Fruits::built_in();
    let fruit_def = |key: &str| fruits.get(key).ok_or(FermentRefused::WontFerment);
    let mash = match (&form.liquid, add) {
        (Some(Liquid::Juice(_) | Liquid::Cordial(_)), Some(_)) => {
            return Err(FermentRefused::TakesNothing);
        }
        (Some(Liquid::Juice(fruit)), None) => Mash::juice(fruit_def(fruit)?, form.ml),
        (Some(Liquid::Cordial(fruit)), None) => Mash::cordial(fruit_def(fruit)?, form.ml),
        (Some(Liquid::Water(water)), _) if *water == Water::Salt => {
            return Err(FermentRefused::Nothing);
        }
        (Some(Liquid::Water(_)), None) => return Err(FermentRefused::NeedsAdding),
        (Some(Liquid::Water(_)), Some(add)) => mashed_in(add, form.ml)?,
        _ => return Err(FermentRefused::Nothing),
    };
    mash.ok_or(FermentRefused::WontFerment)
}

/// All of `add`, a fruit or grain stack, mashed into `water_ml` of water.
fn mashed_in(add: &Item, water_ml: u32) -> Result<Option<Mash>, FermentRefused> {
    let count = add.count();
    if let Some(fruit) = add.top().fruit() {
        let def = Fruits::built_in().get(&fruit.fruit);
        let grams = fruit.grams.saturating_mul(count);
        return Ok(def.and_then(|def| Mash::fruit(def, grams, water_ml)));
    }
    if let Some(vegetable) = add.top().vegetable().filter(|form| !form.serving) {
        let def = Vegetables::built_in().get(&vegetable.vegetable);
        return Ok(def.and_then(|def| Mash::grain(def, count, water_ml)));
    }
    Err(FermentRefused::WontFerment)
}
