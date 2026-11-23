//! Surround: a ring round its centre, never the centre itself, a cell wider
//! for each count.

use crate::combat::distance;
use crate::dungeon::{Level, spots_are_clear};
use crate::facing::TURN;
use crate::spot::CELL_SPAN;

use super::filters::Filters;
use super::hit::{Creature, Hit, Origin};
use super::line::{ahead, wall_stop};

/// How far the ring reaches past the centre's own circle for each count, in
/// units: a cell. First guess.
pub const SURROUND_RING: u64 = CELL_SPAN as u64;

/// Spots the ring gives when it hits nobody, evenly round it, facing out: so
/// Surround → Missile sends this many missiles outward.
pub const SURROUND_SPOTS: u32 = 8;

/// How far from the centre's middle Surround ×`count` reaches, for a centre
/// of `radius` (`Size::radius`).
pub fn surround_reach(radius: u64, count: u8) -> u64 {
    radius + SURROUND_RING * u64::from(count.max(1))
}

/// What Surround ×`count` round `origin` hits, nearest first: anyone whose
/// circle reaches into the ring, with a clear line from the centre. With
/// nobody there, `SURROUND_SPOTS` spots on the ring, facing out.
pub fn surround(
    level: &Level,
    origin: &Origin,
    count: u8,
    filters: Filters,
    creatures: &[Creature],
) -> Vec<Hit> {
    let centre = origin.body(creatures);
    let reach = surround_reach(centre.size.radius().unsigned_abs(), count);
    let mut found: Vec<(u64, usize)> = creatures
        .iter()
        .enumerate()
        .filter(|(index, other)| {
            Some(*index) != origin.creature
                && other.side.allowed_by(filters)
                && distance(&centre, &other.body) <= reach + other.body.size.radius().unsigned_abs()
                && spots_are_clear(level, origin.at, other.body.at)
        })
        .map(|(index, other)| (distance(&centre, &other.body), index))
        .collect();
    found.sort_unstable();
    if !found.is_empty() {
        return found
            .into_iter()
            .map(|(_, index)| Hit::on(creatures, index, origin.at, origin.heading))
            .collect();
    }
    (0..SURROUND_SPOTS)
        .map(|n| {
            let heading = origin.heading.turned((TURN / SURROUND_SPOTS * n) as i32);
            Hit {
                creature: None,
                at: wall_stop(level, origin.at, ahead(origin.at, heading, reach)),
                height: origin.height,
                heading,
            }
        })
        .collect()
}
