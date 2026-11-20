//! A herd's members' numbers, and so their ids and ages (`livestock::age_hours`
//! works an age out from the id alone): picked from a few tries each so a
//! herd's young were born in one season and its grown ones are grown while the
//! young are small. Nothing is saved: the same numbers come every time.

use super::wild::{PURPOSE, wild_mob_id};
use crate::livestock::{Stages, age_hours};
use crate::rng::Seed;
use crate::wilderness::BlockPos;

/// What a member's number is rolled for, as `Seed::at`'s fourth coordinate.
const NUMBER_ROLL: i64 = 0x004e_554d;

/// What a herd's season is rolled for.
const SEASON_ROLL: i64 = 0x5345_4153;

/// How many numbers are tried for a member of an age: enough that even a
/// horse's foals (young a 32nd of their cycle) are nearly always found.
const AGE_TRIES: i64 = 256;

/// How old a member is to be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Age {
    /// Any age (wolves, or a herd with no young).
    Any,
    /// Born in the herd's season: young while the season is.
    Young,
    /// Grown while the herd's young are young: from one to three times the
    /// kind's growing up older than them (more than once, so never growing
    /// itself while they are small).
    Grown,
}

/// The block's herd: where its numbers come from, and its season.
pub(super) struct Herd {
    pub seed: Seed,
    pub block: BlockPos,
}

impl Herd {
    /// The hour in its kind's age cycle (`age_hours` at the world's start)
    /// its young were born at.
    fn season(&self, stages: &Stages) -> u32 {
        let cycle = stages.juvenile_until.max(1) * crate::livestock::LIFE_CYCLES;
        let roll = self
            .seed
            .at(&[PURPOSE, self.block.x, self.block.y, SEASON_ROLL]);
        (roll % u64::from(cycle)) as u32
    }

    /// The `try_`th candidate number for a member.
    fn candidate(&self, member: i64, try_: i64) -> u32 {
        let (x, y) = (self.block.x, self.block.y);
        self.seed.at(&[PURPOSE, x, y, NUMBER_ROLL, member, try_]) as u32
    }

    /// A member's number, for an age of its kind's `stages` (none for kinds
    /// that never grow up: wolves); `None` if no number of that age was
    /// found (then a young one is left out).
    pub fn number(&self, member: i64, age: Age, stages: Option<&Stages>) -> Option<u32> {
        let first = self.candidate(member, 0);
        let Some(stages) = stages.filter(|_| age != Age::Any) else {
            return Some(first);
        };
        let cycle = stages.juvenile_until.max(1) * crate::livestock::LIFE_CYCLES;
        let season = self.season(stages);
        let fits = |number: u32| {
            let born = age_hours(wild_mob_id(self.seed, number), 0, stages);
            let older = (born + cycle - season) % cycle;
            match age {
                Age::Young => older < stages.young_until,
                // Grown by the time the young are born, until they are.
                _ => (stages.juvenile_until + stages.young_until..3 * stages.juvenile_until)
                    .contains(&older),
            }
        };
        let found = (0..AGE_TRIES)
            .map(|try_| self.candidate(member, try_))
            .find(|&number| fits(number));
        match age {
            Age::Young => found,
            _ => Some(found.unwrap_or(first)),
        }
    }
}
