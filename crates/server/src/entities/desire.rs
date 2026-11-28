//! Basic desire (mob behaviour plan PR 9): a mob goes for things it wants
//! lying on the ground (`BehaviourDef::desires`: food, coins, weapons) and
//! takes them into what it carries, one thing a time. Going for a bag is one
//! more choice for its scorer (`choice::Choice::Take`), scored by its desire
//! drive less the cells to walk, so seeing an enemy or a friend usually wins.

use std::collections::BTreeMap;

use shared::behaviour::{BehaviourDef, Behaviours, Desire};
use shared::dungeon::Level;
use shared::ground::in_reach;
use shared::item::Item;
use shared::spot::Spot;

use super::choice::{Choice, Scored};
use super::curious::{FURTHEST_FROM_HOME, WALK_WITHIN, sees_cell};
use super::doubling_back::walk_towards;
use super::{Doing, Maps, Mob, THINKING_COST, Taken, cells_apart};
use crate::ServerWorld;
use crate::storage::SiteId;

/// How long a bag it couldn't get to is passed over: 60 seconds.
pub(super) const PASS_OVER: u64 = 1200;
/// How far from any mob of its level a bag is worth telling them of, in cells.
const NOTICED_WITHIN: i64 = 48;

/// A bag on the ground and which desires it holds something for (bits by
/// `Desire::ALL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Lying {
    pub at: Spot,
    pub holds: u8,
}

/// The bags near mobs on each level in use.
pub(super) type LyingBy = BTreeMap<(SiteId, u32), Vec<Lying>>;

/// The bits of these desires.
pub(super) fn bits(desires: &[Desire]) -> u8 {
    (Desire::ALL.iter().enumerate())
        .filter(|(_, desire)| desires.contains(desire))
        .fold(0, |bits, (bit, _)| bits | 1 << bit)
}

/// What a bag holds for which desires.
pub(super) fn holds(items: &[Item]) -> u8 {
    let mut found = 0;
    for (bit, desire) in Desire::ALL.iter().enumerate() {
        if items.iter().any(|item| desire.wants(item.kind)) {
            found |= 1 << bit;
        }
    }
    found
}

/// The ground level a place's mobs stand on: the wilderness's areas all share
/// its one level.
pub(super) fn ground_index((site, index): (SiteId, u32)) -> u32 {
    match site {
        SiteId::Wilderness => 0,
        _ => index,
    }
}

/// The bags holding anything wanted near the mobs of every level in use.
pub(super) fn lying_by_place(world: &mut ServerWorld) -> LyingBy {
    let mut lying = LyingBy::new();
    let places: Vec<(SiteId, u32)> = world.populations.0.keys().copied().collect();
    for place in places {
        let mobs: Vec<(i64, i64)> = (world.populations.0[&place].mobs.iter())
            .map(|mob| mob.position)
            .collect();
        let Some(level) = world.site(place.0).levels.get(&ground_index(place)) else {
            continue;
        };
        let near = (level.ground.iter())
            .filter(|(at, _)| {
                (mobs.iter()).any(|&mob| cells_apart(mob, at.cell()) <= NOTICED_WITHIN)
            })
            .map(|(&at, items)| Lying {
                at,
                holds: holds(items),
            })
            .filter(|bag| bag.holds != 0);
        let near: Vec<Lying> = near.collect();
        if !near.is_empty() {
            lying.insert(place, near);
        }
    }
    lying
}

/// Going for a bag scores 20 to 70 by its desire drive, one less for every
/// cell to walk.
pub(super) fn take_score(desire: u8, apart: i64) -> i32 {
    20 + i32::from(desire.min(100)) / 2 - apart as i32
}

/// Whether it is free to go for something: at rest, wandering, on its way
/// home, looking into a noise, or going for something already.
fn free_to_take(mob: &Mob) -> bool {
    !mob.penned
        && mob.escort.is_none()
        && matches!(
            mob.doing,
            Doing::Idle
                | Doing::Settling { .. }
                | Doing::GoingHome { .. }
                | Doing::Investigating { .. }
                | Doing::Taking { .. }
        )
}

