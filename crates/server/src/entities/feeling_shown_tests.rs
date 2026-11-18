//! A mob going after a player is shown to them as unfriendly, and blocks them.

use shared::clearance::Crowding;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::protocol::{CharacterId, ServerMessage};
use shared::sheet::Feeling;

use super::tests::beside_a_mob;
use crate::connections::tests::TestClient;

/// A neutral wolf two cells from the player, at rest.
fn beside_a_wolf() -> TestClient {
    let test = beside_a_mob(CreatureKind::Wolf, 2);
    assert_eq!(told(&test), Feeling::Neutral);
    test
}

fn wolf(test: &mut TestClient) -> &mut super::Mob {
    let population = test.world.populations.0.values_mut().next().unwrap();
    &mut population.mobs[0]
}

/// How the wolf's feeling is shown to this character.
fn shown_to(test: &TestClient, character: Option<&CharacterId>) -> Feeling {
    let mob = &test.world.populations.0.values().next().unwrap().mobs[0];
    mob.feeling_shown(test.client.inventory.creature, character)
}

fn told(test: &TestClient) -> Feeling {
    shown_to(test, test.client.character_id.as_ref())
}

/// How the wolf crowds the player's moves.
fn crowding(test: &TestClient) -> Crowding {
    let visit = test.client.visit.as_ref().unwrap();
    let kind = test.client.inventory.creature;
    let character = test.client.character_id.as_ref();
    let others = test.world.crowd_round(visit.population(), kind, character);
    others[0].crowding
}

/// What the player's token for the wolf says, told afresh.
fn token(test: &mut TestClient) -> Option<Feeling> {
    test.client.seen_entities.clear();
    let answers = test.send(&[]);
    answers.iter().find_map(|answer| match answer {
        ServerMessage::EntitySeen(look) if look.kind == CreatureKind::Wolf => Some(look.feeling),
        _ => None,
    })
}

#[test]
fn a_neutral_wolf_after_a_player_is_unfriendly_to_them_until_it_gives_up() {
    let mut test = beside_a_wolf();
    assert_eq!(
        crowding(&test),
        Crowding::Squeeze,
        "walked past, or swapped"
    );
    let me = test.client.character_id.unwrap();
    wolf(&mut test).target = Some(me.0);
    assert_eq!(token(&mut test), Some(Feeling::Unfriendly));
    assert_eq!(
        crowding(&test),
        Crowding::Block,
        "no step or swap through it"
    );
    // Its real feeling, for its own choices, is unchanged.
    let kind = test.client.inventory.creature;
    assert_eq!(wolf(&mut test).feeling(kind, Some(&me)), Feeling::Neutral);
    // Given up.
    wolf(&mut test).target = None;
    assert_eq!(token(&mut test), Some(Feeling::Neutral));
    assert_eq!(crowding(&test), Crowding::Squeeze);
}

#[test]
fn another_player_it_is_not_after_sees_it_as_before() {
    let mut test = beside_a_wolf();
    let me = test.client.character_id.unwrap();
    wolf(&mut test).target = Some(me.0);
    let someone_else = CharacterId(Id::for_test(9_999));
    assert_eq!(shown_to(&test, Some(&someone_else)), Feeling::Neutral);
    assert_eq!(shown_to(&test, None), Feeling::Neutral);
}
