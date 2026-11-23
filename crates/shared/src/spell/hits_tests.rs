//! Where hits on the test room: floor from x −7 to 7 and y −3 to 3 (cells,
//! 240 units each, middles at 240 × the cell), with a wall at x = −2 from
//! y = −1 to 1. The caster stands at the middle, facing east.

use super::*;
use crate::combat::Body;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::spot::Spot;

const ENEMY: Filters = Filters {
    enemy: true,
    ..Filters::NONE
};
const FRIEND: Filters = Filters {
    friend: true,
    ..Filters::NONE
};
const CASTER: Filters = Filters {
    caster: true,
    ..Filters::NONE
};

fn at(x: i64, y: i64, side: Side) -> Creature {
    Creature {
        body: Body {
            at: Spot::new(x, y),
            height: 0,
            size: Size::Medium,
        },
        side,
    }
}

/// The caster first, then the others.
fn with_caster(others: &[Creature]) -> Vec<Creature> {
    let mut all = vec![at(0, 0, Side::Caster)];
    all.extend_from_slice(others);
    all
}

fn from_caster(creatures: &[Creature]) -> Origin {
    Origin::from_creature(creatures, 0, Heading::EAST)
}

fn hit_ones(hits: &[Hit]) -> Vec<usize> {
    hits.iter().filter_map(|hit| hit.creature).collect()
}

fn step(place: Where, count: u8, filters: Filters) -> WhereStep {
    WhereStep {
        place,
        count,
        filters,
    }
}

#[test]
fn touch_reaches_ahead_in_a_widening_arc() {
    let level = Level::test_room();
    let all = with_caster(&[at(240, 0, Side::Enemy), at(-240, 240, Side::Enemy)]);
    let origin = from_caster(&all);
    assert_eq!(
        hit_ones(&touch(&level, &origin, 1, Filters::NONE, &all)),
        [1]
    );
    // Wide enough at ×7 to reach all round, never the caster.
    assert_eq!(touch_half_arc(7), Heading::WEST.0);
    assert_eq!(
        hit_ones(&touch(&level, &origin, 7, Filters::NONE, &all)),
        [1, 2]
    );
    // Nobody in reach: the spot touched, a Medium's radius and arm ahead.
    let alone = with_caster(&[]);
    let hits = touch(&level, &from_caster(&alone), 1, Filters::NONE, &alone);
    assert_eq!((hits[0].creature, hits[0].at), (None, Spot::new(260, 0)));
}

#[test]
fn a_missile_hits_the_first_it_may_and_stops_at_walls() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(480, 0, Side::Friend),
        at(960, 0, Side::Enemy),
        at(-720, 0, Side::Enemy),
    ]);
    let origin = from_caster(&all);
    let flight = missile(&level, &origin, Toward::Creature(2), Filters::NONE, &all);
    assert_eq!((flight.hit.creature, flight.length), (Some(1), 400));
    assert_eq!(flight_millis(flight.length, 1), 105);
    assert!(flight_millis(flight.length, 3) < flight_millis(flight.length, 1));
    // Enemy filter: through the friend to the enemy.
    let flight = missile(&level, &origin, Toward::Creature(2), ENEMY, &all);
    assert_eq!(flight.hit.creature, Some(2));
    assert_eq!(flight.hit.heading, Heading::EAST);
    // West, the wall at x = −2 stops it before the enemy behind.
    let flight = missile(&level, &origin, Toward::Creature(3), Filters::NONE, &all);
    assert_eq!(flight.hit.creature, None);
    assert!(
        (-480..-240).contains(&flight.hit.at.x),
        "{:?}",
        flight.hit.at
    );
    // A spot with nobody on the way: it lands there.
    let flight = missile(
        &level,
        &origin,
        Toward::Spot(Spot::new(0, 480)),
        ENEMY,
        &all,
    );
    assert_eq!(
        (flight.hit.creature, flight.hit.at),
        (None, Spot::new(0, 480))
    );
    assert_eq!(flight.hit.heading, Heading::SOUTH);
}

