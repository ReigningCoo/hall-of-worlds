//! The Pelican (Alchemy plan PR 7): circulation in a reflux vessel. It changes
//! nothing in the batch but uses a step, and the product's purity score
//! (`brewing::purity`) is raised by `CIRCULATION_PERCENT` before it becomes a
//! quality, so it helps a product close to the next quality.

use super::Refused;
use super::batch::Batch;
use crate::brewing::{purity, quality_of_purity};
use crate::item::Quality;
use crate::potion::Contents;

/// Percent each Pelican adds to the purity score (the owner's 20%); two
/// Pelicans add 40%.
pub const CIRCULATION_PERCENT: i32 = 20;

/// Circulates `batch` once.
pub(super) fn pelican(batch: &mut Batch) -> Result<(), Refused> {
    if batch.is_empty() {
        return Err(Refused::NothingToCirculate);
    }
    batch.circulated += 1;
    Ok(())
}

/// The purity quality of `contents` after `circulated` Pelicans: the purity
/// score × (100% + `CIRCULATION_PERCENT` each), rounded down, then banded.
pub fn circulated_quality(contents: &Contents, circulated: u32) -> Quality {
    let percent = 100 + CIRCULATION_PERCENT * circulated as i32;
    quality_of_purity(purity(contents) * percent / 100)
}
