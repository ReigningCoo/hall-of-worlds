use super::*;
use crate::creature::Acuity;
use crate::door::Swing;
use crate::dungeon::{Cell, Level};
use crate::rng::Seed;
use crate::spot::{CELL_SPAN, Spot};

/// An open room 30 × 3 cells, its middle row y = 1.
fn room() -> Level {
    let mut level = Level::new(0, 0, 30, 3);
    for x in 0..30 {
        for y in 0..3 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

fn at(x: i64) -> Spot {
    Spot::middle_of((x, 1))
}

fn hearer(x: i64, hearing: Acuity) -> Hearer {
    Hearer {
        at: at(x),
        hearing,
        seed: Seed::new(7),
    }
}

#[test]
fn a_blow_is_heard_as_far_as_fights_were() {
    let level = room();
    let blow = Noise::new(Source::Blow, at(0));
    let cells = |hearing| reach(&level, &blow, &hearer(20, hearing)) as i64 / CELL_SPAN;
    assert_eq!(cells(Acuity::None), 0);
    assert_eq!(cells(Acuity::Poor), 4);
    assert_eq!(cells(Acuity::Normal), 8);
    assert_eq!(cells(Acuity::Keen), 14);
}

#[test]
fn certain_within_half_its_reach_then_less_likely_then_never() {
    let level = room();
    let blow = Noise::new(Source::Blow, at(0));
    let chance_at = |x| chance(&level, &blow, &hearer(x, Acuity::Normal));
    assert_eq!(chance_at(4), 1000);
    assert_eq!(chance_at(6), 500);
    assert_eq!(chance_at(8), 0);
    assert!(chance_at(5) > chance_at(7));
}

#[test]
fn walls_and_closed_doors_muffle() {
    let mut level = room();
    let blow = Noise::new(Source::Blow, at(0));
    let open = reach(&level, &blow, &hearer(6, Acuity::Normal));
    level.set(3, 1, Cell::Door);
    level.set_swing(3, 1, Swing::Closed);
    let door = reach(&level, &blow, &hearer(6, Acuity::Normal));
    assert_eq!(door, open / 2);
    for y in 0..3 {
        level.set(3, y, Cell::Wall);
    }
    let wall = reach(&level, &blow, &hearer(6, Acuity::Normal));
    assert_eq!(wall, open * WALL_PER_MILLE / 1000);
    // The cell the listener stands in is never between.
    assert_eq!(reach(&level, &blow, &hearer(3, Acuity::Normal)), open);
}

#[test]
fn sneaking_and_the_skill_quieten_footsteps_and_armour_adds() {
    let walking = Footsteps::default().loudness();
    assert_eq!(walking, WALK_CELLS * CELL_SPAN as u32);
    let plate = Footsteps {
        armour_grams: 25_000,
        ..Footsteps::default()
    };
    assert_eq!(plate.loudness(), 2 * walking);
    let sneaking = Footsteps {
        sneaking: true,
        ..Footsteps::default()
    };
    assert_eq!(sneaking.loudness(), walking / 2);
    let master = Footsteps {
        sneaking: true,
        skill: crate::movement::FULL,
        ..Footsteps::default()
    };
    assert_eq!(master.loudness(), walking / 8);
}

#[test]
fn the_roll_is_the_same_on_both_sides_and_varies() {
    let level = room();
    let steps = Noise::footsteps(at(0), Footsteps::default());
    let listener = hearer(4, Acuity::Normal);
    let heard_at = |tick| heard(&level, &steps, &listener, tick);
    let all: Vec<bool> = (0..200).map(heard_at).collect();
    assert_eq!(all, (0..200).map(heard_at).collect::<Vec<_>>());
    assert!(all.iter().any(|&h| h) && all.iter().any(|&h| !h));
    // Never beyond the reach, whatever the roll.
    assert!((0..200).all(|tick| !heard(&level, &steps, &hearer(5, Acuity::Normal), tick)));
}

#[test]
fn a_call_carries_by_its_loudness() {
    let call = Noise::call(at(0), 20);
    assert_eq!(call.loudness, 20 * CELL_SPAN as u32);
    assert_eq!(call.source, Source::Call);
}
