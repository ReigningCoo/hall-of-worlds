//! Hunting (mob behaviour plan PR 5): a hunter always goes after the wild
//! prey of its race's `hunts` it sees (`choice`) or smells, and its band in
//! sight joins the chase. Prey keeps away from those hunting it, zigzagging if
//! its race does. A kill is eaten: it leaves nothing, and the hunters near it
//! stand and eat a while before going home. Prey that gets further than
//! `HUNT_LEASH` from a hunter's home (its den) makes it give up and go home.
//! Penned animals, escortees and still mobs are never prey (`foes`).

use std::collections::BTreeSet;

use shared::behaviour::{Behaviours, Relation};
use shared::creature::{Acuity, CreatureKind};
use shared::dungeon::{Level, line_is_clear};
use shared::id::Id;

use super::harmed::Harmer;
use super::scent::Scents;
use super::{ATTACK_GAP, Doing, Mob, SETTLING_STEPS, cells_apart};

/// How far from its home a hunter chases prey before giving up, in cells.
pub(super) const HUNT_LEASH: i64 = 32;
/// How long hunters eat a kill, in ticks: 30 seconds.
pub(super) const EATING_FOR: u64 = 600;
/// How near the kill a hunter after it must be to share in eating it, in cells.
const FEAST_NEAR: i64 = 4;

/// Whether a mob of `kind` killed by `by` is eaten: `by` is a mob whose race
/// hunts it.
pub(super) fn eaten(behaviours: &Behaviours, by: &Harmer, kind: CreatureKind) -> bool {
    by.member
        .is_some_and(|hunter| behaviours.of(hunter.kind).hunts.contains(&kind))
}

/// `prey`, lying at `at`, was killed and is eaten: its killer, and the others
/// after it within `FEAST_NEAR`, eat; any others after it settle. Those it
/// wakes are made due in `due`.
pub(super) fn feast(
    mobs: &mut [Mob],
    (prey, at): (Id, (i64, i64)),
    killer: Id,
    tick: u64,
    due: &mut BTreeSet<(u64, Id)>,
) {
    for mob in mobs.iter_mut().filter(|mob| mob.id != prey) {
        let after_it = mob.target == Some(prey);
        if mob.id != killer && !after_it {
            continue;
        }
        let at_rest = mob.doing == Doing::Idle;
        mob.doing = match mob.id == killer || cells_apart(mob.position, at) <= FEAST_NEAR {
            true => Doing::Eating {
                until: tick + EATING_FOR,
            },
            false => Doing::Settling {
                steps_left: SETTLING_STEPS,
            },
        };
        mob.target = None;
        if at_rest {
            due.insert((mob.next_due(tick), mob.id));
        }
    }
}

/// A hunter's turn to join a hunt or pick one up by scent: at rest or settling,
/// within `HUNT_LEASH` of home, it goes after the prey one of its band in sight
/// is after, else follows a fresh mark of its prey on or beside its cell.
/// Those it sets going are made due in `due`.
pub(super) fn hunt(
    mobs: &mut [Mob],
    level: &Level,
    (scents, behaviours): (&Scents, &Behaviours),
    tick: u64,
    due: &mut BTreeSet<(u64, Id)>,
) {
    // Who is hunting what, as things stand before any joins.
    let hunting: Vec<_> = (mobs.iter())
        .filter_map(|mob| Some((mob.id, mob.member(), mob.position, hunted(mob)?)))
        .collect();
    for mob in mobs.iter_mut() {
        let free = matches!(
            mob.doing,
            Doing::Idle | Doing::Settling { .. } | Doing::Investigating { .. }
        );
        if !free || mob.foes.prey.is_empty() || cells_apart(mob.home, mob.position) > HUNT_LEASH {
            continue;
        }
        let me = mob.member();
        let sees_within = mob.kind.senses().sight_range();
        let joined = hunting.iter().find(|&&(id, member, at, (prey, _))| {
            id != mob.id
                && mob.foes.is_prey(prey)
                && behaviours.relation(me, member) == Relation::Kin
                && cells_apart(at, mob.position) <= sees_within
                && line_is_clear(level, mob.position, at)
        });
        let doing = match joined {
            Some(&(_, _, _, (after, last_seen))) => Some(Doing::Going {
                after,
                last_seen,
                gap: ATTACK_GAP,
            }),
            None => smelt(mob, scents, tick).map(|after| Doing::Tracking {
                after,
                gap: ATTACK_GAP,
                since: 0,
            }),
        };
        if let Some(doing) = doing {
            let at_rest = mob.doing == Doing::Idle;
            mob.doing = doing;
            mob.target = doing.target();
            if at_rest {
                due.insert((mob.next_due(tick), mob.id));
            }
        }
    }
}

/// The prey a mob is going after or tracking, and where it was last known.
fn hunted(mob: &Mob) -> Option<(Id, (i64, i64))> {
    match mob.doing {
        Doing::Going {
            after, last_seen, ..
        } if mob.foes.is_prey(after) => Some((after, last_seen)),
        Doing::Tracking { after, .. } if mob.foes.is_prey(after) => Some((after, mob.position)),
        _ => None,
    }
}

/// Prey whose fresh mark a mob smells on or beside its cell, if any.
fn smelt(mob: &Mob, scents: &Scents, now: u64) -> Option<Id> {
    let smell = mob.kind.senses().smell;
    if smell == Acuity::None {
        return None;
    }
    let (x, y) = mob.position;
    let cells = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (x + dx, y + dy)));
    cells
        .flat_map(|cell| mob.foes.prey.iter().map(move |&prey| (cell, prey)))
        .find(|&(cell, prey)| scents.smelt(cell, prey, smell, now).is_some())
        .map(|(_, prey)| prey)
}

#[cfg(test)]
mod tests;
