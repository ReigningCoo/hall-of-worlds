//! How wheres look (`Look`, `shown`), on the test room (see `hits_tests`).

use super::*;
use crate::combat::Body;
use crate::creature::Size;
use crate::dungeon::Level;
use crate::facing::Heading;
use crate::spot::{CELL_SPAN, Spot};

fn caster() -> Vec<Creature> {
    vec![Creature {
        body: Body {
            at: Spot::new(0, 0),
            height: 0,
            size: Size::Medium,
        },
        side: Side::Caster,
    }]
}

#[test]
fn a_where_looks_like_every_effect_after_it() {
    let spell = Spell::new(vec![
        Step::at(Where::Missile, 1),
        Step::effects(&[(Effect::Heal, 1)]),
        Step::at(Where::Surround, 1),
        Step::effects(&[(Effect::Void, 1), (Effect::Fire, 2), (Effect::Heal, 1)]),
    ]);
    let first: Vec<Part> = Look::from_step(&spell, 0).iter().collect();
    assert_eq!(
        first,
        [
            Part::plain(Effect::Heal),
            Part::reversed(Effect::Fire),
            Part::reversed(Effect::Heal),
        ]
    );
    let later: Vec<Part> = Look::from_step(&spell, 2).iter().collect();
    assert_eq!(
        later,
        [Part::reversed(Effect::Fire), Part::reversed(Effect::Heal)]
    );
}

#[test]
fn a_look_keeps_at_most_four_parts() {
    let step = EffectStep {
        effects: [
            Effect::Fire,
            Effect::Water,
            Effect::Air,
            Effect::Earth,
            Effect::Light,
        ]
        .map(|effect| (effect, 1))
        .to_vec(),
    };
    assert_eq!(Look::of_effects(&step).iter().count(), LOOK_PARTS);
    assert_eq!(Look::default().iter().count(), 0);
}

#[test]
fn shown_beams_end_at_their_reach_and_rings_round_the_caster() {
    let level = Level::test_room();
    let creatures = caster();
    let origin = Origin::from_creature(&creatures, 0, Heading::EAST);
    let beam = WhereStep {
        place: Where::Beam,
        count: 2,
        filters: Filters::NONE,
        more: Vec::new(),
    };
    let hits = where_hits(&level, &beam, &origin, Toward::Ahead, &creatures);
    let look = Look::default();
    let shown = shown(
        &level,
        (&beam, look),
        &origin,
        Toward::Ahead,
        &creatures,
        &hits,
    )
    .expect("unfiltered never fizzles");
    assert_eq!(shown.ends.len(), 3);
    assert_eq!(shown.heading, Heading::EAST);
    assert_eq!(shown.hits.len(), hits.len());
    assert!(shown.ends.iter().all(|end| end.x > 0));

    let ring = WhereStep {
        place: Where::Surround,
        ..beam.clone()
    };
    let shown = shown_of(&level, &ring, &origin, &creatures);
    assert!(shown.ends.is_empty());
    assert!(u64::from(shown.reach) > 2 * CELL_SPAN as u64);
}

fn shown_of(level: &Level, step: &WhereStep, origin: &Origin, creatures: &[Creature]) -> Shown {
    let hits = where_hits(level, step, origin, Toward::Ahead, creatures);
    shown(
        level,
        (step, Look::default()),
        origin,
        Toward::Ahead,
        creatures,
        &hits,
    )
    .expect("unfiltered never fizzles")
}
