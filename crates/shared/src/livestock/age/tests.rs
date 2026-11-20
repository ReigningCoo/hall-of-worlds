use super::{LIFE_CYCLES, age_hours, stage_of};
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::livestock::{Livestock, Stage};

#[test]
fn people_are_always_grown() {
    for n in 0..50 {
        assert_eq!(
            stage_of(CreatureKind::Human, Id::for_test(n), 0),
            Stage::Adult
        );
    }
}

#[test]
fn a_herd_has_every_age() {
    let stages: Vec<Stage> = (0..400)
        .map(|n| stage_of(CreatureKind::Cattle, Id::for_test(n * 7919 + 1), 5_000))
        .collect();
    for stage in [Stage::Young, Stage::Juvenile, Stage::Adult] {
        assert!(stages.contains(&stage), "{stage:?}");
    }
    let grown = stages.iter().filter(|&&s| s == Stage::Adult).count();
    // About (cycles - 1) of every `LIFE_CYCLES`.
    assert!(grown > 400 * 3 / 5 && grown < 400 * 9 / 10, "{grown}");
}

#[test]
fn ages_grow_with_the_clock() {
    let stages = &Livestock::built_in().get("cattle").expect("cattle").stages;
    let id = Id::for_test(42);
    let now = age_hours(id, 0, stages);
    let cycle = stages.juvenile_until * LIFE_CYCLES;
    let later = age_hours(id, 60 * 10, stages);
    assert_eq!(later, (now + 10) % cycle);
}
