//! What a food is called in a dish's generic name ("Mutton and Turnip
//! Stew"). English for now, as other items' names are.

use super::Source;
use crate::farmyard::{honey_name, milk_name};
use crate::fruit::Fruits;
use crate::fungus::Fungi;
use crate::herb::Herbs;
use crate::livestock::Livestock;
use crate::vegetable::Vegetables;

/// The name of the food with this source and key in a dish: a herb's,
/// fruit's or vegetable's own name; an animal's meat ("Beef"), milk ("Milk"
/// a cow's, "Goat's Milk") or honey;
/// an egg as "Egg" (a hen's) or "Duck Egg"; a part's key in words. Fungi are
/// all "Mushroom", so a dish never names one its eater doesn't know. The key
/// in words for anything a table lacks.
pub fn food_name(source: Source, key: &str) -> String {
    let named = match source {
        Source::Herb => Herbs::built_in_ref().get(key).map(|def| def.name.clone()),
        Source::Fruit => Fruits::built_in().get(key).map(|def| def.name.clone()),
        Source::Vegetable => Vegetables::built_in().get(key).map(|def| def.name.clone()),
        Source::Fungus => Fungi::built_in().get(key).map(|_| "Mushroom".to_owned()),
        Source::Meat => Livestock::built_in()
            .get(key)
            .map(|def| words(&def.meat.name)),
        Source::Milk if key == "cattle" => Some("Milk".to_owned()),
        Source::Milk => Some(milk_name(key)),
        Source::Egg if key == "chicken" => Some("Egg".to_owned()),
        Source::Egg => Some(crate::farmyard::egg_name(key)),
        Source::Honey => Some(honey_name(key)),
        Source::Part => None,
    };
    named.unwrap_or_else(|| words(key))
}

/// "leaf_lard" as "Leaf Lard": each word capitalised.
fn words(key: &str) -> String {
    let words: Vec<String> = (key.split(['_', ' ']))
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect();
    words.join(" ")
}
