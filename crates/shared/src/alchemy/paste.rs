//! Paste as an item (Alchemy plan PR 3): `ItemKind::Paste`, herbs ground dry,
//! counted and stacking by contents and quality like an ointment, with no way
//! to apply it: a process's stored source. Powder (PR 6, `ItemKind::Powder`)
//! the same, from a Crucible. Also what Alchemy gives for a mixture last
//! (`new_mixture`) and what a carried mixture is (`Item::mixture`).

use super::Form;
use crate::brewing::OintmentForm;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::Contents;

/// What one paste weighs, in grams: a little less than an ointment's pot, with
/// no oil in it (first guess).
pub const PASTE_GRAMS: u32 = 40;

/// What one powder weighs, in grams: dry, in a twist of paper (first guess).
pub const POWDER_GRAMS: u32 = 20;

/// `count` of a Paste or an Ointment holding `contents` at `quality` (as
/// Alchemy settled it, luck and all); a Powder likewise. A Tincture is a
/// liquid (a potion when last): an Ointment stands in, so nothing is lost.
pub fn new_mixture(form: Form, contents: Contents, quality: Quality, count: u32, id: Id) -> Item {
    let dry = OintmentForm { contents };
    let (kind, form) = match form {
        Form::Paste => (ItemKind::Paste, ItemForm::Paste(dry)),
        Form::Powder => (ItemKind::Powder, ItemForm::Powder(dry)),
        Form::Ointment | Form::Tincture => (ItemKind::Ointment, ItemForm::Ointment(dry)),
    };
    let mut item = Item::new(id, kind);
    item.form = Some(form);
    item.count = count.max(1);
    item.quality = Some(quality);
    item
}

impl Item {
    /// Its powder, if it is one.
    pub fn powder(&self) -> Option<&OintmentForm> {
        match &self.form {
            Some(ItemForm::Powder(powder)) => Some(powder),
            _ => None,
        }
    }

    /// Its paste, if it is one.
    pub fn paste(&self) -> Option<&OintmentForm> {
        match &self.form {
            Some(ItemForm::Paste(paste)) => Some(paste),
            _ => None,
        }
    }

    /// What mixture it is and its doses, if it is a Paste, an Ointment or a
    /// Powder: what a process can start from.
    pub fn mixture(&self) -> Option<(Form, &Contents)> {
        match &self.form {
            Some(ItemForm::Paste(paste)) => Some((Form::Paste, &paste.contents)),
            Some(ItemForm::Ointment(ointment)) => Some((Form::Ointment, &ointment.contents)),
            Some(ItemForm::Powder(powder)) => Some((Form::Powder, &powder.contents)),
            _ => None,
        }
    }
}
