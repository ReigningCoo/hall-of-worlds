//! Casting's timing and Heal as an effect (magic plan PR 5): the wind-up
//! before the first step, a second between steps, and how much a Heal step
//! restores. The server casts by these; the client shows the wait by them.

use crate::magic::{HEAL_COST, heal_amount};
use crate::skill::Skills;
use crate::vitals::Points;

/// Energy a cast costs, as the Heal ability did: a second at normal speed.
pub const CAST_COST: u32 = HEAL_COST;

/// The wind-up before the first step lands: half of it.
pub const CAST_WIND_UP: u32 = CAST_COST / 2;

/// Real milliseconds between one step and the next (a first guess).
pub const STEP_MILLIS: u64 = 1000;

/// What `count` Heals restore, by the caster's Healing: today's line
/// (`magic::heal_amount`) each, as casting it that many times.
pub fn heal_restores(count: u32, skills: Option<&Skills>) -> Points {
    Points(heal_amount(skills).milli().saturating_mul(count))
}
