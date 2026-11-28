//! Mobs hearing noises (`shared::noise`): turning to them at rest, sneakers
//! missed, and noises kept for the next run.

use shared::creature::CreatureKind;
use shared::facing::Heading;
use shared::id::Id;
use shared::noise::{Footsteps, Noise, Source};
use shared::spot::Spot;

use super::noise::{Made, Overheard};
use super::tests::{beside_a_mob, player};

fn made(noise: Noise, sneaker: Option<Id>) -> Vec<Made> {
    vec![Made {
        noise,
        by: sneaker,
        sneaker,
    }]
}

/// The mob's level and the mob, to listen.
fn listen(test: &mut crate::connections::tests::TestClient, made: &[Made], tick: u64) -> Overheard {
    let population = test.world.populations.0.values_mut().next().unwrap();
    let mut overheard = Overheard::of(made);
    let level = &population.level;
    population.mobs[0].listen(level, made, tick, &mut overheard);
    overheard
}

#[test]
fn a_mob_at_rest_turns_to_a_blow_it_hears_after_its_reaction() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let population = test.world.populations.0.values_mut().next().unwrap();
    population.mobs[0].heading = Heading::EAST;
    let reaction = population.mobs[0].reaction();
    let blow = Noise::new(Source::Blow, Spot::middle_of(player(&test)));
    listen(&mut test, &made(blow, None), 1);
    let heading = |test: &crate::connections::tests::TestClient| {
        test.world.populations.0.values().next().unwrap().mobs[0].heading
    };
    // Alert, but it hasn't turned yet: a moment first.
    assert_eq!(heading(&test), Heading::EAST);
    listen(&mut test, &[], reaction);
    assert_eq!(heading(&test), Heading::EAST);
    listen(&mut test, &[], 1 + reaction);
    assert_eq!(heading(&test), Heading::WEST);
}

#[test]
fn a_sneaker_is_sometimes_missed_by_a_mob_in_reach_and_never_out_of_it() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 2);
    let me = Id::for_test(99);
    let sneaking = Footsteps {
        sneaking: true,
        ..Footsteps::default()
    };
    let steps = made(
        Noise::footsteps(Spot::middle_of(player(&test)), sneaking),
        Some(me),
    );
    let missed: Vec<bool> = (0..100)
        .map(|tick| listen(&mut test, &steps, tick).unheard(&steps) == vec![me])
        .collect();
    assert!(missed.iter().any(|&m| m) && missed.iter().any(|&m| !m));

    // A master sneaker isn't in a kobold's reach at all: nothing to learn.
    let master = Footsteps {
        skill: shared::movement::FULL,
        ..sneaking
    };
    let steps = made(
        Noise::footsteps(Spot::middle_of(player(&test)), master),
        Some(me),
    );
    assert!((0..100).all(|tick| listen(&mut test, &steps, tick).unheard(&steps).is_empty()));
}

#[test]
fn a_noise_made_is_kept_for_the_level_s_mobs() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let place = *test.world.populations.0.keys().next().unwrap();
    let at = Spot::middle_of(player(&test));
    super::make_noise(&mut test.world, place, Source::Door, at);
    let population = test.world.populations.0.values().next().unwrap();
    assert_eq!(population.noises, vec![Noise::new(Source::Door, at)]);
    super::tests::run(&mut test, 1);
    let population = test.world.populations.0.values().next().unwrap();
    assert!(population.noises.is_empty());
}
