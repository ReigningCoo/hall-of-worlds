//! Kindling as items: `ItemKind::Kindling`, 1 kg pieces split from a log,
//! keeping its wood and stacking by it (owner).

use serde::{Deserialize, Serialize};

use super::LogForm;
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::material::Material;

/// What one piece weighs (owner).
pub const KINDLING_GRAMS: u32 = 1000;

/// Which wood a stack of kindling is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KindlingForm {
    pub wood: Material,
}

impl KindlingForm {
    /// What it is called: "Oak Kindling".
    pub fn name(&self) -> String {
        format!("{} Kindling", self.wood.name())
    }

    /// The cells one piece takes up: 2 × 1.
    pub fn shape(&self) -> Shape {
        Shape::rectangle(2, 1)
    }
}

/// How many pieces a log splits into: one for each whole kilogram (at
/// least one).
pub fn kindling_from(log: &LogForm) -> u32 {
    (log.grams() / KINDLING_GRAMS).max(1)
}

/// `count` pieces of `wood`'s kindling, as one new item with id `id`.
pub fn new_kindling(wood: Material, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Kindling);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Kindling(KindlingForm { wood }));
    item
}

impl Item {
    /// Which kindling it is, if it is some.
    pub fn kindling(&self) -> Option<&KindlingForm> {
        match &self.form {
            Some(ItemForm::Kindling(kindling)) => Some(kindling),
            _ => None,
        }
    }
}
