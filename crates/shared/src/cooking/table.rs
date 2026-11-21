//! `Ingredients`: every raw food's culinary data, found by source and key or
//! by an item's form. Built in only, from `ingredients.ron`, so both sides have
//! the same table without it being sent.

use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::{IngredientDef, Source};
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::potion::Liquid;

/// The ingredients built into the game, as written.
const BUILT_IN: &str = include_str!("ingredients.ron");

/// The kitchen staples' keys (`Source::Pantry`): Salt (`ItemKind::Salt`) and
/// soy sauce (`Liquid::SoySauce`, pressed from moromi).
pub const PANTRY: [&str; 2] = ["salt", "soy_sauce"];

static BUILT_IN_TABLE: LazyLock<Ingredients> = LazyLock::new(|| {
    let defs = Ingredients::read(BUILT_IN)
        .unwrap_or_else(|error| panic!("the built-in ingredients: {error}"));
    Ingredients::new(defs)
});

/// Every ingredient, in the table's order.
#[derive(Clone, Debug, Default)]
pub struct Ingredients {
    defs: Vec<IngredientDef>,
    by_key: BTreeMap<(Source, String), usize>,
}

impl Ingredients {
    /// The ingredients built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_TABLE
    }

    /// The ingredients written in a RON file, each checked and written once.
    pub fn read(text: &str) -> Result<Vec<IngredientDef>, String> {
        let defs: Vec<IngredientDef> = ron::from_str(text).map_err(|error| error.to_string())?;
        for (at, def) in defs.iter().enumerate() {
            def.check()?;
            let twice = defs[..at]
                .iter()
                .any(|other| other.source == def.source && other.key == def.key);
            if twice {
                return Err(format!("{:?} {}: written twice", def.source, def.key));
            }
        }
        Ok(defs)
    }

    /// A table of these; a later one with a source and key already in it is
    /// left out.
    pub fn new(defs: Vec<IngredientDef>) -> Self {
        let mut table = Self::default();
        for def in defs {
            let key = (def.source, def.key.clone());
            if table.by_key.contains_key(&key) {
                continue;
            }
            table.by_key.insert(key, table.defs.len());
            table.defs.push(def);
        }
        table
    }

    /// The ingredient from this source with this key.
    pub fn get(&self, source: Source, key: &str) -> Option<&IngredientDef> {
        self.by_key
            .get(&(source, key.to_owned()))
            .map(|&at| &self.defs[at])
    }

    /// What an item with this form is as an ingredient: herbs, fruit,
    /// vegetables and fungi by their key, flour as its grain, eggs by their
    /// animal, and a bottle, vessel or cask by the milk or honey in it. `None`
    /// for anything else (and for a server's own herbs, which have no row).
    pub fn of_form(&self, form: &ItemForm) -> Option<&IngredientDef> {
        match form {
            ItemForm::Herb(herb) => self.get(Source::Herb, &herb.herb),
            ItemForm::Fruit(fruit) => self.get(Source::Fruit, &fruit.fruit),
            ItemForm::Vegetable(vegetable) => self.get(Source::Vegetable, &vegetable.vegetable),
            ItemForm::Fungus(fungus) => self.get(Source::Fungus, &fungus.fungus),
            ItemForm::Flour(flour) => self.get(Source::Vegetable, &flour.grain),
            ItemForm::Egg(egg) => self.get(Source::Egg, &egg.animal),
            ItemForm::Bottle(bottle) => self.of_liquid(bottle.liquid.as_ref()?),
            ItemForm::Vessel(vessel) => self.of_liquid(vessel.liquid.as_ref()?),
            ItemForm::Cask(cask) => self.of_liquid(cask.liquid.as_ref()?),
            _ => None,
        }
    }

    /// What a liquid is as an ingredient: milk and honey by their animal.
    /// `None` for any other so far.
    pub fn of_liquid(&self, liquid: &Liquid) -> Option<&IngredientDef> {
        match liquid {
            Liquid::Milk(animal) => self.get(Source::Milk, animal),
            Liquid::Honey(animal) => self.get(Source::Honey, animal),
            Liquid::SoySauce => self.get(Source::Pantry, "soy_sauce"),
            _ => None,
        }
    }

    /// What a carried item is as an ingredient: Salt by its kind, else by
    /// its form (`of_form`), never a container of a liquid (taken by the
    /// millilitre instead).
    pub fn of_item(&self, item: &Item) -> Option<&IngredientDef> {
        if item.kind == ItemKind::Salt {
            return self.get(Source::Pantry, "salt");
        }
        let form = item.form.as_ref()?;
        let container = matches!(
            form,
            ItemForm::Bottle(_) | ItemForm::Vessel(_) | ItemForm::Cask(_)
        );
        if container { None } else { self.of_form(form) }
    }

    /// Every ingredient from this source.
    pub fn from_source(&self, source: Source) -> impl Iterator<Item = &IngredientDef> {
        self.defs.iter().filter(move |def| def.source == source)
    }

    /// Every ingredient, in the table's order.
    pub fn iter(&self) -> impl Iterator<Item = &IngredientDef> {
        self.defs.iter()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
