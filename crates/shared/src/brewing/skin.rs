//! Ointment rubbed on oneself (decision 8): a quarter of each dose (`on_skin`),
//! attributes for as long as a potion of its quality lasts (`duration_ticks`),
//! Health and Mana at once (decision 17, drinking's rule, so both agree).

use super::{duration_ticks, on_skin, quality};
use crate::bonus::{Bonus, Effect, EndsOn, Source, Stat as BonusStat};
use crate::dungeon::darkvision_reach;
use crate::herb::Stat;
use crate::item::Quality;
use crate::potion::Contents;
use crate::vitals::{PER_POINT, Vital};

/// Health or Mana restored at once for each step, in whole points (decision 17).
pub const VITAL_PER_STEP: i32 = 5;
/// An attribute's change for each step, in thousandths of a point.
pub const ATTRIBUTE_PER_STEP: i32 = 1000;

/// What doses do when they take: a timed effect on attributes (none if no
/// attribute is moved), and Health or Mana given (or taken, below 0) at once,
/// in thousandths of a point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Applied {
    pub effect: Option<Effect>,
    pub vitals: Vec<(Vital, i32)>,
}

/// What these doses do from `now`, for `lasts` ticks, the effect from
/// `source`; Darkvision reaching as far as `quality` lets it
/// (`dungeon::darkvision_reach`).
pub fn applied(
    contents: &Contents,
    quality: Quality,
    lasts: u64,
    source: Source,
    now: u64,
) -> Applied {
    let mut bonuses = Vec::new();
    let mut vitals = Vec::new();
    for dose in contents.doses() {
        let step = i32::from(dose.step);
        match dose.stat {
            Stat::Health => vitals.push((Vital::Health, step * VITAL_PER_STEP * PER_POINT as i32)),
            Stat::Mana => vitals.push((Vital::Mana, step * VITAL_PER_STEP * PER_POINT as i32)),
            Stat::Darkvision => {
                let cells = darkvision_reach(dose.step, quality);
                if cells > 0 {
                    bonuses.push(Bonus::flat(BonusStat::Darkvision, cells as i32 * 1000));
                }
            }
            stat => {
                if let Some(attribute) = stat.attribute() {
                    let amount = step * ATTRIBUTE_PER_STEP;
                    bonuses.push(Bonus::flat(BonusStat::Attribute(attribute), amount));
                }
            }
        }
    }
    let effect = (!bonuses.is_empty()).then(|| Effect {
        source,
        bonuses,
        start: now,
        end: Some(now + lasts),
        ends_on: EndsOn::NOTHING,
    });
    Applied { effect, vitals }
}

/// An ointment holding `contents` rubbed on oneself at `now`: a quarter of
/// each dose, lasting as long as the ointment's own quality says. Only the
/// latest ointment's effect counts (`Source::Ointment`).
pub fn skin_effect(contents: &Contents, now: u64) -> Applied {
    let quality = quality(contents);
    let lasts = duration_ticks(quality);
    applied(&on_skin(contents), quality, lasts, Source::Ointment, now)
}
