//! A mob's choice, in place of the separate rules it had: about those it sees
//! each tick (`choose`), and about whoever has just harmed it (`after_harm`).

use shared::behaviour::{BehaviourDef, Behaviours};
use shared::dungeon::Level;
use shared::sheet::Feeling;

use super::super::alert::in_view;
use super::super::desire::{Lying, take_option};
use super::super::harmed::Harmer;
use super::super::timid::is_timid;
use super::super::{Doing, FRIENDLY_GAP, Mob, SETTLING_STEPS, Spot};
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

/// What `mob` does now about the players and other mobs it sees (`spots`) and
/// the bags it wants that it sees (`lying`, `desire.rs`), if it should stop
/// what it is doing, and its scores. One going after someone, following them
/// or keeping away only changes its mind to flee. `others` are its level's mobs
/// as they stood at the start of the tick, `fires` the open fires there.
pub(in super::super) fn choose(
    mob: &Mob,
    level: &Level,
    spots: &[Spot],
    (others, fires, lying): (&[Other], &[(i64, i64)], &[Lying]),
    behaviours: &Behaviours,
    tick: u64,
) -> (Option<Doing>, Scores) {
    // Only what is in its cone: sneaking up behind works.
    let seen: Vec<Seen> = (spots.iter())
        .filter(|spot| in_view(mob, level, spot))
        .map(|spot| Seen::of(mob, spot, near_fire(fires, spot.position)))
        .collect();
    let def = behaviours.of(mob.kind);
    let take = take_option(mob, level, (lying, fires), def, tick);
    if seen.is_empty() && take.is_none() {
        return (None, Scores::default());
    }
    let friends = match seen.iter().any(|one| one.enemy) {
        true => friends_seen(mob, level, others, behaviours),
        false => 0,
    };
    let (mut options, fear) = score(&mind(mob, def, friends, tick), &seen);
    options.extend(take);
    let current = Choice::of(&mob.doing);
    let chosen = pick(&options, current);
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
        if busy && !(matches!(choice, Choice::Flee(_)) && current != Some(choice)) {
            return None;
        }
        let Some(who) = choice.who() else {
            // Going for a bag: carrying on if it already is.
            return (current != Some(choice)).then(|| choice.doing(mob.position, tick));
        };
        let at = seen.iter().find(|one| one.id == who)?.position;
        Some(choice.doing(at, tick))
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
        return ((!fleeing).then(|| flee.doing(by.position, tick)), scores);
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
        (Some(choice), _) => Some(choice.doing(by.position, tick)),
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
