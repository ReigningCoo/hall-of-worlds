//! Kit that works for the mob wearing it: armour it is too weak or clumsy for
//! (`shared::requirement`) is made again, a few times at most.

use shared::item::Item;
use shared::requirement::{FULL, effectiveness, item_requirements};
use shared::sheet::Attributes;

/// How well a piece must work for a mob to keep it: half (in thousandths).
/// Mobs mind drawbacks less than players do. The owner's first guess.
pub(super) const GOOD_ENOUGH: u32 = FULL / 2;

/// How many times a piece is made before settling for the lightest of them.
pub(super) const TRIES: u32 = 4;

/// Makes a piece with `make` until one works at `GOOD_ENOUGH` or better for
/// someone with these `attributes`, up to `TRIES` times; failing that, the
/// lightest made, as that is likely to ask the least. Things that ask nothing (clothes,
/// blades) are kept the first time.
pub(super) fn suited(attributes: &Attributes, mut make: impl FnMut() -> Item) -> Item {
    let mut kept = make();
    for _ in 1..TRIES {
        if works_for(&kept, attributes) {
            return kept;
        }
        let next = make();
        if works_for(&next, attributes) || next.total_weight() < kept.total_weight() {
            kept = next;
        }
    }
    kept
}

/// Whether `item` works well enough for someone with these `attributes`.
pub(super) fn works_for(item: &Item, attributes: &Attributes) -> bool {
    effectiveness(&item_requirements(item), attributes) >= GOOD_ENOUGH
}

#[cfg(test)]
mod tests;
