//! Eggs as items: `ItemKind::Egg` with an `EggForm` saying which animal laid
//! it and what it weighs. Counted, a stack keying on the animal; no quality
//! (freshness comes with cooking's clocks).

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::livestock::LivestockDef;

/// Which animal laid an egg, and what one weighs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EggForm {
    /// The animal's key (`LivestockDef::key`).
    pub animal: String,
    /// What one egg weighs, in grams (`Produce::grams`).
    pub grams: u32,
}

impl EggForm {
    /// The egg `animal` lays; `None` for one that lays none.
    pub fn of(animal: &LivestockDef) -> Option<Self> {
        let grams = animal.egg_grams();
        (grams > 0).then(|| Self {
            animal: animal.key.clone(),
            grams,
        })
    }

    /// What it is called: "Hen's Egg", "Goose Egg".
    pub fn name(&self) -> String {
        super::egg_name(&self.animal)
    }

    /// The cells one takes up.
    pub fn shape(&self) -> Shape {
        Shape::rectangle(1, 1)
    }
}

/// `count` of `animal`'s eggs, as one new item with id `id`: never magical,
/// blessed or cursed, and without quality. `None` for an animal laying none.
pub fn new_egg(animal: &LivestockDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Option<Item> {
    let mut item = Item::new(id, ItemKind::Egg);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Egg(EggForm::of(animal)?));
    Some(item)
}

impl Item {
    /// Which egg it is, if it is an egg.
    pub fn egg(&self) -> Option<&EggForm> {
        match &self.form {
            Some(ItemForm::Egg(egg)) => Some(egg),
            _ => None,
        }
    }
}
