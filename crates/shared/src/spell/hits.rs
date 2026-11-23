//! What one where step hits, whichever where it is: the one call casting
//! goes through on both sides (magic plan PR 3), and the wheres merged in one
//! step, each leading on from the one before (PR 10).

use crate::dungeon::Level;

use super::aim::aim_first;
use super::beam::beam;
use super::design::{WherePart, WhereStep};
use super::filters::Filters;
use super::hit::{Creature, Hit, Origin};
use super::missile::missile;
use super::place::Where;
use super::point::point;
use super::surround::surround;
use super::touch::touch;
use super::toward::Toward;

/// What `step`'s first where hits from `origin` at `toward`, among
/// `creatures` (the caster's side marked `Side::Caster`). The step's own
/// filters choose its aim (`aim_first`: Self the caster, Friend or Enemy the
/// target if theirs, else the nearest of theirs in reach) and Friend and
/// Enemy limit what it hits, only that step's (`check` keeps filters to the
/// first). Each hit is the next where's origin (`Hit::origin`); a where that
/// met nobody gives the spots it reached instead (none for Point), which
/// `hit_anyone` tells apart. A missile's flight time is `missile`'s. The
/// step's merged wheres after the first are `part_hits`'.
pub fn where_hits(
    level: &Level,
    step: &WhereStep,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Vec<Hit> {
    // Filters choose the first where's aim (`aim_first`); nobody of theirs
    // in reach is no hit at all.
    let Some((origin, toward)) = aim_first(level, step, origin, toward, creatures) else {
        return Vec::new();
    };
    let first = WherePart {
        place: step.place,
        count: step.count,
    };
    hits_of(level, first, step.filters, &origin, toward, creatures)
}

/// What `step`'s where at `index` (1 or more: a merged one) hits from
/// `origin`, one of the hits of the where before it, going on ahead. Its
/// step's Friend and Enemy still limit what it hits, but nothing aims it:
/// only the first where of the first step uses the caster's target. Empty
/// when there is no where at `index`.
pub fn part_hits(
    level: &Level,
    step: &WhereStep,
    index: usize,
    origin: &Origin,
    creatures: &[Creature],
) -> Vec<Hit> {
    let Some(part) = step.parts().nth(index).filter(|_| index > 0) else {
        return Vec::new();
    };
    let filters = later_filters(step.filters);
    hits_of(level, part, filters, origin, Toward::Ahead, creatures)
}

/// Everything the whole of `step` hits at once (missiles landing as they
/// are launched): its first where's hits, then each merged where's from
/// every hit of the one before, all together, the first where's first.
pub fn step_hits(
    level: &Level,
    step: &WhereStep,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Vec<Hit> {
    let mut last = where_hits(level, step, origin, toward, creatures);
    let mut all = last.clone();
    for index in 1..step.len() {
        last = (last.iter())
            .flat_map(|hit| part_hits(level, step, index, &hit.origin(), creatures))
            .collect();
        all.extend(last.iter().copied());
    }
    all
}

/// The filters a merged where keeps: Friend and Enemy limit what it hits;
/// Self only ever aims the first where.
pub(super) fn later_filters(filters: Filters) -> Filters {
    Filters {
        caster: false,
        ..filters
    }
}

/// What one where hits from `origin` at `toward`, by these filters.
fn hits_of(
    level: &Level,
    part: WherePart,
    filters: Filters,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Vec<Hit> {
    let count = part.count;
    match part.place {
        Where::Touch => touch(level, origin, count, filters, creatures),
        Where::Missile => vec![missile(level, origin, toward, filters, creatures).hit],
        Where::Surround => surround(level, origin, count, filters, creatures),
        Where::Beam => beam(level, origin, toward, count, filters, creatures),
        Where::Point => point(level, origin, toward, count, filters, creatures),
    }
}
