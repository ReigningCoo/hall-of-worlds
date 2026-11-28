//! A mob's choice, in place of the separate rules it had: about those it sees
//! each tick (`choose`), and about whoever has just harmed it (`after_harm`).

use shared::behaviour::{BehaviourDef, Behaviours};
use shared::dungeon::{Level, line_is_clear};
use shared::sheet::Feeling;

use super::super::harmed::Harmer;
use super::super::timid::is_timid;
use super::super::{Doing, FRIENDLY_GAP, Mob, SETTLING_STEPS, Spot, cells_apart};
use super::fire::near_fire;
use super::score::{Mind, pick, score};
use super::seen::{Other, Seen, Wants, friends_seen};
use super::{Choice, Scores};
use crate::combat::health_left;

/// The share of its Health a mob has lost, 0 to 100.
fn hurt(mob: &Mob, tick: u64) -> i32 {
    let left = i32::from(health_left(&mob.vitals, mob.limits(), tick));
    (1000 - left).clamp(0, 1000) / 10
}

/// What `mob` does now about the players and other mobs it sees (`spots`), if it should stop
/// what it is doing, and its scores. One going after someone, following them
/// or keeping away only changes its mind to flee. `others` are its level's mobs
/// as they stood at the start of the tick, `fires` the open fires there.
pub(in super::super) fn choose(
    mob: &Mob,
    level: &Level,
    spots: &[Spot],
    (others, fires): (&[Other], &[(i64, i64)]),
    behaviours: &Behaviours,
    tick: u64,
) -> (Option<Doing>, Scores) {
    let sees_within = mob.kind.senses().sight_range();
    // Only what is in its cone: sneaking up behind works.
    let cone = mob.cone();
    let seen: Vec<Seen> = (spots.iter())
        .filter(|spot| {
            spot.id != mob.id
                && cells_apart(spot.position, mob.position) <= sees_within
                && cone.sees(spot.at)
                && line_is_clear(level, mob.position, spot.position)
        })
        .map(|spot| Seen::of(mob, spot, near_fire(fires, spot.position)))
        .collect();
    if seen.is_empty() {
        return (None, Scores::default());
    }
    let def = behaviours.of(mob.kind);
    let friends = match seen.iter().any(|one| one.enemy) {
        true => friends_seen(mob, level, others, behaviours),
        false => 0,
    };
    let (options, fear) = score(&mind(mob, def, friends, tick), &seen);
    let chosen = pick(&options, Choice::of(&mob.doing));
    let scores = Scores {
        options,
        fear,
        chosen,
    };
    let busy = matches!(
        mob.doing,
        Doing::Going { .. } | Doing::Following { .. } | Doing::Fleeing { .. }
    );
    let doing = chosen.and_then(|choice| {
        if busy && !(matches!(choice, Choice::Flee(_)) && Choice::of(&mob.doing) != Some(choice)) {
            return None;
        }
        let at = seen.iter().find(|one| one.id == choice.who())?.position;
        Some(choice.doing(at))
    });
    (doing, scores)
}

fn mind(mob: &Mob, def: &BehaviourDef, friends: i32, tick: u64) -> Mind {
    Mind {
        drives: def.drives,
        hurt: hurt(mob, tick),
        friends,
        fears_fire: def.fears_fire,
    }
}

/// What a mob does once harmed by `by`, by how it now stands with them
/// (`feeling`), and its scores: a timid one keeps away; an unfriendly one goes
/// for them, unless its morale has it flee; one only cooled to neutral stops
/// coming over but doesn't fight back. `friends`: its band and allies in sight.
/// `None` to carry on.
pub(in super::super) fn after_harm(
    mob: &Mob,
    by: &Harmer,
    feeling: Feeling,
    (def, friends): (&BehaviourDef, i32),
    tick: u64,
) -> (Option<Doing>, Scores) {
    let current = Choice::of(&mob.doing);
    if is_timid(mob) {
        let flee = Choice::Flee(by.id);
        let scores = Scores {
            chosen: Some(flee),
            ..Scores::default()
        };
        let fleeing = matches!(mob.doing, Doing::Fleeing { from, .. } if from == by.id);
        return ((!fleeing).then(|| flee.doing(by.position)), scores);
    }
    let enemy = feeling == Feeling::Unfriendly;
    let seen = [Seen {
        id: by.id,
        position: by.position,
        wants: if enemy { Wants::Attack } else { Wants::Nothing },
        enemy,
        apart: 0,
        near_fire: false,
    }];
    let (options, fear) = score(&mind(mob, def, friends, tick), &seen);
    let chosen = pick(&options, current);
    let doing = match (chosen, mob.doing) {
        (Some(choice), _) if Some(choice) == current => None,
        (Some(choice), _) => Some(choice.doing(by.position)),
        (None, Doing::Going { after, gap, .. }) if after == by.id && gap == FRIENDLY_GAP => {
            Some(Doing::Settling {
                steps_left: SETTLING_STEPS,
            })
        }
        (None, _) => None,
    };
    let scores = Scores {
        options,
        fear,
        chosen,
    };
    (doing, scores)
}