#[test]
fn surround_rings_its_centre_but_never_hits_it() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(240, 0, Side::Enemy),
        at(0, 600, Side::Friend),
        at(-720, 0, Side::Enemy),
    ]);
    let origin = from_caster(&all);
    assert_eq!(
        hit_ones(&surround(&level, &origin, 1, Filters::NONE, &all)),
        [1]
    );
    assert_eq!(
        hit_ones(&surround(&level, &origin, 2, Filters::NONE, &all)),
        [1, 2]
    );
    assert_eq!(hit_ones(&surround(&level, &origin, 2, FRIEND, &all)), [2]);
    // Behind the wall, though in reach at ×3.
    assert_eq!(hit_ones(&surround(&level, &origin, 3, ENEMY, &all)), [1]);
    // Hit from the centre, facing on away from it.
    let hits = surround(&level, &origin, 2, FRIEND, &all);
    assert_eq!(hits[0].origin().heading, Heading::SOUTH);
    // Nobody: eight spots round the ring, facing out.
    let alone = with_caster(&[]);
    let hits = surround(&level, &from_caster(&alone), 1, Filters::NONE, &alone);
    assert_eq!(hits.len(), SURROUND_SPOTS as usize);
    assert!(!hit_anyone(&hits));
    assert_eq!(
        (hits[0].at, hits[2].heading),
        (Spot::new(320, 0), Heading::SOUTH)
    );
}

#[test]
fn beams_hit_everything_along_them_and_spread_when_stacked() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(960, 0, Side::Enemy),
        at(480, 0, Side::Friend),
        at(440, 180, Side::Enemy),
        at(-720, 0, Side::Enemy),
    ]);
    let origin = from_caster(&all);
    let target = Toward::Creature(1);
    assert_eq!(
        hit_ones(&beam(&level, &origin, target, 1, Filters::NONE, &all)),
        [2, 1]
    );
    assert_eq!(
        hit_ones(&beam(&level, &origin, target, 1, ENEMY, &all)),
        [1]
    );
    assert_eq!(beam_headings(Heading::EAST, 2).len(), 3);
    assert_eq!(
        hit_ones(&beam(&level, &origin, target, 2, ENEMY, &all)),
        [3, 1]
    );
    // The wall stops a beam west, and nobody hit gives where each ended.
    let hits = beam(&level, &origin, Toward::Creature(4), 2, Filters::NONE, &all);
    assert!(!hit_anyone(&hits));
    assert_eq!(hits.len(), 3);
    assert!(hits.iter().all(|hit| hit.at.x > -480));
}

#[test]
fn point_hits_exactly_its_target_then_the_nearest_on_its_side() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(960, 0, Side::Enemy),
        at(480, 0, Side::Friend),
        at(960, 480, Side::Enemy),
        at(240, 240, Side::Friend),
        at(-720, 0, Side::Enemy),
        at(240, -240, Side::Neutral),
    ]);
    let origin = from_caster(&all);
    let target = Toward::Creature(1);
    // Nothing in between counts: the friend on the way is passed over.
    assert_eq!(
        hit_ones(&point(&level, &origin, target, 1, Filters::NONE, &all)),
        [1]
    );
    // ×2: the nearest enemy to the first, not the nearer friend.
    assert_eq!(
        hit_ones(&point(&level, &origin, target, 2, Filters::NONE, &all)),
        [1, 3]
    );
    // Aimed at a friend with Enemy: the nearest enemy instead.
    let at_friend = Toward::Creature(2);
    assert_eq!(
        hit_ones(&point(&level, &origin, at_friend, 1, ENEMY, &all)),
        [1]
    );
    // Self + Point ×2: you and two friends; Friend + Point ×2: two friends.
    assert_eq!(
        hit_ones(&point(&level, &origin, at_friend, 2, CASTER, &all)),
        [0, 2, 4]
    );
    assert_eq!(
        hit_ones(&point(&level, &origin, at_friend, 2, FRIEND, &all)),
        [2, 4]
    );
    // Unfiltered at oneself, it is a spell on the caster.
    let me = Toward::Creature(0);
    assert_eq!(
        hit_ones(&point(&level, &origin, me, 1, Filters::NONE, &all)),
        [0]
    );
    // Never through the wall: aimed behind it, the nearest in sight instead.
    let behind = Toward::Creature(5);
    assert_eq!(
        hit_ones(&point(&level, &origin, behind, 1, ENEMY, &all)),
        [1]
    );
    // The neutral only unfiltered.
    let neutral = Toward::Creature(6);
    assert_eq!(
        hit_ones(&point(&level, &origin, neutral, 1, Filters::NONE, &all)),
        [6]
    );
    assert_eq!(
        hit_ones(&point(&level, &origin, neutral, 1, ENEMY, &all)),
        [1]
    );
    // Enemy + Point with no enemy in sight: no hit at all (it fizzles).
    let friends = with_caster(&[at(480, 0, Side::Friend)]);
    let hits = point(
        &level,
        &from_caster(&friends),
        Toward::Ahead,
        2,
        ENEMY,
        &friends,
    );
    assert!(hits.is_empty());
}

