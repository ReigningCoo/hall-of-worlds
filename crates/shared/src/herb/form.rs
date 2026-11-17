//! Herbs as items: `ItemKind::Herb` with a `HerbForm` saying which herb, how
//! one looks and what it weighs. Counted: a stack is one item with a count, and
//! keys on its herb and quality alone, whatever each one's look.

use serde::{Deserialize, Serialize};

use super::HerbDef;
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt};
use crate::item_form::ItemForm;

/// Which herb an item is, and what one of it is like.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HerbForm {
    /// The herb's key (`HerbDef::key`).
    pub herb: String,
    /// A seed for how it looks, a little different from others of its herb. A
    /// stack shows its bottom one's; stacks don't compare it.
    pub look: u64,
    /// What one weighs, in grams, from its herb when it was picked.
    pub grams: u32,
}

/// `count` of a herb, as one new item with id `id`: the server's to make. Its
/// quality is the plant's rarity (`Abundance::quality`). Herbs are never
/// magical, blessed or cursed.
pub fn new_item(def: &HerbDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Herb);
    item.quality = Some(def.abundance.quality());
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Herb(HerbForm {
        herb: def.key.clone(),
        look: id.seed().derive("herb look").value(),
        grams: def.grams,
    }));
    item
}

impl Item {
    /// Which herb it is, if it is one.
    pub fn herb(&self) -> Option<&HerbForm> {
        match &self.form {
            Some(ItemForm::Herb(herb)) => Some(herb),
            _ => None,
        }
    }
}
