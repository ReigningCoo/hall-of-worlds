//! Vegetables as items: `ItemKind::Vegetable` with a `VegetableForm` saying
//! which one, whether it is a serving cut from a whole one, how it looks and
//! what it weighs. Counted: a stack keys on its vegetable and serving alone,
//! whatever each one's look.

use serde::{Deserialize, Serialize};

use super::{Unit, VegetableDef, Vegetables};
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::rng::Seed;

/// Which vegetable an item is, and what one of it is like.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VegetableForm {
    /// The vegetable's key (`VegetableDef::key`).
    pub vegetable: String,
    /// One serving cut from a whole one or a piece (`cut`).
    #[serde(default)]
    pub serving: bool,
    /// A seed for how it looks. A stack shows its bottom one's; stacks don't
    /// compare it.
    pub look: u64,
    /// What one weighs, in grams.
    pub grams: u32,
}

impl VegetableForm {
    /// One item of `def`, its look from `seed`.
    pub fn new(def: &VegetableDef, seed: Seed) -> Self {
        Self {
            vegetable: def.key.clone(),
            serving: false,
            look: seed.derive("vegetable look").value(),
            grams: def.grams,
        }
    }

    /// One serving of `def`, cut from a whole one.
    pub fn serving(def: &VegetableDef, seed: Seed) -> Self {
        let servings = u32::from(def.servings.max(1));
        Self {
            serving: true,
            grams: def.grams.div_ceil(servings),
            ..Self::new(def, seed)
        }
    }

    /// Whether it is cut into servings when eaten: a whole one or a piece of
    /// more than one serving.
    pub fn is_whole(&self, def: &VegetableDef) -> bool {
        matches!(def.unit, Unit::Whole | Unit::Piece) && def.servings > 1 && !self.serving
    }

    /// What it is called: its vegetable's name, a slice of it for a serving,
    /// or "Vegetable" for one the table lacks.
    pub fn name(&self) -> String {
        let Some(def) = Vegetables::built_in().get(&self.vegetable) else {
            return "Vegetable".to_owned();
        };
        match self.serving {
            true => format!("{} Slice", def.name),
            false => def.name.clone(),
        }
    }

    /// The cells one takes up: big ones more, as fruit.
    pub fn shape(&self) -> Shape {
        match self.grams {
            4000.. => Shape::rectangle(2, 2),
            1000.. => Shape::rectangle(2, 1),
            _ => Shape::rectangle(1, 1),
        }
    }
}

/// `count` of a vegetable, as one new item with id `id`: never magical,
/// blessed or cursed, and without quality.
pub fn new_item(def: &VegetableDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Vegetable);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Vegetable(VegetableForm::new(def, id.seed())));
    item
}

impl Item {
    /// Which vegetable it is, if it is one.
    pub fn vegetable(&self) -> Option<&VegetableForm> {
        match &self.form {
            Some(ItemForm::Vegetable(vegetable)) => Some(vegetable),
            _ => None,
        }
    }
}
