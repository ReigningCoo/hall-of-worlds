//! Packs defending each other (`defend`).

use shared::behaviour::Band;
use shared::creature::CreatureKind;
use shared::facing::Heading;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;
use shared::vitals::Points;

use super::tests::{beside_a_mob, player};
use super::{ATTACK_GAP, Doing, Harmer, Mob, harm_mob};
use crate::connections::tests::TestClient;

/// A player, a wolf 4 cells east of them (mob 0) and another wolf (mob 1)
/// between them, 2 cells east, of `band` and facing `heading`. The player then
/// strikes the first wolf.
fn wolf_struck_beside(band: Option<Band>, heading: Heading) -> TestClient {
    let mut test = beside_a_mob(CreatureKind::Wolf, 4);
    let (x, y) = player(&test);
    let population = test.world.populations.0.values_mut().next().unwrap();
    assert!(population.mobs.len() >= 2, "a second mob to remake");
    let pack = population.mobs[0].band;
    let other = population.remake_for_test(1, CreatureKind::Wolf, (x + 2, y));
    other.band = band.unwrap_or(pack);
    other.heading = heading;
    let place = *test.world.populations.0.keys().next().unwrap();
    let victim = test.world.populations.0[&place].mobs[0].id;
    let by = Harmer {
        id: test.client.entity_id(),
        position: (x, y),
        kind: CreatureKind::Human,
        member: None,
    };
    let tick = test.world.action_clock.ticks();
    harm_mob(
        &mut test.world,
        (place, place.1),
        victim,
        Points::whole(1),
        by,
        tick,
    )
    .expect("struck");
    test
}

fn second_wolf(test: &TestClient) -> Mob {
    test.world.populations.0.values().next().unwrap().mobs[1].clone()
}

#[test]
fn a_wolf_that_sees_its_pack_struck_joins_and_it_is_self_defence() {
    let test = wolf_struck_beside(None, Heading::EAST);
    let me = test.client.entity_id();
    let wolf = second_wolf(&test);
    let character = CharacterId(me);
    assert_eq!(
        wolf.memory.standing(&character, Feeling::Neutral),
        Feeling::Unfriendly
    );
    assert!(wolf.memory.provoked(&character));
    assert!(
        matches!(wolf.doing, Doing::Going { after, gap, .. } if after == me && gap == ATTACK_GAP),
        "{:?}",
        wolf.doing
    );
    assert_eq!(wolf.target, Some(me));
}

#[test]
fn a_wolf_looking_away_does_not_see_it() {
    let test = wolf_struck_beside(None, Heading::WEST);
    let wolf = second_wolf(&test);
    assert!(!wolf.memory.provoked(&CharacterId(test.client.entity_id())));
    assert_eq!(wolf.doing, Doing::Idle);
}

#[test]
fn another_pack_does_not_join() {
    let test = wolf_struck_beside(Some(Band(12345)), Heading::EAST);
    let wolf = second_wolf(&test);
    assert!(!wolf.memory.provoked(&CharacterId(test.client.entity_id())));
    assert_eq!(wolf.doing, Doing::Idle);
}
