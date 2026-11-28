use shared::behaviour::{Behaviours, Drives, Fences, Flee};
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::movement::Step;
use shared::vitals::Points;
use std::collections::BTreeSet;

use super::super::tests::{beside_a_mob, player, run, the_mob};
use super::super::{ATTACK_GAP, Doing, FRIENDLY_GAP};
use super::morale::fear;
use super::score::{MARGIN, Mind, attack_score, pick, score};
use super::seen::{Seen, Wants};
use super::zigzag::turn;
use super::{Choice, Scored, Ways, with_fire};

fn drives(aggression: u8, courage: u8) -> Drives {
    Drives {
        aggression,
        courage,
        ..Drives::default()
    }
}

fn mind(hurt: i32, friends: i32) -> Mind {
    Mind {
        drives: drives(60, 30),
        hurt,
        friends,
        fears_fire: true,
    }
}

fn seen(n: u128, wants: Wants, apart: i64) -> Seen {
    Seen {
        id: Id::for_test(n),
        position: (apart, 0),
        wants,
        enemy: wants == Wants::Attack,
        apart,
        near_fire: false,
    }
}

fn scored(choice: Choice, score: i32) -> Scored {
    Scored { choice, score }
}

#[test]
fn nothing_seen_or_wanted_is_no_choice() {
    assert_eq!(pick(&[], None), None);
    let (options, fear) = score(&mind(0, 0), &[seen(1, Wants::Nothing, 2)]);
    assert!(options.is_empty());
    assert_eq!(fear, 0);
}

#[test]
fn each_want_gives_its_option() {
    let all = [
        seen(1, Wants::Attack, 2),
        seen(2, Wants::Approach, 2),
        seen(3, Wants::Guard, 2),
        seen(4, Wants::KeepAway, 2),
    ];
    let (options, _) = score(&mind(0, 0), &all);
    let choices: Vec<Choice> = options.iter().map(|option| option.choice).collect();
    let id = Id::for_test;
    assert_eq!(
        choices,
        [
            Choice::Attack(id(1)),
            Choice::Approach(id(2)),
            Choice::Guard(id(3)),
            Choice::Flee(id(4)),
        ]
    );
    assert_eq!(options[0].score, attack_score(drives(60, 30)));
}

#[test]
fn the_best_wins_and_the_first_of_equals() {
    let (a, b) = (
        Choice::Attack(Id::for_test(1)),
        Choice::Attack(Id::for_test(2)),
    );
    assert_eq!(pick(&[scored(a, 50), scored(b, 70)], None), Some(b));
    assert_eq!(pick(&[scored(a, 70), scored(b, 70)], None), Some(a));
}

#[test]
fn what_it_is_doing_keeps_unless_beaten_by_the_margin() {
    let (now, other) = (
        Choice::Attack(Id::for_test(1)),
        Choice::Flee(Id::for_test(2)),
    );
    let close = [scored(now, 60), scored(other, 60 + MARGIN)];
    assert_eq!(pick(&close, Some(now)), Some(now));
    let beaten = [scored(now, 60), scored(other, 61 + MARGIN)];
    assert_eq!(pick(&beaten, Some(now)), Some(other));
    // Not among the options: no margin.
    assert_eq!(
        pick(&close, Some(Choice::Guard(Id::for_test(3)))),
        Some(other)
    );
}

#[test]
fn unhurt_and_alone_it_fears_nothing_but_numbers() {
    assert_eq!(fear(30, 0, 1, 0), 0);
    assert_eq!(fear(30, 0, 0, 0), 0);
    assert_eq!(fear(30, 0, 3, 0), 40);
    assert_eq!(fear(30, 0, 3, 2), 0);
}

#[test]
fn hurt_counts_more_the_less_courage() {
    assert_eq!(fear(60, 50, 1, 0), 50);
    assert_eq!(fear(20, 50, 1, 0), 100);
    assert_eq!(fear(100, 90, 1, 0), 0);
    // Friends in sight make it braver, three at most.
    assert_eq!(fear(60, 50, 1, 1), 40);
    assert_eq!(fear(60, 80, 1, 5), 50);
}

#[test]
fn badly_hurt_it_flees_its_nearest_enemy() {
    let all = [seen(1, Wants::Attack, 5), seen(2, Wants::Attack, 2)];
    let (options, fear) = score(&mind(90, 0), &all);
    assert!(fear > attack_score(drives(60, 30)));
    let flee = Choice::Flee(Id::for_test(2));
    assert_eq!(pick(&options, None), Some(flee));
    // Already going after one, it still turns to flee.
    assert_eq!(
        pick(&options, Some(Choice::Attack(Id::for_test(1)))),
        Some(flee)
    );
}