#[test]
fn where_hits_goes_by_the_steps_where_and_hits_lead_on() {
    let level = Level::test_room();
    let all = with_caster(&[at(480, 0, Side::Enemy), at(960, 0, Side::Enemy)]);
    let origin = from_caster(&all);
    let missile = step(Where::Missile, 1, ENEMY);
    let first = where_hits(&level, &missile, &origin, Toward::Creature(2), &all);
    assert_eq!(hit_ones(&first), [1]);
    // The next where starts from the hit, facing on, and never hits the one
    // it starts from.
    let next = first[0].origin();
    assert_eq!(
        (next.at, next.heading, next.creature),
        (Spot::new(480, 0), Heading::EAST, Some(1))
    );
    let on = step(Where::Missile, 1, Filters::NONE);
    let second = where_hits(&level, &on, &next, Toward::Ahead, &all);
    assert_eq!(hit_ones(&second), [2]);
    let ring = step(Where::Surround, 1, Filters::NONE);
    assert!(hit_ones(&where_hits(&level, &ring, &next, Toward::Ahead, &all)).is_empty());
}

#[test]
fn self_aims_at_the_caster_whatever_is_targeted_and_limits_nothing() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(240, 0, Side::Enemy),
        at(0, 240, Side::Friend),
        at(1200, 0, Side::Enemy),
    ]);
    let origin = from_caster(&all);
    // Self + Surround: a ring round you, the far enemy targeted or not.
    let ring = step(Where::Surround, 1, CASTER);
    let hits = where_hits(&level, &ring, &origin, Toward::Creature(3), &all);
    assert_eq!(hit_ones(&hits), [1, 2]);
}

#[test]
fn friend_and_enemy_take_the_target_if_theirs_else_the_nearest_in_reach() {
    let level = Level::test_room();
    let all = with_caster(&[
        at(480, 0, Side::Friend),
        at(0, 480, Side::Enemy),
        at(0, -720, Side::Enemy),
    ]);
    let origin = from_caster(&all);
    let missile = step(Where::Missile, 1, ENEMY);
    // Aimed at the friend: the nearest enemy instead.
    let hits = where_hits(&level, &missile, &origin, Toward::Creature(1), &all);
    assert_eq!(hit_ones(&hits), [2]);
    // Aimed at an enemy: that one.
    let hits = where_hits(&level, &missile, &origin, Toward::Creature(3), &all);
    assert_eq!(hit_ones(&hits), [3]);
    // Touch with Enemy turns to the nearest one in arm's reach: none here.
    let touch = step(Where::Touch, 1, ENEMY);
    let hits = where_hits(&level, &touch, &origin, Toward::Ahead, &all);
    assert!(hits.is_empty());
    // Friend + Beam with no friend about: no hit at all (it fizzles).
    let enemies = with_caster(&[at(480, 0, Side::Enemy)]);
    let beam = step(Where::Beam, 1, FRIEND);
    let hits = where_hits(
        &level,
        &beam,
        &from_caster(&enemies),
        Toward::Ahead,
        &enemies,
    );
    assert!(hits.is_empty());
}
