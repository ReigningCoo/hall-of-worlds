//! What eating does: the effects laid on whoever eats a food, healing if it
//! suits them, poison if it doesn't and poisons.

use super::effect::{Effect, EndsOn, Source};
use super::food::{Others, food};
use super::stat::{Bonus, Stat};
use crate::creature::CreatureKind;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::ItemKind;
use crate::vitals::{PER_POINT, Vital};

/// Health poison takes a second, in thousandths of a point (never below 1).
pub const POISON_DRAIN: u32 = PER_POINT;
/// How long poison lasts: 60 seconds of action ticks.
pub const POISON_TICKS: u64 = 60 * ACTION_TICKS_PER_SECOND as u64;

/// The healing of eating one of these at `now`, for whoever it suits: Health
/// given back a second, ended by moving or being hurt.
pub fn food_effect(kind: ItemKind, now: u64) -> Option<Effect> {
    let food = food(kind)?;
    Some(Effect {
        source: Source::Eaten(kind),
        bonuses: vec![Bonus::flat(
            Stat::Restore(Vital::Health),
            food.restore as i32,
        )],
        start: now,
        end: Some(now + food.ticks),
        ends_on: EndsOn {
            moving: true,
            hurt: true,
        },
    })
}

/// Poison from `now`: 1 Health a second for 60 s, never below 1 Health,
/// ended by nothing but running out (owner, 2026-11-14: a cure later).
pub fn poison(now: u64) -> Effect {
    Effect {
        source: Source::Poison,
        bonuses: vec![Bonus::flat(Stat::Drain(Vital::Health), POISON_DRAIN as i32)],
        start: now,
        end: Some(now + POISON_TICKS),
        ends_on: EndsOn::NOTHING,
    }
}

/// The effects of a creature of kind `eater` eating one of these at `now`:
/// `None` if it can't be eaten or is refused to that kind (`food::may_eat`).
pub fn eating(kind: ItemKind, eater: CreatureKind, now: u64) -> Option<Vec<Effect>> {
    let food = food(kind)?;
    if food.suits(eater) {
        return food_effect(kind, now).map(|effect| vec![effect]);
    }
    match food.others {
        Others::Refused => None,
        Others::Poisoned => Some(vec![poison(now)]),
    }
}
