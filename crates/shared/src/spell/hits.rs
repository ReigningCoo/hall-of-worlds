//! What one where step hits, whichever where it is: the one call casting
//! goes through on both sides (magic plan PR 3).

use crate::dungeon::Level;

use super::beam::beam;
use super::design::WhereStep;
use super::hit::{Creature, Hit, Origin};
use super::missile::missile;
use super::place::Where;
use super::point::point;
use super::surround::surround;
use super::touch::touch;
use super::toward::Toward;

/// What `step` hits from `origin` at `toward`, among `creatures` (the caster's
/// side marked `Side::Caster`). The step's own filters limit it, so a first
/// step's limit only that step (`check` keeps filters to the first). Each hit
/// is the next step's origin (`Hit::origin`); a step that met nobody gives
/// the spots it reached instead (none for Point), which `hit_anyone` tells
/// apart. A missile's flight time is `missile`'s.
pub fn where_hits(
    level: &Level,
    step: &WhereStep,
    origin: &Origin,
    toward: Toward,
    creatures: &[Creature],
) -> Vec<Hit> {
    let (count, filters) = (step.count, step.filters);
    match step.place {
        Where::Touch => touch(level, origin, count, filters, creatures),
        Where::Missile => vec![missile(level, origin, toward, filters, creatures).hit],
        Where::Surround => surround(level, origin, count, filters, creatures),
        Where::Beam => beam(level, origin, toward, count, filters, creatures),
        Where::Point => point(level, origin, toward, count, filters, creatures),
    }
}
