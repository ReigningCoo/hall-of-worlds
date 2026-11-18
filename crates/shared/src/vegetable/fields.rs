//! What a farm's fields grow (owner, 2026-11-17): each field of crops (not
//! pasture, not an orchard) one crop on every cell, chosen by the farm's biome
//! from the cultivated ones (`field_crops`), rolled from the site's seed and
//! the field's first cell. A field is sown at once, so its plants share a
//! phase and grow together, until each is picked.

use super::place::field_crops;
use super::{VegetablePlant, Vegetables};
use crate::climate::Biome;
use crate::rng::Seed;
use crate::site::FieldLook;

/// The vegetable plants in `fields` at a farm in `biome`, from `seed` (the
/// site's), by cell.
pub fn plant_fields(
    vegetables: &Vegetables,
    fields: &[FieldLook],
    biome: Biome,
    seed: Seed,
) -> Vec<((i64, i64), VegetablePlant)> {
    let crops = field_crops(vegetables, biome);
    let mut plants = Vec::new();
    if crops.is_empty() {
        return plants;
    }
    let sown = (fields.iter()).filter(|field| !field.livestock && !field.orchard);
    for field in sown {
        let rect = field.rect;
        let at = seed.derive("field crop").at(&[rect.x, rect.y]);
        let def = crops[(at % crops.len() as u64) as usize];
        let phase = seed.derive("field phase").at(&[rect.x, rect.y]) as u32;
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                let plant = VegetablePlant {
                    vegetable: def.key.clone(),
                    phase,
                    picked: None,
                };
                plants.push(((x, y), plant));
            }
        }
    }
    plants
}
