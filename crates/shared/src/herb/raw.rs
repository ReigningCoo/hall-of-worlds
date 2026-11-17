//! Eating a herb raw: a weak, short dose of its first effect (owner: eating one
//! raw teaches its first effect and gives a weak dose of it). Potions (to come)
//! draw out all four, stronger.

use super::{HerbDef, Stat, potency};
use crate::bonus::{self, Bonus, Effect, EndsOn, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::Quality;
use crate::vitals::Vital;

/// How long a raw dose lasts: 30 seconds of action ticks. First guess.
pub const RAW_TICKS: u64 = 30 * ACTION_TICKS_PER_SECOND as u64;
/// An attribute's change for each step of an effect's strength (1 to 3), in
/// thousandths of a point, at a common herb's potency. First guess.
pub const RAW_ATTRIBUTE: i32 = 1000;
/// Health or Mana restored or drained a second for each step of strength, in
/// thousandths of a point, at a common herb's potency. First guess.
pub const RAW_VITAL: i32 = 500;

/// What eating one of `def` raw at this quality does from `now`, for a user
/// with this much Herbalism (`skill_value`): its first effect, scaled by the
/// herb's `potency`, for `RAW_TICKS`; nothing ends it early. `None` for a herb
/// with no effects (a mod's mistake `HerbDef::check` refuses).
pub fn raw_dose(def: &HerbDef, quality: Quality, herbalism: u32, now: u64) -> Option<Effect> {
    let first = def.effects.first()?;
    let scale = |per_step: i32| {
        let milli = i64::from(first.change) * i64::from(per_step);
        (milli * i64::from(potency(quality, herbalism)) / 100) as i32
    };
    let vital = |vital: Vital, amount: i32| match amount > 0 {
        true => Bonus::flat(bonus::Stat::Restore(vital), amount),
        false => Bonus::flat(bonus::Stat::Drain(vital), -amount),
    };
    let bonus = match first.stat {
        Stat::Health => vital(Vital::Health, scale(RAW_VITAL)),
        Stat::Mana => vital(Vital::Mana, scale(RAW_VITAL)),
        stat => {
            let attribute = stat.attribute()?;
            Bonus::flat(bonus::Stat::Attribute(attribute), scale(RAW_ATTRIBUTE))
        }
    };
    Some(Effect {
        source: Source::RawHerb,
        bonuses: vec![bonus],
        start: now,
        end: Some(now + RAW_TICKS),
        ends_on: EndsOn::NOTHING,
    })
}
