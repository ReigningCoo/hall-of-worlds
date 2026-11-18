//! Ointment as an item (brewing plan, decision 8; PR 6): `ItemKind::Ointment`,
//! counted, stacking by contents and quality, its quality from its contents
//! (`quality`).

use serde::{Deserialize, Serialize};

use super::quality;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::potion::Contents;

/// What one dose of ointment weighs, in grams: a small pot's worth of salve.
pub const OINTMENT_GRAMS: u32 = 60;

/// An ointment's form (`ItemForm::Ointment`): the doses each one holds.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OintmentForm {
    pub contents: Contents,
}

/// `count` of an ointment holding `contents`, its quality worked out from them.
pub fn new_ointment(contents: Contents, count: u32, id: Id) -> Item {
    let mut item = Item::new(id, ItemKind::Ointment);
    item.quality = Some(quality(&contents));
    item.form = Some(ItemForm::Ointment(OintmentForm { contents }));
    item.count = count.max(1);
    item
}

impl Item {
    /// Its ointment, if it is one.
    pub fn ointment(&self) -> Option<&OintmentForm> {
        match &self.form {
            Some(ItemForm::Ointment(ointment)) => Some(ointment),
            _ => None,
        }
    }
}
