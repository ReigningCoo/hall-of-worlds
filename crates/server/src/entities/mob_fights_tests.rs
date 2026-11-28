//! Mobs fighting mobs (`foes`, `mob_blows`).

use shared::behaviour::{Band, Member};
use shared::creature::CreatureKind;
use shared::facing::Heading;
use shared::id::Id;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;
use shared::vitals::Points;

use super::fight::MobBlow;
use super::mob_blows::strike_mob;
use super::tests::{beside_a_mob, player, run};
use super::{ATTACK_GAP, Doing, Harmer, Mob, harm_mob};
use crate::connections::tests::TestClient;

/// Two wolves facing each other, mob 0 four cells east of the player and mob 1
/// two cells east, the second of `band` (else the first's pack), with no other
/// mobs about and the player unseen (god mode).
fn two_wolves(band: Option<Band>) -> TestClient {
    let mut test = beside_a_mob(CreatureKind::Wolf, 4);
    test.client.god_mode = true;
    let (x, y) = player(&test);
    let population = test.world.populations.0.values_mut().next().unwrap();
    assert!(population.mobs.len() >= 2, "a second mob to remake");
    let pack = population.mobs[0].band;
    let other = population.remake_for_test(1, CreatureKind::Wolf, (x + 2, y));
    other.band = band.unwrap_or(pack);
    other.heading = Heading::EAST;
    population.mobs.truncate(2);
    let kept: Vec<Id> = population.mobs.iter().map(|mob| mob.id).collect();
    population.due.retain(|(_, id)| kept.contains(id));
    test
}

fn wolf(test: &TestClient, index: usize) -> Mob {
    test.world.populations.0.values().next().unwrap().mobs[index].clone()
}

fn going_after(mob: &Mob, them: Id) -> bool {
    matches!(mob.doing, Doing::Going { after, gap, .. } if after == them && gap == ATTACK_GAP)
}

#[test]
fn rival_packs_go_for_each_other() {
    let mut test = two_wolves(Some(Band(12345)));
    run(&mut test, 1);
    let (first, second) = (wolf(&test, 0), wolf(&test, 1));
    assert!(going_after(&first, second.id), "{:?}", first.doing);
    assert!(going_after(&second, first.id), "{:?}", second.doing);
}

#[test]
fn one_pack_leaves_itself_alone() {
    let mut test = two_wolves(None);
    run(&mut test, 1);
    assert_eq!(wolf(&test, 0).doing, Doing::Idle);
    assert_eq!(wolf(&test, 1).doing, Doing::Idle);
}

#[test]
fn penned_animals_are_never_fought() {
    let mut test = two_wolves(Some(Band(12345)));
    let population = test.world.populations.0.values_mut().next().unwrap();
    population.mobs[1].penned = true;
    run(&mut test, 1);
    assert_eq!(wolf(&test, 0).doing, Doing::Idle);
}

#[test]
fn rival_packs_harm_each_other() {
    let mut test = two_wolves(Some(Band(12345)));
    let ids = [wolf(&test, 0).id, wolf(&test, 1).id];
    run(&mut test, 200);
    let harmed: Vec<Id> = test.world.harmed.iter().map(|harm| harm.id).collect();
    assert!(
        ids.iter().any(|id| harmed.contains(id)),
        "nobody harmed: {harmed:?}"
    );
}

#[test]
fn a_mobs_blow_harms_another_and_it_remembers_the_attacker() {
    let mut test = two_wolves(Some(Band(12345)));
    let place = *test.world.populations.0.keys().next().unwrap();
    let (attacker, target) = (wolf(&test, 0), wolf(&test, 1));
    let blow = MobBlow::Landed {
        mob: attacker.id,
        target: target.id,
        kind: attacker.kind,
        attributes: attacker.attributes,
        wielded: attacker.wielded,
        guard: Default::default(),
        swing: attacker.fighter().swing(0),
    };
    let tick = test.world.action_clock.ticks();
    strike_mob(&mut test.world, place, blow, tick);
    assert!(test.world.harmed.iter().any(|harm| harm.id == target.id));
    let target = wolf(&test, 1);
    let them = CharacterId(attacker.id);
    assert_eq!(
        target.memory.standing(&them, Feeling::None),
        Feeling::Unfriendly
    );
    assert!(going_after(&target, attacker.id), "{:?}", target.doing);
}

#[test]
fn a_pack_defends_its_own_against_another_mob() {
    let mut test = two_wolves(None);
    let place = *test.world.populations.0.keys().next().unwrap();
    let (victim, defender) = (wolf(&test, 0), wolf(&test, 1));
    let rival = Id::for_test(77);
    let by = Harmer {
        id: rival,
        position: victim.position,
        kind: CreatureKind::Wolf,
        member: Some(Member {
            kind: CreatureKind::Wolf,
            band: Band(12345),
        }),
    };
    let tick = test.world.action_clock.ticks();
    harm_mob(
        &mut test.world,
        (place, place.1),
        victim.id,
        Points::whole(1),
        by,
        tick,
    )
    .expect("struck");
    let defender_now = wolf(&test, 1);
    assert_eq!(defender_now.id, defender.id);
    assert_eq!(
        defender_now
            .memory
            .standing(&CharacterId(rival), Feeling::None),
        Feeling::Unfriendly
    );
    assert!(
        going_after(&defender_now, rival),
        "{:?}",
        defender_now.doing
    );
}
