//! Point: exactly the target, at once, with nothing in between but a clear
//! line of sight. Each count after the first adds the nearest creature to the
//! first on the same side (`Side::same_as`).

use crate::combat::distance;
use crate::dungeon::{Level, spots_are_clear};
use crate::spot::CELL_SPAN;

use super::filters::Filters;
use super::hit::{Creature, Hit, Origin, Side};
use super::toward::Toward;

/// The furthest Point reaches, in units, between middles in 3D: 8 cells.
/// First guess.
pub const POINT_RANGE: u64 = 8 * CELL_SPAN as u64;

/// Whom Point ×`count` from `origin` hits: the creature aimed at if the
/// filters allow it, else the nearest one they do; then `count - 1` more, the
/// nearest to the first on its side. Everyone in range with a clear line
/// from the origin, never the origin's own creature.
///
/// With Self, the caster is hit as well ("you and"), and the count is chosen
/// from the caster's side: Self + Point ×2 is you and two friends, Friend +
/// Point ×2 two friends. Nobody to hit is no hit at all.
pub fn point(
    level: &Level,
    origin: &Origin,
    toward: Toward,
    count: u8,
    filters: Filters,
    creatures: &[Creature],
) -> Vec<Hit> {
    let from = origin.body(creatures);
    let mut hits: Vec<usize> = Vec::new();
    let mut picking = filters;
    if filters.caster {
        hits.extend(creatures.iter().position(|c| c.side == Side::Caster));
        picking.friend = true;
    }
    let reachable = |index: usize| {
        let other = &creatures[index];
        Some(index) != origin.creature
            && other.side != Side::Caster
            && other.side.allowed_by(picking)
            && distance(&from, &other.body) <= POINT_RANGE
            && spots_are_clear(level, origin.at, other.body.at)
    };
    let nearest_to = |at: crate::spot::Spot, side: Option<Side>, taken: &[usize]| {
        (0..creatures.len())
            .filter(|&index| !taken.contains(&index) && reachable(index))
            .filter(|&index| side.is_none_or(|side| side.same_as(creatures[index].side)))
            .min_by_key(|&index| (creatures[index].body.at.distance_squared(at), index))
    };
    let aimed = match toward {
        Toward::Creature(index)
            if reachable(index) || unfiltered_self(index, filters, creatures) =>
        {
            Some(index)
        }
        _ => None,
    };
    let Some(first) = aimed.or_else(|| nearest_to(origin.at, None, &hits)) else {
        return to_hits(&hits, origin, creatures);
    };
    hits.push(first);
    let (at, side) = (creatures[first].body.at, creatures[first].side);
    let wanted = usize::from(count.max(1)) + usize::from(filters.caster);
    while hits.len() < wanted {
        match nearest_to(at, Some(side), &hits) {
            Some(index) => hits.push(index),
            None => break,
        }
    }
    to_hits(&hits, origin, creatures)
}

/// Whether an unfiltered Point aimed at the caster may hit them: casting on
/// oneself.
fn unfiltered_self(index: usize, filters: Filters, creatures: &[Creature]) -> bool {
    !filters.any() && creatures[index].side == Side::Caster
}

fn to_hits(hits: &[usize], origin: &Origin, creatures: &[Creature]) -> Vec<Hit> {
    hits.iter()
        .map(|&index| Hit::on(creatures, index, origin.at, origin.heading))
        .collect()
}
