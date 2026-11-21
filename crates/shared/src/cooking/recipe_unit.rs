//! What a recipe counts its foods in: one item of a food (`unit_grams`), or
//! millilitres of a liquid one (`is_liquid_food`); and the usual food a
//! recipe means by "any egg" or "any milk" (`usual_food`).

use super::{Ingredients, Source, Usage, Wanted};
use crate::flour::FlourForm;
use crate::fruit::Fruits;
use crate::fungus::Fungi;
use crate::herb::Herbs;
use crate::item::ItemKind;
use crate::livestock::Livestock;
use crate::vegetable::{VegetableSet, Vegetables};

/// The food of a source a recipe means when it asks for any of it: the
/// common one (a hen's egg, cow's milk, bees' honey, beef).
const USUAL: [(Source, &str); 5] = [
    (Source::Egg, "chicken"),
    (Source::Milk, "cattle"),
    (Source::Honey, "honeybee"),
    (Source::Meat, "cattle"),
    (Source::Fruit, "apple"),
];

/// What a recipe counts as one of a butchered part (an organ, fat, bones),
/// until parts are items.
pub const PART_GRAMS: u32 = 100;

/// Whether a food is measured by the millilitre (milk, honey, soy sauce),
/// not counted by the item.
pub fn is_liquid_food(source: Source, key: &str) -> bool {
    match source {
        Source::Milk | Source::Honey => true,
        Source::Pantry => key == "soy_sauce",
        _ => false,
    }
}

/// What one item of a food weighs, in grams: a herb, fruit, vegetable or
/// fungus by its table, a grain as a sack of its flour, an egg by its animal,
/// meat as Raw Meat, other parts as `PART_GRAMS`, salt as Salt. `None` for a liquid food
/// or one no table knows.
pub fn unit_grams(source: Source, key: &str) -> Option<u32> {
    if is_liquid_food(source, key) {
        return None;
    }
    let grams = match source {
        Source::Herb => Herbs::built_in_ref().get(key)?.grams,
        Source::Fruit => Fruits::built_in().get(key)?.grams,
        Source::Vegetable => {
            let def = Vegetables::built_in().get(key)?;
            match def.set {
                VegetableSet::Grain => FlourForm::of(def)?.grams,
                _ => def.grams,
            }
        }
        Source::Fungus => Fungi::built_in().get(key)?.grams,
        Source::Egg => Livestock::built_in().get(key)?.egg_grams(),
        Source::Meat => ItemKind::RawMeat.def().weight,
        Source::Part => PART_GRAMS,
        Source::Pantry => ItemKind::Salt.def().weight,
        Source::Milk | Source::Honey => return None,
    };
    (grams > 0).then_some(grams)
}

/// The food a recipe's `wanted` means: itself if it names one, else the
/// usual one of its source, else the first common one in the table.
pub fn usual_food(wanted: &Wanted, ingredients: &Ingredients) -> Option<(Source, String)> {
    match wanted {
        Wanted::One(source, key) => Some((*source, key.clone())),
        Wanted::Any(source) => {
            let usual = (USUAL.iter()).find(|(s, _)| s == source).map(|(_, k)| *k);
            if let Some(key) = usual.filter(|key| ingredients.get(*source, key).is_some()) {
                return Some((*source, key.to_owned()));
            }
            let common =
                |def: &&super::IngredientDef| matches!(def.usage, Usage::Staple | Usage::Common);
            let def = (ingredients.from_source(*source).find(common))
                .or_else(|| ingredients.from_source(*source).next())?;
            Some((def.source, def.key.clone()))
        }
    }
}
