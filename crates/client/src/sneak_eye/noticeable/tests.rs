use super::*;
use shared::dungeon::Cell;

fn room() -> Level {
    let mut level = Level::new(0, 0, 20, 3);
    for x in 0..20 {
        for y in 0..3 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

fn at(x: i64) -> Spot {
    Spot::middle_of((x, 1))
}

#[test]
fn nobody_in_sight_leaves_the_eye_closed() {
    let none: [(CreatureKind, Spot); 0] = [];
    assert_eq!(noticeable(&room(), at(0), Footsteps::default(), none), 0);
}

#[test]
fn nearer_and_keener_ears_open_it_and_sneaking_closes_it() {
    let level = room();
    let walking = Footsteps::default();
    let sneaking = Footsteps {
        sneaking: true,
        ..walking
    };
    let kobold = |x| [(CreatureKind::Kobold, at(x))];
    assert_eq!(noticeable(&level, at(0), walking, kobold(2)), 1000);
    assert!(noticeable(&level, at(0), sneaking, kobold(2)) < 1000);
    assert_eq!(noticeable(&level, at(0), sneaking, kobold(6)), 0);
    // A wolf hears further than a kobold: the best of them counts.
    let both = [(CreatureKind::Kobold, at(3)), (CreatureKind::Wolf, at(3))];
    let wolf = noticeable(&level, at(0), sneaking, [both[1]]);
    assert_eq!(noticeable(&level, at(0), sneaking, both), wolf);
    assert!(wolf > noticeable(&level, at(0), sneaking, kobold(3)));
}
