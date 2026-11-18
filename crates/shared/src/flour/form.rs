//! Flour as items: `ItemKind::Flour` with a `FlourForm` saying which grain it
//! was milled from and what it weighs. A stack keys on the grain alone.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::vegetable::{VegetableDef, VegetableSet, Vegetables};

/// Which grain's flour a sack holds, and what it weighs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FlourForm {
    /// The grain's key (`VegetableDef::key`).
    pub grain: String,
    /// What one sack weighs, in grams: a sheaf's weight times the yield.
    pub grams: u32,
}

impl FlourForm {
    /// The flour one sheaf of `grain` mills into; `None` for anything but grain.
    pub fn of(grain: &VegetableDef) -> Option<Self> {
        if grain.set != VegetableSet::Grain {
            return None;
        }
        let grams = (grain.grams * u32::from(grain.flour_yield) + 50) / 100;
        Some(Self {
            grain: grain.key.clone(),
            grams: grams.max(1),
        })
    }

    /// What it is called: "Sack of Rye Flour", or "Sack of Flour" for a grain
    /// the table lacks.
    pub fn name(&self) -> String {
        match Vegetables::built_in().get(&self.grain) {
            Some(def) => format!("Sack of {} Flour", def.name),
            None => "Sack of Flour".to_owned(),
        }
    }

    /// The cells one takes up: a small sack.
    pub fn shape(&self) -> Shape {
        Shape::rectangle(1, 1)
    }
}

/// `count` sacks of `grain`'s flour, as one new item with id `id`: never
/// magical, blessed or cursed, and without quality. `None` for anything but
/// grain.
pub fn new_item(grain: &VegetableDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Option<Item> {
    let mut item = Item::new(id, ItemKind::Flour);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Flour(FlourForm::of(grain)?));
    Some(item)
}

impl Item {
    /// Which flour it is, if it is flour.
    pub fn flour(&self) -> Option<&FlourForm> {
        match &self.form {
            Some(ItemForm::Flour(flour)) => Some(flour),
            _ => None,
        }
    }
}
