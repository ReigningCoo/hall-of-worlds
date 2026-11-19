//! The Alembic step (Alchemy plan PR 5): distils what came before. The
//! alcohol fed in sets how far its volume is reduced, rounded down, pure
//! alcohol the most; the doses and the alcohol stay at full amount, so a
//! potion comes out stronger in fewer bottles, and drinks with nothing else
//! in them come over as pure alcohol. Salt Water's salt stays behind as Salt.

use super::Refused;
use super::batch::Batch;
use crate::potion::Liquid;

/// Each this many tenths of a percent of alcohol fed in divides the volume
/// once more (Claude's first guess, for the owner to tune): under 10% (ale,
/// cider) not at all, wine (10% to 19.9%) by 2, pure alcohol by 11.
pub const STRENGTH_A_DIVISION: u32 = 100;

/// What the volume is divided by, fed alcohol of `strength` (tenths of a
/// percent), or nothing (0).
pub fn reduced_by(strength: u32) -> u32 {
    1 + strength / STRENGTH_A_DIVISION
}

/// Distils `batch`, fed `fed` or nothing.
pub(super) fn alembic(batch: &mut Batch, fed: Option<&Liquid>) -> Result<(), Refused> {
    if batch.units == 0 {
        return Err(Refused::NothingToDistil);
    }
    let strength = match fed {
        None => 0,
        Some(liquid) => match liquid.alcohol_strength() {
            0 => return Err(Refused::NotAlcohol),
            strength => strength,
        },
    };
    let divisor = reduced_by(strength);
    batch.units /= divisor;
    if !batch.mix.is_empty() {
        // A potion is never boiled away entirely.
        batch.units = batch.units.max(1);
    }
    batch.distilled = divisor > 1;
    batch.salt_made += std::mem::take(&mut batch.salt);
    Ok(())
}
