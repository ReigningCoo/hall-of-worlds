//! The server's side of a process once planned: whether it works, and luck.

use super::{Planned, Product, luck_steps, moved, succeeds};
use crate::item::Quality;

/// What a process gave.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// It didn't work: the ingredients are used up and practice still given,
    /// but nothing is made or learned.
    Failed,
    /// It worked: the product (perhaps `Product::Nothing`) at this quality
    /// (none for nothing).
    Made {
        product: Product,
        quality: Option<Quality>,
    },
}

/// Settles a planned process with the server's two rolls, each from 0 to
/// `CERTAIN` − 1: `chance_roll` against its chance, then `luck_roll` for
/// steps above its least quality (`luck_steps`).
pub fn outcome(planned: &Planned, chance_roll: u32, luck_roll: u32) -> Outcome {
    if !succeeds(planned.chance, chance_roll) {
        return Outcome::Failed;
    }
    Outcome::Made {
        product: planned.product.clone(),
        quality: planned.quality.map(|q| moved(q, luck_steps(luck_roll))),
    }
}
