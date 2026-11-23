//! Missiles flown a stretch at a time, on the where hits' test room (cells
//! 240 units, a wall at x = −2 from y = −1 to 1).

use super::*;
use crate::combat::Body;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::spot::Spot;

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

fn missile_step(count: u8, filters: Filters) -> WhereStep {
    WhereStep {
        place: Where::Missile,
        count,
        filters,
    }
}

#[test]
fn flying_in_stretches_hits_the_same_as_all_at_once() {
    let level = Level::test_room();
    let all = [
        at(0, 0, Side::Caster),
        at(720, 0, Side::Neutral),
        at(1200, 0, Side::Enemy),
    ];
    let origin = Origin::from_creature(&all, 0, Heading::EAST);
    let step = missile_step(1, Filters::NONE);
    let path = launch(&level, &step, &origin, Toward::Creature(2), &all).unwrap();
    let whole = missile(&level, &origin, Toward::Creature(2), Filters::NONE, &all);
    let mut done = 0;
    let mut met = None;
    for tick in 1..100 {
        let along = path.along_after(tick * 50);
        if let Some(found) = path.meets((done, along), &all, Some(0)) {
            met = Some(found);
            break;
        }
        done = along;
    }
    let (along, index) = met.unwrap();
    assert_eq!(index, 1);
    assert_eq!(path.hit(&all, index), whole.hit);
    assert_eq!(along, whole.length);
}

#[test]
fn someone_stepping_out_of_the_way_is_missed() {
    let level = Level::test_room();
    let mut all = vec![at(0, 0, Side::Caster), at(720, 0, Side::Enemy)];
    let origin = Origin::from_creature(&all, 0, Heading::EAST);
    let step = missile_step(1, Filters::NONE);
    let path = launch(&level, &step, &origin, Toward::Creature(1), &all).unwrap();
    let part = path.along_after(path.millis() / 3);
    assert_eq!(path.meets((0, part), &all, Some(0)), None);
    // It steps aside before the missile gets there: it lands on the spot.
    all[1].body.at = Spot::new(720, 600);
    assert_eq!(path.meets((part, path.length), &all, Some(0)), None);
    assert_eq!(path.end().at, Spot::new(720, 0));
}

#[test]
fn launching_stops_at_walls_and_speeds_up_by_count() {
    let level = Level::test_room();
    let all = [at(0, 0, Side::Caster)];
    let mut origin = Origin::from_creature(&all, 0, Heading::EAST);
    origin.heading = Heading::WEST;
    let path = launch(
        &level,
        &missile_step(2, Filters::NONE),
        &origin,
        Toward::Ahead,
        &all,
    )
    .unwrap();
    assert!(path.to.x > -2 * 240 && path.to.x < 0, "{:?}", path.to);
    assert_eq!(path.speed(), missile_speed(2));
    assert_eq!(path.at(0), path.from);
    assert_eq!(path.at(path.length), path.to);
    assert_eq!(path.along_after(u64::MAX), path.length);
}

#[test]
fn enemy_missiles_with_no_enemy_about_fizzle() {
    let level = Level::test_room();
    let all = [at(0, 0, Side::Caster), at(480, 0, Side::Neutral)];
    let origin = Origin::from_creature(&all, 0, Heading::EAST);
    let enemy = Filters {
        enemy: true,
        ..Filters::NONE
    };
    let step = missile_step(1, enemy);
    assert_eq!(
        launch(&level, &step, &origin, Toward::Creature(1), &all),
        None
    );
}
