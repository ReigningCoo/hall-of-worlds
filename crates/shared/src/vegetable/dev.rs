//! The vegetable the dev tools' Give Item makes, chosen the same way on both
//! sides so its preview shows what the server gives.

use super::{VegetableDef, Vegetables};
use crate::rng::Seed;

/// The vegetable with this key, or with none, one rolled from `seed`
/// (`item_form::batch_seed`), any alike. `None` for no such vegetable.
pub fn dev_vegetable(key: Option<&str>, seed: Seed) -> Option<&'static VegetableDef> {
    let vegetables = Vegetables::built_in();
    if let Some(key) = key {
        return vegetables.get(key);
    }
    let count = vegetables.len();
    let at = seed.derive("dev vegetable").value() % count.max(1) as u64;
    vegetables.iter().nth(at as usize)
}
