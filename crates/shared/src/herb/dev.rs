//! The herb dev tools' Give Item makes, chosen the same way on both sides so its
//! preview shows what the server gives.

use super::{HerbDef, Herbs};
use crate::rng::Seed;

/// The herb with this key, or with none, one rolled from `seed` (`item_form::batch_seed`, as other kinds are) (any herb in
/// the table alike). `None` if the table has no such herb, or none at all.
pub fn dev_herb<'a>(herbs: &'a Herbs, key: Option<&str>, seed: Seed) -> Option<&'a HerbDef> {
    if let Some(key) = key {
        return herbs.get(key);
    }
    let count = herbs.len();
    if count == 0 {
        return None;
    }
    let at = seed.derive("dev herb").value() % count as u64;
    herbs.iter().nth(at as usize)
}
