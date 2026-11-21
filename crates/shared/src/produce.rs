//! Fruit, nuts, vegetables, pulses and grains suiting a place, for what town
//! shops sell and what mobs carry (owner, 2026-11-18): until then both gave
//! only apples. Only the server calls it, as the seeds are secret.
//!
//! A shop sells what is grown near it (orchards and fields: `Source::Grown`);
//! a mob has what it picked up (`Source::Foraged`), what grows wild there,
//! or what is grown there when nothing does. Either way the commoner more
//! often, chosen as `herb::wild_herbs` chooses herbs: by biome and warmth.
//! Eggs, and milk and honey wherever they are held, are of an animal kept
//! there (`farmyard::animal_for`).

use crate::farmyard::{self, EggForm};
use crate::flour::FlourForm;
use crate::fruit::{self, Abundance as FruitAbundance, FruitDef, FruitForm, Fruits};
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::livestock::Product;
use crate::mob::Habitat;
use crate::potion::Liquid;
use crate::rng::Seed;
use crate::vegetable::{self, VegetableDef, VegetableForm, VegetableSet, Vegetables};

/// Where food comes from, which decides what may be chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Grown in orchards and fields near by: what shops sell.
    Grown,
    /// Picked up in the wild: what mobs carry.
    Foraged,
}

/// The fruit grown near a place like this: the fruit items whose biomes and
/// warmth suit it, cultivated ones too, else what an orchard there grows
/// (by warmth alone).
pub fn grown_fruits<'a>(fruits: &'a Fruits, habitat: &Habitat) -> Vec<&'a FruitDef> {
    let suited: Vec<_> = (fruits.items())
        .filter(|def| def.biomes.contains(&habitat.biome) && def.suits_warmth(habitat.celsius))
        .collect();
    match suited.is_empty() {
        true => fruit::orchard_fruits(fruits, habitat.celsius),
        false => suited,
    }
}

/// One fruit for a place like this, from `seed`: grown ones by
/// `orchard_weight`, foraged ones by `wild_weight` (grown ones where nothing
/// grows wild).
pub fn fruit_for<'a>(
    fruits: &'a Fruits,
    habitat: &Habitat,
    source: Source,
    seed: Seed,
) -> Option<&'a FruitDef> {
    let mut rng = seed.derive("produce fruit").rng();
    let wild = fruit::wild_fruits(fruits, habitat);
    if source == Source::Foraged && !wild.is_empty() {
        return fruit::choose(&wild, FruitAbundance::wild_weight, &mut rng);
    }
    let grown = grown_fruits(fruits, habitat);
    fruit::choose(&grown, FruitAbundance::orchard_weight, &mut rng)
}

/// One vegetable, pulse or grain for a place like this, from `seed`, the
/// commoner more often: grown ones from a farm's crops there
/// (`field_crops`); foraged ones from what grows wild there, farmed or not,
/// else those crops.
pub fn vegetable_for<'a>(
    vegetables: &'a Vegetables,
    habitat: &Habitat,
    source: Source,
    seed: Seed,
) -> Option<&'a VegetableDef> {
    let mut rng = seed.derive("produce vegetable").rng();
    let biome = habitat.biome;
    let mut wild = vegetable::wild_crops(vegetables, biome);
    wild.extend(vegetable::wild_only_vegetables(vegetables, biome));
    if source == Source::Foraged && !wild.is_empty() {
        return vegetable::choose_wild(&wild, &mut rng);
    }
    vegetable::choose_wild(&vegetable::field_crops(vegetables, biome), &mut rng)
}

/// The grain a place like this mills into flour, from `seed`, the commoner
/// more often: one of the farms' crops there (`field_crops`) that is a grain,
/// else any grain (a miller buys in what isn't grown near).
pub fn flour_grain_for<'a>(
    vegetables: &'a Vegetables,
    habitat: &Habitat,
    seed: Seed,
) -> Option<&'a VegetableDef> {
    let mut rng = seed.derive("produce flour").rng();
    let is_grain = |def: &&VegetableDef| def.set == VegetableSet::Grain;
    let grown: Vec<_> = (vegetable::field_crops(vegetables, habitat.biome).into_iter())
        .filter(is_grain)
        .collect();
    if !grown.is_empty() {
        return vegetable::choose_wild(&grown, &mut rng);
    }
    let any: Vec<_> = vegetables.iter().filter(is_grain).collect();
    vegetable::choose_wild(&any, &mut rng)
}

/// Makes a fruit, vegetable, flour or egg item, or the milk or honey in a
/// bottle, vessel or cask, one suiting a place like this, chosen from its own
/// id's seed, so the same item is always the same; anything else is left
/// alone. Its count and everything else stay as they were.
pub fn suit_to_place(item: &mut Item, habitat: &Habitat, source: Source) {
    let seed = item.id.seed();
    suit_liquid(item, habitat);
    let form = match item.kind {
        ItemKind::Fruit => fruit_for(Fruits::built_in(), habitat, source, seed)
            .map(|def| ItemForm::Fruit(FruitForm::new(def, seed))),
        ItemKind::Vegetable => vegetable_for(Vegetables::built_in(), habitat, source, seed)
            .map(|def| ItemForm::Vegetable(VegetableForm::new(def, seed))),
        ItemKind::Flour => flour_grain_for(Vegetables::built_in(), habitat, seed)
            .and_then(FlourForm::of)
            .map(ItemForm::Flour),
        ItemKind::Egg => farmyard::animal_for(Product::Egg, habitat.biome, seed)
            .and_then(EggForm::of)
            .map(ItemForm::Egg),
        _ => None,
    };
    if form.is_some() {
        item.form = form;
    }
}

/// The milk or honey a bottle, vessel or cask holds made an animal's kept
/// there; any other liquid is left alone.
fn suit_liquid(item: &mut Item, habitat: &Habitat) {
    let seed = item.id.seed();
    let liquid = match &mut item.form {
        Some(ItemForm::Bottle(bottle)) => bottle.liquid.as_mut(),
        Some(ItemForm::Vessel(vessel)) => vessel.liquid.as_mut(),
        Some(ItemForm::Cask(cask)) => cask.liquid.as_mut(),
        _ => None,
    };
    let (product, animal) = match liquid {
        Some(Liquid::Milk(animal)) => (Product::Milk, animal),
        Some(Liquid::Honey(animal)) => (Product::Honey, animal),
        _ => return,
    };
    if let Some(def) = farmyard::animal_for(product, habitat.biome, seed) {
        *animal = def.key.clone();
    }
}

#[cfg(test)]
mod tests;
