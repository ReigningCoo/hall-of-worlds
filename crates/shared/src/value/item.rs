//! What an item is worth, one of it and a whole stack.

use super::{BASE_MINUTES, FINE_PER_COPPER, LABOUR_PER_MINUTE, liquid, per_kg, quality_times, raw};
use crate::bonus::armour_material;
use crate::item::{Item, ItemCategory, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::{EMPTY_GRAMS, Liquid, Oil};
use crate::pour::BOTTLE_ML;

/// A kilogram of food made from something (dishes, bread), when nothing
/// says what it is made of.
const FOOD_PER_KG: u64 = 3_000;

/// What one of `item` is worth in fine units: its top alone, if a stack, with
/// what is in it, set in it or poured in it. Quest items are worth nothing,
/// and coins what they say.
pub fn each(item: &Item) -> u64 {
    if item.is_quest_item() {
        return 0;
    }
    if let Some(copper) = item.kind.coin_value() {
        return copper * FINE_PER_COPPER;
    }
    let times = quality_times(item.quality.unwrap_or_default());
    let own = raw::of(item).unwrap_or_else(|| made(item));
    // A bottle's quality is its potion's; any other liquid counts as Common.
    let poured = match poured(item) {
        Some((liquid @ Liquid::Potion(_), ml)) => liquid::of(&liquid, ml) * times,
        Some((liquid, ml)) => liquid::of(&liquid, ml) * quality_times(Quality::Common),
        None => 0,
    };
    let inside: u64 = item.contents.iter().map(|placed| total(&placed.item)).sum();
    let gems: u64 = item.set_gems().map(total).sum();
    own * times + poured + inside + gems
}

/// What a whole stack is worth in fine units: every item in it.
pub fn total(item: &Item) -> u64 {
    let stacked: u64 = item.above.iter().map(total).sum();
    each(item) * item.count as u64 + stacked
}

/// A made item's materials and labour, before quality.
fn made(item: &Item) -> u64 {
    let def = item.kind.def();
    let grams = own_grams(item) as u64;
    let materials = match material(item).or_else(|| usual_material(item.kind)) {
        Some(material) => grams * per_kg(material) / 1000,
        None => grams * FOOD_PER_KG / 1000,
    };
    let labour = BASE_MINUTES * def.make_percent / 100 * LABOUR_PER_MINUTE;
    materials + labour
}

/// What it weighs itself, without anything in it, set in it or poured in it.
fn own_grams(item: &Item) -> u32 {
    let form = item.form.as_ref();
    let grams = form
        .and_then(|form| form.grams_at(item.made.size))
        .unwrap_or(item.kind.def().weight);
    match form {
        Some(ItemForm::Bottle(_)) => EMPTY_GRAMS,
        // A gram a millilitre of what is in them.
        Some(ItemForm::Cask(cask)) => grams.saturating_sub(cask.ml),
        Some(ItemForm::Vessel(vessel)) => grams.saturating_sub(vessel.ml),
        Some(ItemForm::Lantern(lantern)) => grams.saturating_sub(lantern.ml),
        _ => grams,
    }
}

/// What is poured in it, and how much.
fn poured(item: &Item) -> Option<(Liquid, u32)> {
    match item.form.as_ref()? {
        ItemForm::Bottle(bottle) => Some((bottle.liquid.clone()?, BOTTLE_ML)),
        ItemForm::Cask(cask) => Some((cask.liquid.clone()?, cask.ml)),
        ItemForm::Vessel(vessel) => Some((vessel.liquid.clone()?, vessel.ml)),
        // A lantern doesn't keep which oil: the plainest.
        ItemForm::Lantern(lantern) => Some((Liquid::Oil(Oil::Neutral), lantern.ml)),
        _ => None,
    }
}

/// What its form says it is made of, the main part if several.
fn material(item: &Item) -> Option<Material> {
    if let Some(material) = armour_material(item) {
        return Some(material);
    }
    Some(match item.form.as_ref()? {
        ItemForm::Blade(blade) => blade.blade_metal,
        ItemForm::Ring(ring) => ring.metal,
        ItemForm::Necklace(necklace) => necklace.metal,
        ItemForm::Gem(gem) => gem.material,
        ItemForm::Footwear(footwear) => footwear.upper,
        ItemForm::Under(under) => under.material,
        ItemForm::Clothing(clothing) => clothing.material,
        ItemForm::Cask(cask) => cask.wood,
        _ => return None,
    })
}

/// What a kind is taken to be made of when its form doesn't say: none for
/// food.
fn usual_material(kind: ItemKind) -> Option<Material> {
    Some(match kind.category() {
        ItemCategory::Weapons => Material::Steel,
        ItemCategory::Armour | ItemCategory::Containers => Material::Leather,
        ItemCategory::Clothing => Material::Linen,
        ItemCategory::Jewellery => Material::Silver,
        ItemCategory::Food => return None,
        ItemCategory::Valuables | ItemCategory::Materials => Material::Wood,
        ItemCategory::Other => Material::Iron,
    })
}
