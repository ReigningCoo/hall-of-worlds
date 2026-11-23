//! Touch: whoever is within arm's reach in the direction faced, a wider arc
//! for each count.

use crate::combat::{distance, reach_between};
use crate::dungeon::{Level, spots_are_clear};
use crate::facing::{Heading, TURN};

use super::filters::Filters;
use super::hit::{Creature, Hit, Origin};
use super::line::{ahead, wall_stop};

/// Half Touch's arc at ×1: an eighth of a turn each side (a quarter in all).
/// First guess.
pub const TOUCH_HALF_ARC: u16 = (TURN / 8) as u16;

/// How much wider each side each count after the first makes it (a
/// sixteenth of a turn). First guess.
pub const TOUCH_WIDER: u16 = (TURN / 16) as u16;

/// Half the arc Touch ×`count` covers, at most half a turn (all round).
pub fn touch_half_arc(count: u8) -> u16 {
    let extra = u32::from(count.max(1) - 1) * u32::from(TOUCH_WIDER);
    (u32::from(TOUCH_HALF_ARC) + extra).min(TURN / 2) as u16
}

/// What Touch ×`count` from `origin` hits, nearest first; with nobody there,
/// the spot touched, straight ahead.
pub fn touch(
    level: &Level,
    origin: &Origin,
    count: u8,
    filters: Filters,
    creatures: &[Creature],
) -> Vec<Hit> {
    let from = origin.body(creatures);
    let half = touch_half_arc(count);
    let mut found: Vec<(u64, usize)> = creatures
        .iter()
        .enumerate()
        .filter(|(index, other)| {
            Some(*index) != origin.creature
                && other.side.allowed_by(filters)
                && distance(&from, &other.body) <= reach_between(from.size, 0, other.body.size)
                && Heading::towards(origin.at, other.body.at)
                    .is_none_or(|way| way.apart(origin.heading) <= half)
                && spots_are_clear(level, origin.at, other.body.at)
        })
        .map(|(index, other)| (distance(&from, &other.body), index))
        .collect();
    found.sort_unstable();
    if found.is_empty() {
        let reach = from.size.radius().unsigned_abs() + crate::combat::arm_reach(from.size);
        let at = wall_stop(level, origin.at, ahead(origin.at, origin.heading, reach));
        return vec![Hit {
            creature: None,
            at,
            height: origin.height,
            heading: origin.heading,
        }];
    }
    found
        .into_iter()
        .map(|(_, index)| Hit::on(creatures, index, origin.at, origin.heading))
        .collect()
}
