//! A creature's age and life stage, worked out and never saved (livestock
//! plan PR 6): from its id and the world's clock. Each kind lives through a
//! cycle of `LIFE_CYCLES` times its growing up (young and juvenile), starting
//! from a point its id picks, so a herd is a mix of ages that grows up as the
//! world's time passes. Nothing dies of old age (owner): at the cycle's end a
//! creature's age starts again, as if another took its place.

use super::{Livestock, Stage, Stages};
use crate::creature::CreatureKind;
use crate::id::Id;

/// How many times its growing up a kind's ages cycle over: one in this many of
/// a kind is still growing at any time.
pub const LIFE_CYCLES: u32 = 4;

/// What an age is rolled for, as `Seed::at`'s first coordinate.
const AGE_ROLL: i64 = 0x41_4745;

/// Game minutes in a game hour.
const MINUTES_AN_HOUR: u64 = 60;

/// Its age in game hours at this many game minutes into the world's life.
pub fn age_hours(id: Id, world_minutes: u64, stages: &Stages) -> u32 {
    let cycle = u64::from(stages.juvenile_until.max(1) * LIFE_CYCLES);
    let start = id.seed().at(&[AGE_ROLL]);
    let hours = world_minutes / MINUTES_AN_HOUR;
    (start.wrapping_add(hours) % cycle) as u32
}

/// The stage of life of a creature of this kind with this id, at this many
/// game minutes into the world's life: `Adult` for kinds not in the livestock
/// table (people, wolves).
pub fn stage_of(kind: CreatureKind, id: Id, world_minutes: u64) -> Stage {
    Livestock::built_in()
        .by_kind(kind)
        .map_or(Stage::Adult, |def| {
            def.stages.at(age_hours(id, world_minutes, &def.stages))
        })
}

#[cfg(test)]
mod tests;
