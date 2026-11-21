//! The animal whose milk, eggs or honey a place's shops sell: one kept in its
//! biome, the commoner in cooking more often (`cooking::Usage`), else any
//! giving it (bought in). Only the server calls it, from a shop's poi.

use crate::climate::Biome;
use crate::cooking::{Ingredients, Source, Usage};
use crate::livestock::{LivestockDef, Product};
use crate::rng::Seed;

/// One animal giving `product` for a place in `biome`, from `seed`.
pub fn animal_for(product: Product, biome: Biome, seed: Seed) -> Option<&'static LivestockDef> {
    let mut rng = seed.derive("farmyard animal").rng();
    let kept: Vec<_> = (super::givers(product))
        .filter(|def| def.kept.contains(&biome))
        .collect();
    let choices = match kept.is_empty() {
        true => super::givers(product).collect(),
        false => kept,
    };
    let weighted: Vec<_> = (choices.iter().enumerate())
        .map(|(at, def)| (at, weight(product, def)))
        .collect();
    if weighted.iter().all(|&(_, weight)| weight == 0) {
        return None;
    }
    Some(choices[rng.pick_weighted(&weighted)])
}

/// How often an animal's product is chosen, by how commonly it is cooked.
fn weight(product: Product, def: &LivestockDef) -> u64 {
    let source = match product {
        Product::Milk => Source::Milk,
        Product::Egg => Source::Egg,
        Product::Honey => Source::Honey,
        _ => return 0,
    };
    let row = Ingredients::built_in().get(source, &def.key);
    match row.map(|row| row.usage) {
        Some(Usage::Staple) => 8,
        Some(Usage::Common) => 4,
        Some(Usage::Occasional) => 2,
        _ => 1,
    }
}
