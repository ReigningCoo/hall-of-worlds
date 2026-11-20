//! The fungus the dev tools' Give Item makes, chosen the same way on both
//! sides so its preview shows what the server gives.

use super::{Fungi, FungusDef};
use crate::rng::Seed;

/// The fungus with this key, or with none, one rolled from `seed`
/// (`item_form::batch_seed`), any alike. `None` for no such fungus.
pub fn dev_fungus(key: Option<&str>, seed: Seed) -> Option<&'static FungusDef> {
    let fungi = Fungi::built_in();
    if let Some(key) = key {
        return fungi.get(key);
    }
    let count = fungi.len();
    let at = seed.derive("dev fungus").value() % count.max(1) as u64;
    fungi.iter().nth(at as usize)
}
