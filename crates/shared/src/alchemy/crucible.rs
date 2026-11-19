//! The Crucible and the Filter (Alchemy plan PR 6). The Crucible calcines
//! whatever came before into a Powder, ¾ as strong, its liquid, alcohol and
//! salt driven off (coagulation merged in). The Filter takes a Powder's doses:
//! each stat in both what came before and the Powder adds together, a stat in
//! only one is taken out.

use super::batch::Batch;
use super::{Form, Mix, Refused};
use crate::potion::Contents;

/// A Powder's doses in percent of what was calcined (the owner's ¾).
pub const POWDER_PERCENT: i32 = 75;

/// Calcines `batch` into a Powder.
pub(super) fn crucible(batch: &mut Batch) -> Result<(), Refused> {
    if batch.is_empty() {
        return Err(Refused::NothingToCalcine);
    }
    batch.mix.scale(POWDER_PERCENT);
    *batch = Batch {
        mix: batch.mix,
        form: Some(Form::Powder),
        salt_made: batch.salt_made,
        ..Batch::default()
    };
    Ok(())
}

/// Filters `batch` through a Powder holding `powder`.
pub(super) fn filter(batch: &mut Batch, powder: Option<&Contents>) -> Result<(), Refused> {
    if batch.is_empty() {
        return Err(Refused::NothingToFilter);
    }
    let powder = powder.ok_or(Refused::NoPowder)?;
    batch.mix = batch.mix.filtered(&Mix::of(powder));
    Ok(())
}
