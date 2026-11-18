//! `Held`: what a container of a liquid holds and takes, whatever it is, and
//! the same container holding something else.

use crate::item::Item;
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid};

/// What a bottle holds, in millilitres: a bottle is full or empty, never in
/// between (what a full one weighs more, `potion::POTION_GRAMS`).
pub const BOTTLE_ML: u32 = 100;

/// What is in a container of a liquid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Held {
    /// What is in it, none when empty.
    pub liquid: Option<Liquid>,
    /// How many millilitres.
    pub ml: u32,
    /// How many more it takes.
    pub room: u32,
}

impl Held {
    /// What `item` holds, if it is a cask, a vessel or a bottle.
    pub fn of(item: &Item) -> Option<Self> {
        match &item.form {
            Some(ItemForm::Cask(cask)) => Some(Self {
                liquid: cask.liquid.clone(),
                ml: cask.ml,
                room: cask.room(),
            }),
            Some(ItemForm::Vessel(vessel)) => Some(Self {
                liquid: vessel.liquid.clone(),
                ml: vessel.ml,
                room: vessel.room(),
            }),
            Some(ItemForm::Bottle(bottle)) => {
                let ml = if bottle.is_empty() { 0 } else { BOTTLE_ML };
                Some(Self {
                    liquid: bottle.liquid.clone(),
                    ml,
                    room: BOTTLE_ML - ml,
                })
            }
            _ => None,
        }
    }
}

/// `item` holding `ml` of `liquid` instead (nothing at 0), sealed if `sealed`
/// and it can be; a cask is opened whatever is poured.
pub(super) fn holding(item: &Item, liquid: &Liquid, ml: u32, sealed: bool) -> Item {
    let liquid = (ml > 0).then(|| liquid.clone());
    let form = match &item.form {
        Some(ItemForm::Cask(cask)) => ItemForm::Cask(cask.clone().holding(liquid, ml).opened()),
        Some(ItemForm::Vessel(vessel)) => {
            let vessel = vessel.clone().holding(liquid, ml);
            ItemForm::Vessel(if sealed { vessel.sealed() } else { vessel })
        }
        Some(ItemForm::Bottle(bottle)) => {
            ItemForm::Bottle(BottleForm::new(liquid, bottle.look).with_seal(sealed))
        }
        _ => return item.clone(),
    };
    Item {
        form: Some(form),
        ..item.clone()
    }
}
