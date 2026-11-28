//! Hunting: wolves after wild rabbits, eating kills, giving up past the leash.

use shared::creature::CreatureKind;
use shared::facing::Heading;
use shared::id::Id;
use shared::vitals::Points;

use super::super::scent::Went;
use super::super::tests::{beside_a_mob, player, run};
use super::super::{ATTACK_GAP, Doing, Harmer, Mob, harm_mob};
use super::{EATING_FOR, HUNT_LEASH};
use crate::connections::tests::TestClient;

/// A wolf (mob 0) two cells east of the player facing east, a rabbit (mob 1)
/// three cells further east, and with `second` another wolf of its pack (mob
/// 2) beside the player facing west, away from the rabbit. No other mobs
/// about, and the player unseen (god mode).
fn wolf_and_rabbit(second: bool) -> TestClient {
    let mut test = beside_a_mob(CreatureKind::Wolf, 2);
    test.client.god_mode = true;
    let (x, y) = player(&test);
    let population = test.world.populations.0.values_mut().next().unwrap();
    let wanted = if second { 3 } else { 2 };
    assert!(population.mobs.len() >= 2, "mobs to remake");
    if second && population.mobs.len() < 3 {
        let mut copy = population.mobs[0].clone();
        copy.id = Id::for_test(88);
        population.mobs.insert(2, copy);
    }
    population.mobs[0].heading = Heading::EAST;
    let pack = population.mobs[0].band;
    population.remake_for_test(1, CreatureKind::Rabbit, (x + 5, y));
    if second {
        let other = population.remake_for_test(2, CreatureKind::Wolf, (x + 1, y));
        other.band = pack;
        other.heading = Heading::WEST;
    }
    population.mobs.truncate(wanted);
    let kept: Vec<Id> = population.mobs.iter().map(|mob| mob.id).collect();
    population.due.retain(|(_, id)| kept.contains(id));
    test
}

fn mob(test: &TestClient, index: usize) -> Mob {
    test.world.populations.0.values().next().unwrap().mobs[index].clone()
}

fn after(mob: &Mob, them: Id) -> bool {
    match mob.doing {
        Doing::Going { after, gap, .. } | Doing::Tracking { after, gap, .. } => {
            after == them && gap == ATTACK_GAP
        }
        _ => false,
    }
}

#[test]
fn a_wolf_goes_after_a_rabbit_it_sees_and_the_rabbit_runs() {
    let mut test = wolf_and_rabbit(false);
    run(&mut test, 1);
    let (wolf, rabbit) = (mob(&test, 0), mob(&test, 1));
    assert!(after(&wolf, rabbit.id), "{:?}", wolf.doing);
    assert!(
        matches!(rabbit.doing, Doing::Fleeing { from, .. } if from == wolf.id),
        "{:?}",
        rabbit.doing
    );
}

#[test]
fn penned_rabbits_are_not_hunted() {
    let mut test = wolf_and_rabbit(false);
    let population = test.world.populations.0.values_mut().next().unwrap();
    population.mobs[1].penned = true;
    run(&mut test, 1);
    assert_eq!(mob(&test, 0).doing, Doing::Idle);
}

#[test]
fn the_pack_in_sight_joins_the_hunt() {
    let mut test = wolf_and_rabbit(true);
    run(&mut test, 1);
    let rabbit = mob(&test, 1).id;
    let second = mob(&test, 2);
    assert!(after(&second, rabbit), "{:?}", second.doing);
}

#[test]
fn a_wolf_picks_up_prey_by_its_scent() {
    let mut test = wolf_and_rabbit(false);
    let rabbit = mob(&test, 1).id;
    let tick = test.world.action_clock.ticks();
    let population = test.world.populations.0.values_mut().next().unwrap();
    // Facing away, it can't see the rabbit; but its trail is right here.
    let wolf = &mut population.mobs[0];
    wolf.heading = Heading::WEST;
    let at = wolf.position;
    population
        .scents
        .lay(rabbit, at, Went::Towards(1, 0), 100, tick);
    run(&mut test, 1);
    let wolf = mob(&test, 0);
    assert!(after(&wolf, rabbit), "{:?}", wolf.doing);
}

#[test]
fn a_kill_is_eaten_leaving_nothing_then_the_wolf_goes_home() {
    let mut test = wolf_and_rabbit(false);
    run(&mut test, 1);
    let place = *test.world.populations.0.keys().next().unwrap();
    let (wolf, rabbit) = (mob(&test, 0), mob(&test, 1));
    let by = Harmer {
        id: wolf.id,
        position: wolf.position,
        kind: CreatureKind::Wolf,
        member: Some(wolf.member()),
    };
    let tick = test.world.action_clock.ticks();
    let kills = test.world.kills.len();
    harm_mob(
        &mut test.world,
        (place, place.1),
        rabbit.id,
        Points::whole(1000),
        by,
        tick,
    );
    let population = test.world.populations.0.values().next().unwrap();
    assert!(population.mobs.iter().all(|one| one.id != rabbit.id));
    assert_eq!(test.world.kills.len(), kills, "eaten, not left as a kill");
    assert!(matches!(mob(&test, 0).doing, Doing::Eating { .. }));
    run(&mut test, EATING_FOR + 1);
    assert!(
        matches!(mob(&test, 0).doing, Doing::GoingHome { .. } | Doing::Idle),
        "{:?}",
        mob(&test, 0).doing
    );
}

#[test]
fn prey_past_the_leash_ends_the_hunt() {
    let mut test = wolf_and_rabbit(false);
    let rabbit = mob(&test, 1);
    let tick = test.world.action_clock.ticks();
    let population = test.world.populations.0.values_mut().next().unwrap();
    let wolf = &mut population.mobs[0];
    // Its den far behind it: the rabbit is too far from home to chase.
    wolf.home = (wolf.position.0 - HUNT_LEASH, wolf.position.1);
    wolf.doing = Doing::Going {
        after: rabbit.id,
        last_seen: rabbit.position,
        gap: ATTACK_GAP,
    };
    let id = wolf.id;
    population.due.insert((tick, id));
    run(&mut test, 1);
    assert!(
        matches!(mob(&test, 0).doing, Doing::GoingHome { .. }),
        "{:?}",
        mob(&test, 0).doing
    );
}

#[test]
fn a_wolf_left_to_it_eats_the_rabbit_or_gives_up_and_goes_home() {
    let mut test = wolf_and_rabbit(false);
    let rabbit = mob(&test, 1).id;
    run(&mut test, 400);
    let population = test.world.populations.0.values().next().unwrap();
    let gone = population.mobs.iter().all(|one| one.id != rabbit);
    let wolf = mob(&test, 0);
    assert!(
        gone || matches!(wolf.doing, Doing::GoingHome { .. } | Doing::Idle),
        "still at it: {:?}",
        wolf.doing
    );
}

#[test]
fn bolder_prey_keeps_away_from_hunters_too() {
    let mut test = wolf_and_rabbit(false);
    let (x, y) = mob(&test, 1).position;
    let population = test.world.populations.0.values_mut().next().unwrap();
    population.remake_for_test(1, CreatureKind::Sheep, (x, y));
    run(&mut test, 1);
    let (wolf, sheep) = (mob(&test, 0), mob(&test, 1));
    assert!(after(&wolf, sheep.id), "{:?}", wolf.doing);
    assert!(
        matches!(sheep.doing, Doing::Fleeing { from, .. } if from == wolf.id),
        "{:?}",
        sheep.doing
    );
}
