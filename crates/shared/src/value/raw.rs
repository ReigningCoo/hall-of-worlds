//! Raw goods: what was gathered, picked or split rather than made, worth its
//! weight at a price by how common it is, with no labour. First guesses,
//! anchored on the apple: an Uncommon 200 g fruit is ½ copper before quality,
//! so 1 copper at Common.

use super::per_kg;
use crate::fruit::{self, Fruits};
use crate::fungus::Fungi;
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::vegetable::{self, Vegetables};

/// A kilogram of herbs: rarer herbs are better quality, which raises them.
const HERB_PER_KG: u64 = 30_000;
const EGG_PER_KG: u64 = 4_000;
const FLOUR_PER_KG: u64 = 2_000;
const ASH_PER_KG: u64 = 200;
/// What anything gathered is if its table doesn't know it.
const UNKNOWN_PER_KG: u64 = 1_500;

/// What one raw item is worth in fine units before quality, or `None` if it
/// is made, not raw.
pub(super) fn of(item: &Item) -> Option<u64> {
    let (grams, per_kg) = match (item.kind, item.form.as_ref()) {
        (ItemKind::Herb, Some(ItemForm::Herb(form))) => (form.grams, HERB_PER_KG),
        (ItemKind::Herb, _) => (10, HERB_PER_KG),
        (ItemKind::Fruit, Some(ItemForm::Fruit(form))) => {
            let def = Fruits::built_in().get(&form.fruit);
            (
                form.grams,
                def.map_or(UNKNOWN_PER_KG, |def| fruit_per_kg(def.abundance)),
            )
        }
        (ItemKind::Vegetable, Some(ItemForm::Vegetable(form))) => {
            let def = Vegetables::built_in().get(&form.vegetable);
            (
                form.grams,
                def.map_or(UNKNOWN_PER_KG, |def| vegetable_per_kg(def.abundance)),
            )
        }
        (ItemKind::Fungus, Some(ItemForm::Fungus(form))) => {
            let def = Fungi::built_in().get(&form.fungus);
            (
                form.grams,
                def.map_or(UNKNOWN_PER_KG, |def| vegetable_per_kg(def.abundance)),
            )
        }
        (ItemKind::Egg, Some(ItemForm::Egg(form))) => (form.grams, EGG_PER_KG),
        (ItemKind::Flour, Some(ItemForm::Flour(form))) => (form.grams, FLOUR_PER_KG),
        (ItemKind::Log, Some(ItemForm::Log(form))) => (item.own_weight(), per_kg(form.wood)),
        (ItemKind::Kindling, Some(ItemForm::Kindling(form))) => {
            (item.own_weight(), per_kg(form.wood))
        }
        (ItemKind::Ash, _) => (item.own_weight(), ASH_PER_KG),
        (
            ItemKind::Fruit
            | ItemKind::Vegetable
            | ItemKind::Fungus
            | ItemKind::Egg
            | ItemKind::Flour
            | ItemKind::Log
            | ItemKind::Kindling,
            _,
        ) => (item.own_weight(), UNKNOWN_PER_KG),
        _ => return None,
    };
    Some(grams as u64 * per_kg / 1000)
}

fn fruit_per_kg(abundance: fruit::Abundance) -> u64 {
    match abundance {
        fruit::Abundance::Common => 1_500,
        fruit::Abundance::OrchardOnly => 2_000,
        fruit::Abundance::Uncommon => 2_500,
        fruit::Abundance::Rare => 6_000,
    }
}

/// Vegetables', pulses', grains' and fungi's.
fn vegetable_per_kg(abundance: vegetable::Abundance) -> u64 {
    match abundance {
        vegetable::Abundance::Common => 1_500,
        vegetable::Abundance::Uncommon => 2_500,
        vegetable::Abundance::Rare => 6_000,
        vegetable::Abundance::VeryRare => 15_000,
    }
}
