//! Make Campfire through a connected client: learned on carrying kindling, a
//! fire lit on the ground faced and fed, set on the player's level, and Ash
//! left on the ground once it burns out.

use shared::ability::Ability;
use shared::campfire::{feed_ticks, light_ticks};
use shared::facing::Heading;
use shared::id::Id;
use shared::item::{ItemKind, StackLimits};
use shared::material::Material;
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::woodcutting::new_kindling;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// A player in a castle with room to the east, facing it, carrying three
/// pieces of pine kindling.
fn with_kindling() -> TestClient {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let visit = test.client.visit.as_mut().unwrap();
    let at = visit.room_to_walk_east(2);
    visit.move_for_test(at);
    let kindling = new_kindling(Material::Pine, 3, Id::for_test(5), None);
    let stand_ins = &mut shared::id::stand_ins();
    (test.client.inventory)
        .give(kindling, StackLimits::NONE, stand_ins)
        .unwrap();
    let learned = (test.client.sheet.abilities).learn_from(&test.client.inventory);
    assert!(learned, "Make Campfire, on first carrying kindling");
    test.send(&[ClientMessage::Face(Heading::EAST)]);
    test
}

fn make_campfire(test: &mut TestClient) {
    test.send(&[ClientMessage::UseAbility {
        ability: Ability::MakeCampfire,
        target: None,
    }]);
}

/// Sets the fires on the player's level, as every tick does.
fn sync(test: &mut TestClient) {
    crate::sites::sync_campfires(std::slice::from_mut(&mut test.client), &mut test.world);
}

/// The fires on the player's level, with their pieces left.
fn fires(test: &TestClient) -> Vec<((i64, i64), u8)> {
    let visit = test.client.visit.as_ref().unwrap();
    visit.cells().campfires().collect()
}

/// Pieces of kindling carried.
fn kindling(test: &TestClient) -> u32 {
    test.client.inventory.count_of(ItemKind::Kindling)
}

#[test]
fn kindling_lights_a_fire_on_the_ground_faced_and_feeds_it() {
    let mut test = with_kindling();
    make_campfire(&mut test);
    assert!(test.client.fight.pending.is_some(), "lighting");
    run(&mut test, light_ticks() + 1, false);
    assert!(received(&mut test).iter().any(|message| matches!(
        message,
        ServerMessage::AbilityLanded {
            landed: Landed::Campfire { pieces: 1 },
            ..
        }
    )));
    assert_eq!(kindling(&test), 2);
    sync(&mut test);
    let lit = fires(&test);
    assert_eq!(lit.len(), 1, "{lit:?}");
    assert_eq!(lit[0].1, 1);
    // The fire is faced now: the next piece feeds it.
    make_campfire(&mut test);
    run(&mut test, feed_ticks() + 1, false);
    assert_eq!(kindling(&test), 1);
    sync(&mut test);
    assert_eq!(fires(&test), vec![(lit[0].0, 2)]);
}

#[test]
fn a_fire_burnt_out_leaves_ash_where_it_was() {
    let mut test = with_kindling();
    make_campfire(&mut test);
    run(&mut test, light_ticks() + 1, false);
    sync(&mut test);
    assert_eq!(fires(&test).len(), 1);
    // Ten minutes on: a piece of pine burns under seven.
    let later = test.world.server_clock.seconds() + 600.0;
    test.world.server_clock = crate::clock::ServerClock::new(later);
    sync(&mut test);
    assert!(fires(&test).is_empty(), "burnt out");
    let visit = test.client.visit.as_ref().unwrap();
    let (site, index) = (visit.id(), visit.index());
    let ground = test.world.site(site).levels[&index].ground.clone();
    let ash: u32 = (ground.values().flatten())
        .filter(|item| item.kind == ItemKind::Ash)
        .map(|item| item.count())
        .sum();
    assert_eq!(ash, 1, "a handful for the piece burnt");
}
