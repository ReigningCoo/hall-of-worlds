//! Fruits as items: `ItemKind::Fruit` with a `FruitForm` saying which fruit,
//! whether it is a serving cut from a whole one, how it looks and what it
//! weighs. Counted: a stack keys on its fruit and serving alone, whatever each
//! one's look.

use serde::{Deserialize, Serialize};

use super::{FruitDef, Fruits, Unit};
use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::rng::Seed;

/// The fruit old saves' apples become, and what shops and mobs give until
/// where fruit grows decides (PR 3 of the fruit plan).
pub const APPLE: &str = "apple";

/// Which fruit an item is, and what one of it is like.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FruitForm {
    /// The fruit's key (`FruitDef::key`).
    pub fruit: String,
    /// One serving cut from a whole fruit (`cut`), not the fruit itself.
    #[serde(default)]
    pub serving: bool,
    /// A seed for how it looks. A stack shows its bottom one's; stacks don't
    /// compare it.
    pub look: u64,
    /// What one weighs, in grams.
    pub grams: u32,
}

impl FruitForm {
    /// One whole item of `def`, its look from `seed`.
    pub fn new(def: &FruitDef, seed: Seed) -> Self {
        Self {
            fruit: def.key.clone(),
            serving: false,
            look: seed.derive("fruit look").value(),
            grams: def.grams,
        }
    }

    /// One serving of `def`, cut from a whole one.
    pub fn serving(def: &FruitDef, seed: Seed) -> Self {
        let servings = u32::from(def.servings.max(1));
        Self {
            serving: true,
            grams: def.grams.div_ceil(servings),
            ..Self::new(def, seed)
        }
    }

    /// Whether it is a whole fruit to be cut into servings when eaten.
    pub fn is_whole(&self, def: &FruitDef) -> bool {
        def.unit == Unit::Whole && !self.serving
    }

    /// What it is called: its fruit's name, a slice of it for a serving, or
    /// "Fruit" for one the table lacks.
    pub fn name(&self) -> String {
        let Some(def) = Fruits::built_in().get(&self.fruit) else {
            return "Fruit".to_owned();
        };
        match self.serving {
            true => format!("{} Slice", def.name),
            false => def.name.clone(),
        }
    }

    /// The cells one takes up: big whole fruits more.
    pub fn shape(&self) -> Shape {
        match self.grams {
            4000.. => Shape::rectangle(2, 2),
            1000.. => Shape::rectangle(2, 1),
            _ => Shape::rectangle(1, 1),
        }
    }
}

/// `count` of a fruit, as one new item with id `id`: never magical, blessed or
/// cursed, and without quality.
pub fn new_item(def: &FruitDef, count: u32, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Fruit);
    item.count = count.max(1);
    item.made_at = made_at;
    item.form = Some(ItemForm::Fruit(FruitForm::new(def, id.seed())));
    item
}

impl Item {
    /// Which fruit it is, if it is one.
    pub fn fruit(&self) -> Option<&FruitForm> {
        match &self.form {
            Some(ItemForm::Fruit(fruit)) => Some(fruit),
            _ => None,
        }
    }
}
