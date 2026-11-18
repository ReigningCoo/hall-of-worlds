//! The flour the dev tools' Give Item makes, chosen the same way on both sides
//! so its preview shows what the server gives.

use crate::rng::Seed;
use crate::vegetable::{VegetableDef, VegetableSet, Vegetables};

/// Every grain, in the table's order: those that mill into flour.
pub fn grains() -> impl Iterator<Item = &'static VegetableDef> {
    (Vegetables::built_in().iter()).filter(|def| def.set == VegetableSet::Grain)
}

/// The grain with this key, or with none, one rolled from `seed`
/// (`item_form::batch_seed`), any alike. `None` for no such grain.
pub fn dev_flour(key: Option<&str>, seed: Seed) -> Option<&'static VegetableDef> {
    if let Some(key) = key {
        return grains().find(|def| def.key == key);
    }
    let count = grains().count();
    let at = seed.derive("dev flour").value() % count.max(1) as u64;
    grains().nth(at as usize)
}
