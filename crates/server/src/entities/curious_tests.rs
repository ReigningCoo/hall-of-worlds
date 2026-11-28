//! Curiosity and alarm calls (`curious`): going to look at a noise it can't
//! see, timid ones shying away, calls given when hurt and answered by the band.

use shared::behaviour::{Band, Behaviours};
use shared::creature::CreatureKind;
use shared::noise::{Noise, Source};
use shared::spot::Spot;
use shared::vitals::Points;

use super::choice::others_of;
use super::curious::{LOOK_TICKS, SHY_STEPS, look_into};
use super::noise::{Made, Overheard};
use super::tests::{beside_a_mob, player, run};
use super::{ATTACK_GAP, Doing, Harmer, Maps, Taken, harm_mob};
use crate::connections::tests::TestClient;

/// A cell 22 cells east of the player: further than any mob sees, near
/// enough to its home (2 or 3 cells east of the player) to go and look.
fn out_of_sight(test: &TestClient) -> (i64, i64) {
    let (x, y) = player(test);
    (x + 22, y)
}

/// How many of 100 ticks mob 0, at rest, would go into a noise at `cell` it
/// has turned to, and what it would do the first time.
fn wondering(test: &mut TestClient, cell: (i64, i64)) -> (usize, Option<Doing>) {
    let population = test.world.populations.0.values_mut().next().unwrap();
    let level = population.level.clone();
    let curiosity = Behaviours::built_in_ref()
        .of(population.mobs[0].kind)
        .drives
        .curiosity;
    let mut first = None;
    let mut times = 0;
    for tick in 0..100 {
        let mob = &mut population.mobs[0];
        mob.doing = Doing::Idle;
        if mob.wonder(&level, Spot::middle_of(cell), curiosity, tick) {
            times += 1;
            first.get_or_insert(mob.doing);
        }
    }
    (times, first)
}

#[test]
fn a_curious_mob_sometimes_goes_to_look_at_a_noise_it_cannot_see() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let cell = out_of_sight(&test);
    let (times, first) = wondering(&mut test, cell);
    // A kobold's curiosity is 50: about half the time.
    assert!((20..80).contains(&times), "{times}");
    assert!(matches!(
        first,
        Some(Doing::Investigating { at, looking: None, .. }) if at == cell
    ));
}

#[test]
fn nothing_to_look_into_where_it_can_see_or_too_far_from_home() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    // The player's cell is in plain sight.
    let seen = player(&test);
    assert_eq!(wondering(&mut test, seen).0, 0);
    let (x, y) = player(&test);
    assert_eq!(wondering(&mut test, (x + 40, y)).0, 0);
}

#[test]
fn a_timid_mob_always_shies_from_a_noise_it_cannot_see() {
    let mut test = beside_a_mob(CreatureKind::Rabbit, 3);
    let cell = out_of_sight(&test);
    let (times, first) = wondering(&mut test, cell);
    assert_eq!(times, 100);
    let shying = Doing::Shying {
        from: cell,
        steps_left: SHY_STEPS,
    };
    assert_eq!(first, Some(shying));
}

#[test]
fn once_it_can_see_the_spot_it_looks_a_while_then_goes_home() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let at = player(&test);
    let population = test.world.populations.0.values_mut().next().unwrap();
    let level = population.level.clone();
    let mob = &mut population.mobs[0];
    mob.doing = Doing::Investigating {
        at,
        since: 10,
        looking: None,
    };
    let (mut maps, taken) = (Maps::default(), Taken::new());
    look_into(mob, &level, &mut maps, 10, &taken);
    let looking = Some(10 + LOOK_TICKS);
    assert_eq!(
        mob.doing,
        Doing::Investigating {
            at,
            since: 10,
            looking
        }
    );
    look_into(mob, &level, &mut maps, 10 + LOOK_TICKS - 1, &taken);
    assert!(matches!(mob.doing, Doing::Investigating { .. }));
    look_into(mob, &level, &mut maps, 10 + LOOK_TICKS, &taken);
    assert!(matches!(mob.doing, Doing::GoingHome { .. }));
}

#[test]
fn seeing_someone_while_looking_hands_over_to_its_choices() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let far = out_of_sight(&test);
    let me = test.client.entity_id();
    let population = test.world.populations.0.values_mut().next().unwrap();
    population.mobs[0].doing = Doing::Investigating {
        at: far,
        since: 0,
        looking: None,
    };
    run(&mut test, 1);
    let mob = &test.world.populations.0.values().next().unwrap().mobs[0];
    assert!(
        matches!(mob.doing, Doing::Going { after, gap: ATTACK_GAP, .. } if after == me),
        "{:?}",
        mob.doing
    );
}

/// Kobold 0 at rest hears a call from kobold 1, of its band or not: what it
/// then does.
fn hears_a_call(same_band: bool, doing: Doing) -> (TestClient, Doing) {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let (x, y) = player(&test);
    let population = test.world.populations.0.values_mut().next().unwrap();
    assert!(population.mobs.len() >= 2, "a second mob to remake");
    let band = population.mobs[0].band;
    let caller = population.remake_for_test(1, CreatureKind::Kobold, (x + 1, y));
    caller.band = match same_band {
        true => band,
        false => Band(band.0 ^ 1),
    };
    let call = vec![Made {
        noise: Noise::call(caller.spot, 12),
        by: Some(caller.id),
        sneaker: None,
    }];
    let others = others_of(&population.mobs);
    let level = population.level.clone();
    let mob = &mut population.mobs[0];
    mob.doing = doing;
    let mut overheard = Overheard::of(&call);
    let behaviours = Behaviours::built_in_ref();
    mob.listen(&level, &call, (&others, behaviours), 1, &mut overheard);
    let doing = mob.doing;
    (test, doing)
}

#[test]
fn its_band_answers_a_call_by_coming_over() {
    let (test, doing) = hears_a_call(true, Doing::GoingHome { since: 0 });
    let (x, y) = player(&test);
    assert_eq!(
        doing,
        Doing::Investigating {
            at: (x + 1, y),
            since: 1,
            looking: None,
        }
    );
}

#[test]
fn another_band_s_call_or_a_busy_mob_is_not_answered() {
    let (_, doing) = hears_a_call(false, Doing::Idle);
    assert_eq!(doing, Doing::Idle);
    let fleeing = Doing::Fleeing {
        from: shared::id::Id::for_test(5),
        struck: false,
    };
    let (_, doing) = hears_a_call(true, fleeing);
    assert_eq!(doing, fleeing);
}

#[test]
fn a_hurt_mob_of_a_social_race_calls_out() {
    let mut test = beside_a_mob(CreatureKind::Kobold, 3);
    let place = *test.world.populations.0.keys().next().unwrap();
    let kobold = test.world.populations.0[&place].mobs[0].clone();
    let by = Harmer {
        id: test.client.entity_id(),
        position: player(&test),
        kind: CreatureKind::Human,
        member: None,
    };
    let tick = test.world.action_clock.ticks();
    harm_mob(
        &mut test.world,
        (place, place.1),
        kobold.id,
        Points::whole(1),
        by,
        tick,
    )
    .expect("struck");
    let population = &test.world.populations.0[&place];
    let call = Noise::call(kobold.spot, 12);
    assert!(
        (population.noises.iter()).any(|made| made.noise == call && made.by == Some(kobold.id))
    );
    assert_eq!(call.source, Source::Call);
}
