//! What a kilogram of each material is worth: first guesses.

use crate::material::{Category, Material};

/// How much dearer each tier is than the plainest (tiers 1 to 4).
const TIER_TIMES: [u64; 4] = [1, 3, 10, 40];

/// A gem's worth a kilogram by tier: 20 copper a gram for the plainest up to
/// 2,000 for diamonds.
const GEM_PER_KG: [u64; 4] = [20_000_000, 100_000_000, 500_000_000, 2_000_000_000];

/// A kilogram of `material` in fine units. Metals are each their own, as
/// silver and gold are worth far more than their tier says.
pub fn per_kg(material: Material) -> u64 {
    let tier = (material.tier().clamp(1, 4) - 1) as usize;
    match material {
        Material::Copper => 6_000,
        Material::Iron => 8_000,
        Material::Brass => 10_000,
        Material::Bronze => 12_000,
        Material::Steel => 20_000,
        Material::Silver => 1_000_000,
        Material::Mithril => 2_000_000,
        Material::Adamantine => 3_000_000,
        Material::Gold => 12_000_000,
        _ => match material.category() {
            Category::Gem => GEM_PER_KG[tier],
            category => plainest(category) * TIER_TIMES[tier],
        },
    }
}

/// A kilogram of the plainest material of a category.
fn plainest(category: Category) -> u64 {
    match category {
        Category::Wood => 1_000,
        Category::Plant => 1_500,
        Category::Bone => 4_000,
        Category::Leather | Category::Textile => 6_000,
        Category::Metal => 8_000,
        Category::Gem => GEM_PER_KG[0],
    }
}
