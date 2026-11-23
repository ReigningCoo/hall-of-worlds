//! What a first where with filters is aimed at (owner, 2026-11-23): filters
//! choose the target rather than only limiting it.
//!
//! - **Self**: "do me instead": aimed at the caster whatever is targeted
//!   (Self + Surround is a ring round you). Self limits nothing it hits.
//! - **Friend / Enemy**: "is my target one?": the target if it is on that
//!   side, otherwise the nearest creature on it in the where's reach with a
//!   clear line; none, and the spell fizzles. They still limit what the step
//!   hits (`Side::allowed_by`).

use crate::combat::{distance, reach_between};
use crate::dungeon::{Level, spots_are_clear};
use crate::facing::Heading;

use super::beam::BEAM_RANGE;
use super::design::WhereStep;
use super::hit::{Creature, Origin};
use super::missile::MISSILE_RANGE;
use super::place::Where;
use super::point::POINT_RANGE;
use super::surround::surround_reach;
use super::toward::Toward;

/// The first where's origin and aim by its filters, or `None` when Friend or
/// Enemy find nobody of theirs in reach (a fizzle). A step with no filters
/// is left as it is.
pub(super) fn aim_first(
    level: &Level,
    step: &WhereStep,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Option<(Origin, Toward)> {
    let filters = step.filters;
    if filters.caster {
        let me = origin.creature.map_or(Toward::Ahead, Toward::Creature);
        return Some((*origin, me));
    }
    if !(filters.friend || filters.enemy) {
        return Some((*origin, toward));
    }
    let theirs = |index: usize| creatures[index].side.allowed_by(filters);
    if let Toward::Creature(index) = toward
        && theirs(index)
    {
        return Some((*origin, toward));
    }
    // Surround is round its centre whatever is aimed at: it only limits.
    if step.place == Where::Surround {
        return Some((*origin, toward));
    }
    let from = origin.body(creatures);
    let nearest = (0..creatures.len())
        .filter(|&index| Some(index) != origin.creature && theirs(index))
        .filter(|&index| {
            let body = &creatures[index].body;
            distance(&from, body) <= reach(step, &from, body)
                && spots_are_clear(level, origin.at, body.at)
        })
        .min_by_key(|&index| (distance(&from, &creatures[index].body), index))?;
    let mut turned = *origin;
    let at = creatures[nearest].body.at;
    turned.heading = Heading::towards(origin.at, at).unwrap_or(origin.heading);
    Some((turned, Toward::Creature(nearest)))
}

/// How far a where reaches from `from` to `to`, for choosing a target.
fn reach(step: &WhereStep, from: &crate::combat::Body, to: &crate::combat::Body) -> u64 {
    match step.place {
        Where::Touch => reach_between(from.size, 0, to.size),
        Where::Missile => MISSILE_RANGE,
        Where::Beam => BEAM_RANGE,
        Where::Point => POINT_RANGE,
        Where::Surround => {
            let radius = |body: &crate::combat::Body| body.size.radius().unsigned_abs();
            surround_reach(radius(from), step.count) + radius(to)
        }
    }
}
