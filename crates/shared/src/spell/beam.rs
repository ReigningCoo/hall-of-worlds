//! Beam: a line to the target, hitting everything the step's filters allow
//! along it, stopped by walls. Each count after the first adds a beam either
//! side, `BEAM_SPREAD` further round.

use std::collections::BTreeMap;

use crate::dungeon::Level;
use crate::facing::{Heading, TURN};
use crate::spot::{CELL_SPAN, Spot};

use super::filters::Filters;
use super::hit::{Creature, Hit, Origin};
use super::line::{ahead, entry_along, wall_stop};
use super::toward::Toward;

/// The furthest a beam reaches, in units: 8 cells. First guess.
pub const BEAM_RANGE: u64 = 8 * CELL_SPAN as u64;

/// The angle between beams side by side: a sixteenth of a turn. First guess.
pub const BEAM_SPREAD: u16 = (TURN / 16) as u16;

/// The ways Beam ×`count` goes from `heading`: the middle one first, then
/// one either side for each count after the first (right, then left).
pub fn beam_headings(heading: Heading, count: u8) -> Vec<Heading> {
    let mut ways = vec![heading];
    for n in 1..i32::from(count.max(1)) {
        let turn = n * i32::from(BEAM_SPREAD);
        ways.push(heading.turned(turn));
        ways.push(heading.turned(-turn));
    }
    ways
}

/// Each of Beam ×`count`'s ways from `origin` at `toward` and where it ends:
/// as far as the spot aimed at (at most `BEAM_RANGE`; straight ahead that far
/// with no spot), every beam as long, stopped by walls.
pub(super) fn beam_ends(
    level: &Level,
    origin: &Origin,
    toward: Toward,
    count: u8,
    creatures: &[Creature],
) -> Vec<(Heading, Spot)> {
    let middle = toward.heading(origin, creatures);
    let length = toward
        .spot(creatures)
        .map_or(BEAM_RANGE, |spot| origin.at.distance(spot).min(BEAM_RANGE));
    beam_headings(middle, count)
        .into_iter()
        .map(|way| {
            (
                way,
                wall_stop(level, origin.at, ahead(origin.at, way, length)),
            )
        })
        .collect()
}

/// What Beam ×`count` from `origin` at `toward` hits, nearest first, each once:
/// as far as the spot aimed at (at most `BEAM_RANGE`; straight ahead that far
/// with no spot), every beam as long. With nobody there, each beam's end.
pub fn beam(
    level: &Level,
    origin: &Origin,
    toward: Toward,
    count: u8,
    filters: Filters,
    creatures: &[Creature],
) -> Vec<Hit> {
    let ends = beam_ends(level, origin, toward, count, creatures);
    // Each creature by the nearest point any beam meets it.
    let mut met: BTreeMap<usize, (u64, Heading)> = BTreeMap::new();
    for &(way, end) in &ends {
        for (index, other) in creatures.iter().enumerate() {
            if Some(index) == origin.creature || !other.side.allowed_by(filters) {
                continue;
            }
            let radius = other.body.size.radius().unsigned_abs();
            if let Some(along) = entry_along(origin.at, end, other.body.at, radius) {
                let nearer = met.get(&index).is_none_or(|&(was, _)| along < was);
                if nearer {
                    met.insert(index, (along, way));
                }
            }
        }
    }
    if met.is_empty() {
        return ends
            .into_iter()
            .map(|(heading, at)| Hit {
                creature: None,
                at,
                height: origin.height,
                heading,
            })
            .collect();
    }
    let mut found: Vec<(u64, usize, Heading)> = met
        .into_iter()
        .map(|(index, (along, way))| (along, index, way))
        .collect();
    found.sort_unstable();
    found
        .into_iter()
        .map(|(_, index, way)| Hit::on(creatures, index, origin.at, way))
        .collect()
}
