//! What a farm's orchards grow (owner, 2026-11-17: a basic orchard of a few
//! rows suiting the place's climate): in each orchard field (`FieldLook::orchard`)
//! a row of one fruit every `ROW_GAP` cells, chosen from the site's seed and
//! the row's first cell. Plants with a trunk stand `ROW_GAP` apart along their
//! row, smaller ones side by side. Rows keep a cell clear inside the fence all
//! round, so the gate always leads somewhere and every plant can be reached.

use super::place::{choose, orchard_fruits};
use super::{Abundance, FruitPlant, Fruits};
use crate::rng::Seed;
use crate::site::FieldLook;

/// Cells from one row to the next, and between trunks along a row.
pub const ROW_GAP: i64 = 3;

/// The fruit plants in `fields`' orchards at a place this warm (yearly
/// average, °C), from `seed` (the site's): by cell, with a phase each from its
/// cell, so a row's plants ripen at different times.
pub fn plant_orchards(
    fruits: &Fruits,
    fields: &[FieldLook],
    celsius: i32,
    seed: Seed,
) -> Vec<((i64, i64), FruitPlant)> {
    let suited = orchard_fruits(fruits, celsius);
    let phases = seed.derive("orchard phase");
    let mut plants = Vec::new();
    for field in fields.iter().filter(|field| field.orchard) {
        let rect = field.rect;
        // Rows from the second cell in, never on the last.
        for y in (rect.y + 1..rect.y + rect.height - 1).step_by(ROW_GAP as usize) {
            let mut rng = Seed::new(seed.derive("orchard row").at(&[rect.x, y])).rng();
            let Some(def) = choose(&suited, Abundance::orchard_weight, &mut rng) else {
                return plants;
            };
            let step = if def.plant.has_trunk() { ROW_GAP } else { 1 };
            for x in (rect.x + 1..rect.x + rect.width - 1).step_by(step as usize) {
                let plant = FruitPlant {
                    fruit: def.key.clone(),
                    phase: phases.at(&[x, y]) as u32,
                    picked: None,
                };
                plants.push(((x, y), plant));
            }
        }
    }
    plants
}
