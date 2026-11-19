//! Paste as an item (Alchemy plan PR 3): `ItemKind::Paste`, herbs ground dry,
//! counted and stacking by contents and quality like an ointment, with no way
//! to apply it: a process's stored source. Also what Alchemy gives for a
//! Mortar last (`new_mixture`) and what a carried mixture is (`Item::mixture`).

use super::Form;
use crate::brewing::{OintmentForm, new_ointment};
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::Contents;

/// What one paste weighs, in grams: a little less than an ointment's pot, with
/// no oil in it (first guess).
pub const PASTE_GRAMS: u32 = 40;

/// `count` of a Paste or an Ointment holding `contents` at `quality` (as
/// Alchemy settled it, luck and all). A Powder can't be made yet (PR 6), and a
/// Tincture is a liquid (a potion when last): an Ointment stands in, so
/// nothing is lost.
pub fn new_mixture(form: Form, contents: Contents, quality: Quality, count: u32, id: Id) -> Item {
    let mut item = match form {
        Form::Paste => {
            let mut item = Item::new(id, ItemKind::Paste);
            item.form = Some(ItemForm::Paste(OintmentForm { contents }));
            item.count = count.max(1);
            item
        }
        Form::Ointment | Form::Powder | Form::Tincture => new_ointment(contents, count, id),
    };
    item.quality = Some(quality);
    item
}

impl Item {
    /// Its paste, if it is one.
    pub fn paste(&self) -> Option<&OintmentForm> {
        match &self.form {
            Some(ItemForm::Paste(paste)) => Some(paste),
            _ => None,
        }
    }

    /// What mixture it is and its doses, if it is a Paste or an Ointment: what
    /// a process can start from.
    pub fn mixture(&self) -> Option<(Form, &Contents)> {
        match &self.form {
            Some(ItemForm::Paste(paste)) => Some((Form::Paste, &paste.contents)),
            Some(ItemForm::Ointment(ointment)) => Some((Form::Ointment, &ointment.contents)),
            _ => None,
        }
    }
}
