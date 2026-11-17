//! The fruit dev tools' Give Item makes, chosen the same way on both sides so
//! its preview shows what the server gives.

use super::{FruitDef, Fruits};
use crate::rng::Seed;

/// The fruit item with this key, or with none, one rolled from `seed`
/// (`item_form::batch_seed`), any fruit item alike. Herbs' fruits are never
/// given (they are herbs). `None` for no such fruit item.
pub fn dev_fruit(key: Option<&str>, seed: Seed) -> Option<&'static FruitDef> {
    let fruits = Fruits::built_in();
    if let Some(key) = key {
        return fruits.get(key).filter(|def| def.is_item());
    }
    let count = fruits.items().count();
    let at = seed.derive("dev fruit").value() % count.max(1) as u64;
    fruits.items().nth(at as usize)
}
