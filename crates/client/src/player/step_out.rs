//! Stepping out of someone squeezed past when a walk ends inside them (owner,
//! 2026-11-27): to the closest edge of their circle first; back towards where
//! the player came in from only if there is no room there or they stand at
//! the exact centre. The step is an ordinary move away from them, which costs
//! no squeeze (`shared::clearance::crowd_per_mille`).

use bevy::math::DVec2;
use shared::clearance::{Crowding, Other, crowd_per_mille, crowded_at};
use shared::spot::Spot;

use super::stride::{Walker, to_vector};

/// How far outside the two circles touching the player is put, in units, so
/// rounding never leaves them overlapping.
const MARGIN: f64 = 2.0;

/// Where a player standing at `at` inside someone squeezed past steps out to,
/// if they are inside anyone and there is room: `entered` is where they last
/// stood clear of everyone, for the side they came in from.
pub(super) fn step_out(walker: &Walker, at: Spot, entered: Option<Spot>) -> Option<Spot> {
    let inside: Vec<&Other> = walker
        .others
        .iter()
        .filter(|other| other.crowding == Crowding::Squeeze)
        .filter(|other| crowded_at(walker.size, at, std::slice::from_ref(*other)))
        .collect();
    if inside.is_empty() {
        return None;
    }
    let edge = |other: &Other, towards: Spot| {
        let way = to_vector(other.at, towards);
        if way.length() < 1.0 {
            return None;
        }
        let reach = (walker.size.radius() + other.size.radius()) as f64 + MARGIN;
        let by = way / way.length() * reach;
        Some(offset(other.at, by))
    };
    // The closest edges first, nearest the player first.
    let mut closest: Vec<Spot> = inside.iter().filter_map(|o| edge(o, at)).collect();
    closest.sort_by_key(|spot| at.distance_squared(*spot));
    let back = entered
        .into_iter()
        .flat_map(|entered| inside.iter().filter_map(move |o| edge(o, entered)));
    closest
        .into_iter()
        .chain(back)
        .find(|&to| clear(walker, at, to))
}

/// Whether stepping from `at` to `to` is a legal walk (room and footing, no
/// jump, not into an enemy) ending clear of everyone.
fn clear(walker: &Walker, at: Spot, to: Spot) -> bool {
    to != at
        && !crowded_at(walker.size, to, walker.others)
        && walker
            .passage(at, to)
            .is_some_and(|passage| passage.jumped.is_none())
        && crowd_per_mille(walker.size, at, to, walker.others).is_some()
}

/// `from` moved by `by` units, rounded.
fn offset(from: Spot, by: DVec2) -> Spot {
    from.offset(by.x.round() as i64, by.y.round() as i64)
}

#[cfg(test)]
mod tests;