/// Going for the nearest bag it can see holding something it wants, not too
/// far from home, nor by an open fire if it fears fire, nor one it lately
/// couldn't get to; `None` if there is none or it isn't free to.
pub(super) fn take_option(
    mob: &Mob,
    level: &Level,
    (lying, fires): (&[Lying], &[(i64, i64)]),
    def: &BehaviourDef,
    tick: u64,
) -> Option<Scored> {
    let wants = bits(&def.desires);
    if wants == 0 || !free_to_take(mob) {
        return None;
    }
    let passed =
        |at: Spot| matches!(mob.passed_over, Some((bag, until)) if bag == at && tick < until);
    let fire = |at: Spot| def.fears_fire && super::choice::near_fire(fires, at.cell());
    let bag = (lying.iter())
        .filter(|bag| bag.holds & wants != 0)
        .filter(|bag| cells_apart(mob.home, bag.at.cell()) <= FURTHEST_FROM_HOME)
        .filter(|bag| !passed(bag.at) && !fire(bag.at))
        .filter(|bag| sees_cell(mob, level, bag.at.cell()))
        .min_by_key(|bag| (mob.spot.distance_squared(bag.at), bag.at))?;
    let score = take_score(def.drives.desire, cells_apart(mob.position, bag.at.cell()));
    (score > 0).then_some(Scored {
        choice: Choice::Take(bag.at),
        score,
    })
}

/// A turn of a mob `Taking` (`moves::act`): walking to the bag until in
/// reach of it (`reached` then takes), giving up when it can't get there or
/// takes too long. What it cost.
pub(super) fn go_take(
    mob: &mut Mob,
    level: &Level,
    maps: &mut Maps,
    tick: u64,
    taken: &Taken,
) -> u32 {
    let Doing::Taking { at, since } = mob.doing else {
        return THINKING_COST;
    };
    if in_reach(mob.size(), mob.spot, at) {
        return THINKING_COST;
    }
    if tick.saturating_sub(since) > WALK_WITHIN {
        give_up(mob, at, tick);
        return THINKING_COST;
    }
    walk_towards(mob, level, maps, at.cell(), taken).unwrap_or_else(|| {
        give_up(mob, at, tick);
        THINKING_COST
    })
}

fn give_up(mob: &mut Mob, at: Spot, tick: u64) {
    mob.passed_over = Some((at, tick + PASS_OVER));
    mob.doing = Doing::GoingHome { since: tick };
}

/// A mob that has come within reach of a bag it was going for, to take from
/// once every level has had its turn.
#[derive(Clone, Copy, Debug)]
pub(super) struct Reached {
    pub place: (SiteId, u32),
    pub mob: shared::id::Id,
    pub at: Spot,
}

/// After its turn: a mob `Taking` within reach of its bag will take from it,
/// and then heads home (where, still wanting, it may come back for more).
pub(super) fn reached(mob: &mut Mob, place: (SiteId, u32), tick: u64, all: &mut Vec<Reached>) {
    if let Doing::Taking { at, .. } = mob.doing
        && in_reach(mob.size(), mob.spot, at)
    {
        all.push(Reached {
            place,
            mob: mob.id,
            at,
        });
        mob.doing = Doing::GoingHome { since: tick };
    }
}

/// Each mob that `reached` its bag takes the first thing in it it wants,
/// whole (a stack too), into what it carries: kept with it, and dropped
/// where it dies. A bag left empty goes.
pub(super) fn take_up(world: &mut ServerWorld, reached: Vec<Reached>, behaviours: &Behaviours) {
    for Reached { place, mob, at } in reached {
        let Some(found) = (world.populations.0.get(&place))
            .and_then(|population| population.mobs.iter().find(|one| one.id == mob))
        else {
            continue;
        };
        let desires = behaviours.of(found.kind).desires.clone();
        let keep = (found.kept_with(place.0), found.home_level, found.home);
        let memory = found.memory.clone();
        let level = world
            .site_mut(place.0)
            .levels
            .entry(ground_index(place))
            .or_default();
        let Some(pile) = level.ground.get_mut(&at) else {
            continue;
        };
        let Some(index) = pile
            .iter()
            .position(|item| Desire::any_wants(&desires, item.kind))
        else {
            continue;
        };
        let item = pile.remove(index);
        if pile.is_empty() {
            level.ground.remove(&at);
        }
        let (site, home_level, home) = keep;
        let record = super::gear::record_of(world, site, (mob, home_level, home));
        record.memory = memory;
        record.carried.push(item);
    }
}

#[cfg(test)]
mod tests;
