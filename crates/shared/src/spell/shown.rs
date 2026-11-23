//! What a where step looked like as it happened (magic plan PR 7), for every
//! client on the level to draw: where it started and faced, how far it
//! reached, each beam's end and the spots it hit, coloured by its `Look`.
//! Missiles are told of apart, as they fly (`protocol::MissileNews`).

use serde::{Deserialize, Serialize};

use crate::combat::arm_reach;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::spot::Spot;

use super::aim::aim_first;
use super::beam::beam_ends;
use super::design::{WherePart, WhereStep};
use super::hit::{Creature, Hit, Origin};
use super::look::Look;
use super::place::Where;
use super::surround::surround_reach;
use super::toward::Toward;

/// One where as drawn.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shown {
    pub place: Where,
    pub count: u8,
    /// Where it started (after its filters' aim), and the ground's height
    /// there in units.
    pub from: Spot,
    pub height: i32,
    /// The way it faced: Touch's arc's middle, Beam's middle beam.
    pub heading: Heading,
    /// How far it reached from `from`, in units: Touch's arm, Surround's ring.
    /// 0 for the rest.
    pub reach: u32,
    /// Beam's ends, one a beam; empty for the rest.
    pub ends: Vec<Spot>,
    /// The spots of what it hit (or reached, hitting nobody).
    pub hits: Vec<Spot>,
    pub look: Look,
}

/// How `step`'s first where from `origin` at `toward` looks, having hit
/// `hits`. `None` when its filters found nobody to aim at (a fizzle: nothing
/// to draw).
pub fn shown(
    level: &Level,
    (step, look): (&WhereStep, Look),
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
    hits: &[Hit],
) -> Option<Shown> {
    let (origin, toward) = aim_first(level, step, origin, toward, creatures)?;
    let first = WherePart {
        place: step.place,
        count: step.count,
    };
    Some(drawn(
        level,
        (first, look),
        &origin,
        toward,
        creatures,
        hits,
    ))
}

/// How `step`'s merged where at `index` (1 or more) looks from `origin`, a
/// hit of the where before it, going on ahead, having hit `hits`. `None`
/// when there is no where at `index`.
pub fn shown_part(
    level: &Level,
    (step, index, look): (&WhereStep, usize, Look),
    origin: &Origin,
    creatures: &[Creature],
    hits: &[Hit],
) -> Option<Shown> {
    let part = step.parts().nth(index).filter(|_| index > 0)?;
    Some(drawn(
        level,
        (part, look),
        origin,
        Toward::Ahead,
        creatures,
        hits,
    ))
}

/// How one where looks from `origin` at `toward`, aimed already.
fn drawn(
    level: &Level,
    (part, look): (WherePart, Look),
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
    hits: &[Hit],
) -> Shown {
    let body = origin.body(creatures);
    let radius = body.size.radius().unsigned_abs();
    let reach = match part.place {
        Where::Touch => radius + arm_reach(body.size),
        Where::Surround => surround_reach(radius, part.count),
        Where::Missile | Where::Beam | Where::Point => 0,
    };
    let ends: Vec<(Heading, Spot)> = match part.place {
        Where::Beam => beam_ends(level, origin, toward, part.count, creatures),
        _ => Vec::new(),
    };
    let heading = ends.first().map_or(origin.heading, |(way, _)| *way);
    Shown {
        place: part.place,
        count: part.count,
        from: origin.at,
        height: origin.height,
        heading,
        reach: u32::try_from(reach).unwrap_or(u32::MAX),
        ends: ends.into_iter().map(|(_, end)| end).collect(),
        hits: hits.iter().map(|hit| hit.at).collect(),
        look,
    }
}
