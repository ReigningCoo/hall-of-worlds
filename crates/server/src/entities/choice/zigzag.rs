//! Fleeing zigzag (`Flee::Zigzag`, prey animals): each step away turned an
//! eighth of a turn to one side, then the other, where that still gets further
//! from the threat; else straight away.
use shared::behaviour::Flee;
use shared::dungeon::Level;
use shared::movement::{Step, mob_can_stand};
use shared::pathfinding::DistanceMap;

use super::super::{Mob, cells_apart};

/// The step a fleeing `mob` takes instead of `away` (straight from the threat,
/// whose distance map is `map`): turned to this turn's side if it zigzags and
/// the cell there is `free`, standable and no nearer.
pub(in super::super) fn zigzag(
    mob: &mut Mob,
    level: &Level,
    map: &DistanceMap,
    (away, threat): (Option<Step>, (i64, i64)),
    free: impl Fn((i64, i64)) -> bool,
) -> Option<Step> {
    let away = away?;
    if mob.ways.flee != Flee::Zigzag {
        return Some(away);
    }
    mob.ways.zig = !mob.ways.zig;
    let turned = turn(away, mob.ways.zig)?;
    let to = turned.from(mob.position);
    let here = map.cost(mob.position).unwrap_or(u32::MAX);
    let there = map.cost(to).unwrap_or(u32::MAX);
    // As many cells off as going straight, not only a longer way round.
    let as_far = cells_apart(to, threat) >= cells_apart(away.from(mob.position), threat);
    let fine = free(to) && mob_can_stand(level, to.0, to.1) && there > here && as_far;
    Some(if fine { turned } else { away })
}

/// `step` turned an eighth of a turn, one way or the other.
pub(in super::super) fn turn(step: Step, left: bool) -> Option<Step> {
    const ROUND: [(i64, i64); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let at = ROUND
        .iter()
        .position(|&(dx, dy)| (dx, dy) == (step.dx(), step.dy()))?;
    let next = (if left { at + 1 } else { at + 7 }) % 8;
    let (dx, dy) = ROUND[next];
    Step::new(dx, dy)
}
