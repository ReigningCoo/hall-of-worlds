//! Fungi as items: `ItemKind::Fungus` with a `FungusForm` saying which one,
//! whether it is a serving cut from a piece, how it looks and what it weighs.
//! Counted, without quality: a stack keys on its fungus and serving alone,
//! whatever each one's look.

use serde::{Deserialize, Serialize};

use super::{Fungi, FungusDef, Unit};
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::rng::Seed;

/// Which fungus an item is, and what one of it is like.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FungusForm {
    /// The fungus's key (`FungusDef::key`).
    pub fungus: String,
    /// One serving cut from a piece (`cut`).
    #[serde(default)]
    pub serving: bool,
    /// A seed for how it looks. A stack shows its bottom one's; stacks don't
    /// compare it.
    pub look: u64,
    /// What one weighs, in grams.
    pub grams: u32,
}

impl FungusForm {
    /// One item of `def`, its look from `seed`.
    pub fn new(def: &FungusDef, seed: Seed) -> Self {
        Self {
            fungus: def.key.clone(),
            serving: false,
            look: seed.derive("fungus look").value(),
            grams: def.grams,
        }
    }

    /// One serving of `def`, cut from a piece.
    pub fn serving(def: &FungusDef, seed: Seed) -> Self {
        let servings = u32::from(def.servings.max(1));
        Self {
            serving: true,
            grams: def.grams.div_ceil(servings),
            ..Self::new(def, seed)
        }
    }

    /// Whether it is cut into servings when eaten: a piece of more than one.
    pub fn is_whole(&self, def: &FungusDef) -> bool {
        def.unit == Unit::Piece && def.servings > 1 && !self.serving
    }

    /// What it is called: its fungus's name, a slice of it for a serving, or
    /// "Fungus" for one the table lacks.
    pub fn name(&self) -> String {
        let Some(def) = Fungi::built_in().get(&self.fungus) else {
            return "Fungus".to_owned();
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

/// `count` of a fungus, as one new item with id `id`: never magical, blessed
/// or cursed, and without quality.
pub fn new_item(def: &FungusDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Fungus);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Fungus(FungusForm::new(def, id.seed())));
    item
}

impl Item {
    /// Which fungus it is, if it is one.
    pub fn fungus(&self) -> Option<&FungusForm> {
        match &self.form {
            Some(ItemForm::Fungus(fungus)) => Some(fungus),
            _ => None,
        }
    }
}
