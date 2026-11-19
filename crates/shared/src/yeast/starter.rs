//! Starting a starter (owner: from flour and fresh water in a bottle, which
//! develops over 24 hours of playtime): a sack of flour let go on a bottle of
//! fresh water turns the bottle into a crock of yeast still developing, Poor
//! as anything wild (Claude's first guess), to be fed up from there.

use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Quality};

use super::form::{YeastForm, new_item};
use super::supplies::fresh;

/// How long a starter takes to be ready, by the yeast's clock: 24 hours.
pub const STARTS_IN: u32 = 24 * 3600;

/// Fermenting practice for starting one (Claude's first guess).
pub const STARTER_PRACTICE: u32 = 4;

/// Why a starter can't be begun.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartRefused {
    NotFlour,
    /// It needs a bottle of fresh water (not salt).
    NotWater,
}

impl StartRefused {
    /// Why, in English, for logs and the client's red line.
    pub fn words(self) -> &'static str {
        match self {
            Self::NotFlour => "a starter is begun with flour",
            Self::NotWater => "a starter needs a bottle of fresh water",
        }
    }
}

/// Whether `flour` (a stack's top) let go on `water` (a stack's top) would
/// begin a starter.
pub fn can_start(flour: &Item, water: &Item) -> Result<(), StartRefused> {
    if flour.kind != ItemKind::Flour {
        return Err(StartRefused::NotFlour);
    }
    match water.bottle() {
        Some(bottle) if fresh(bottle.liquid.as_ref()) => Ok(()),
        _ => Err(StartRefused::NotWater),
    }
}

/// A new starter: a Poor yeast developing for `STARTS_IN`.
pub fn new_starter(id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = new_item(Quality::Poor, id, made_at);
    item.form = Some(crate::item_form::ItemForm::Yeast(YeastForm {
        developing: STARTS_IN,
        ..YeastForm::default()
    }));
    item
}