#[test]
fn no_fleeing_from_enemies_already_far_off() {
    let (options, fear) = score(&mind(90, 0), &[seen(1, Wants::Attack, 12)]);
    assert!(fear > 0);
    assert_eq!(options, [scored(Choice::Attack(Id::for_test(1)), 80)]);
}

#[test]
fn one_that_fears_fire_goes_up_to_nobody_by_one() {
    let by_fire = Seen {
        near_fire: true,
        ..seen(1, Wants::Attack, 2)
    };
    let (options, _) = score(&mind(0, 0), &[by_fire]);
    assert!(options.is_empty());
    let brave = Mind {
        fears_fire: false,
        ..mind(0, 0)
    };
    assert_eq!(score(&brave, &[by_fire]).0.len(), 1);
}

#[test]
fn choices_are_read_from_what_it_is_doing() {
    let id = Id::for_test(1);
    let going = |gap| Doing::Going {
        after: id,
        last_seen: (0, 0),
        gap,
    };
    assert_eq!(Choice::of(&going(ATTACK_GAP)), Some(Choice::Attack(id)));
    assert_eq!(Choice::of(&going(FRIENDLY_GAP)), Some(Choice::Approach(id)));
    assert_eq!(Choice::of(&Doing::Idle), None);
    for choice in [
        Choice::Attack(id),
        Choice::Approach(id),
        Choice::Guard(id),
        Choice::Flee(id),
    ] {
        assert_eq!(Choice::of(&choice.doing((3, 4), 0)), Some(choice));
    }
}

#[test]
fn fences_stop_by_race_seldom_unless_fleeing() {
    let ways = |fences| Ways {
        fences,
        ..Ways::default()
    };
    assert!(!ways(Fences::Through).stopped_by_fences(false));
    assert!(ways(Fences::Seldom).stopped_by_fences(false));
    assert!(!ways(Fences::Seldom).stopped_by_fences(true));
    assert!(ways(Fences::Never).stopped_by_fences(true));
    // The game's wolves and rabbits.
    let wolf = Ways::of(Behaviours::built_in_ref().of(CreatureKind::Wolf));
    assert_eq!((wolf.fences, wolf.fears_fire), (Fences::Seldom, true));
    let rabbit = Ways::of(Behaviours::built_in_ref().of(CreatureKind::Rabbit));
    assert_eq!(rabbit.flee, Flee::Zigzag);
}

#[test]
fn a_zigzag_turns_an_eighth_either_way() {
    let east = Step::new(1, 0).unwrap();
    let turned = |left| turn(east, left).map(|step| (step.dx(), step.dy()));
    assert_eq!(turned(true), Some((1, 1)));
    assert_eq!(turned(false), Some((1, -1)));
    let north_west = Step::new(-1, -1).unwrap();
    let turned = turn(north_west, true).map(|step| (step.dx(), step.dy()));
    assert_eq!(turned, Some((0, -1)));
}

#[test]
fn near_a_fire_it_may_only_step_away() {
    let taken = BTreeSet::new();
    assert_eq!(with_fire(&taken, (20, 0), &[(0, 0)]), None);
    let feared = with_fire(&taken, (5, 0), &[(0, 0)]).unwrap();
    assert!(feared.contains(&(3, 0)) && !feared.contains(&(4, 0)));
    let feared = with_fire(&taken, (2, 0), &[(0, 0)]).unwrap();
    assert!(feared.contains(&(1, 1)) && !feared.contains(&(2, 1)) && !feared.contains(&(3, 0)));
}

#[test]
fn a_badly_hurt_goblin_turns_and_flees() {
    let mut test = beside_a_mob(CreatureKind::Goblin, 3);
    let tick = test.world.action_clock.ticks();
    let population = test.world.populations.0.values_mut().next().unwrap();
    let mob = &mut population.mobs[0];
    let limits = mob.limits();
    let health = mob.health(tick).milli() / 1000;
    mob.vitals
        .hurt(Points::whole(health * 9 / 10), limits, tick, 0);
    run(&mut test, 3);
    let mob = the_mob(&test);
    let me = test.client.entity_id();
    assert!(matches!(mob.doing, Doing::Fleeing { from, .. } if from == me));
    assert_eq!(mob.scores.chosen, Some(Choice::Flee(me)));
    assert!(super::super::cells_apart(mob.position, player(&test)) >= 3);
}

#[test]
fn an_unhurt_goblin_still_attacks() {
    let mut test = beside_a_mob(CreatureKind::Goblin, 3);
    run(&mut test, 3);
    let me = test.client.entity_id();
    assert_eq!(the_mob(&test).scores.chosen, Some(Choice::Attack(me)));
}
