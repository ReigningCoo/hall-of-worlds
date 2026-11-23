//! Air knocking creatures back, and a vacuum pulling them in (magic plan PR
//! 12): how far, by the Air skill and the count, and where a creature's
//! circle ends up along that way without passing walls (`clearance`). The
//! server shares the distance out by the target's resistance, as harm.
//! Numbers are first guesses.

use crate::clearance::check_segment;
use crate::combat::skill_percent;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::skill::Skills;
use crate::spot::{CELL_SPAN, Spot};

use super::effect::Effect;
use super::line::ahead;

/// How far one Air knocks, in units, at no skill: a cell; twice that at 100%.
pub const KNOCK_UNITS: u64 = CELL_SPAN as u64;
/// The furthest anything is knocked, in units: 5 cells.
pub const MOST_KNOCK: u64 = 5 * CELL_SPAN as u64;

/// How far `count` Airs knock, in units, by the caster's Air skill.
pub fn knock_units(count: u32, skills: Option<&Skills>) -> u64 {
    let percent = skills.map_or(0, |s| skill_percent(s.value(Effect::Air.skill())));
    (KNOCK_UNITS * (100 + percent) / 100 * u64::from(count)).min(MOST_KNOCK)
}

/// Where a creature of `size` at `at` ends up knocked `units` going
/// `heading`: as far along that way as its circle can go (halving the way
/// until a move there is allowed), or where it is.
pub fn knocked_to(level: &Level, size: Size, at: Spot, heading: Heading, units: u64) -> Spot {
    let mut units = units;
    while units >= 8 {
        let to = ahead(at, heading, units);
        if check_segment(level, size, at, to).is_some() {
            return to;
        }
        units /= 2;
    }
    at
}
