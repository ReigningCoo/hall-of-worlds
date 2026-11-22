//! Refilling a lantern (owner): a bottle of any oil dropped on it pours in by
//! the millilitre, as much as fits; the bottle comes back empty, and what did
//! not fit is spilt (a bottle is full or empty, never in between: `pour`).

use crate::item::Item;
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid};
use crate::pour::BOTTLE_ML;

/// Why a bottle can't fill a lantern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillRefused {
    /// What it was dropped on is no lantern.
    NotALantern,
    /// What was dropped is no bottle of oil.
    NotOil,
    /// The fount is full already.
    Full,
}

/// A lantern filled from a bottle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filled {
    /// The lantern, with the oil in.
    pub lantern: Item,
    /// The bottle, empty now.
    pub bottle: Item,
    /// Millilitres poured in.
    pub ml: u32,
    /// Millilitres that did not fit and were spilt.
    pub spilt: u32,
}

/// Pours one full bottle of oil (`bottle`, a single one: the server takes it
/// off its stack) into `lantern`, refusing a full one.
pub fn fill(lantern: &Item, bottle: &Item) -> Result<Filled, FillRefused> {
    let fount = lantern.lantern().ok_or(FillRefused::NotALantern)?;
    let Some(ItemForm::Bottle(form)) = &bottle.form else {
        return Err(FillRefused::NotOil);
    };
    if !matches!(form.liquid, Some(Liquid::Oil(_))) {
        return Err(FillRefused::NotOil);
    }
    if fount.room() == 0 {
        return Err(FillRefused::Full);
    }
    let ml = fount.room().min(BOTTLE_ML);
    let mut lantern = lantern.clone();
    if let Some(fount) = lantern.lantern_mut() {
        fount.ml += ml;
    }
    let empty = BottleForm::new(None, form.look);
    let bottle = Item {
        form: Some(ItemForm::Bottle(empty)),
        above: Vec::new(),
        ..bottle.clone()
    };
    Ok(Filled {
        lantern,
        bottle,
        ml,
        spilt: BOTTLE_ML - ml,
    })
}
