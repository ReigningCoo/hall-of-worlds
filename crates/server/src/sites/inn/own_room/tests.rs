use shared::facing::Heading;
use shared::furniture::Held;
use shared::inn::{self, OPEN_TIER, spots};
use shared::item::{ItemKind, Quality};
use shared::protocol::{BenchAsk, ClientMessage, DevGive};
use shared::spot::Spot;
use shared::world::WorldSeed;

use super::hidden_in_own_room;
use crate::ServerWorld;
use crate::connections::tests::TestClient;
use crate::storage::{SiteId, Storage, tests::TestFolder};

/// The open tier's chest (a small one in its north-east corner, beside the
/// bed) and where to stand in front of it.
fn the_chest() -> ((i64, i64), (i64, i64)) {
    let room = inn::own_rooms()[0];
    assert_eq!(room.tier, OPEN_TIER);
    let chest = (room.rect.x + room.rect.width - 2, room.rect.y);
    (chest, (chest.0, chest.1 + 1))
}

fn ring() -> ClientMessage {
    ClientMessage::DevGiveItem(DevGive {
        kind: ItemKind::Ring,
        size: None,
        magical: false,
        quality: Quality::Common,
        set: false,
        favour: None,
        seed: Some(1),
        count: 1,
        identical: true,
        request: shared::item_form::RollRequest::NONE,
        herb: None,
        to: None,
    })
}

/// Into the inn, up the stairs and into the open tier's own room, standing
/// before its chest facing it.
fn in_own_room(test: &mut TestClient) {
    test.send(&[ClientMessage::DevToInn]);
    let visit = test.client.visit.as_mut().expect("in the inn");
    visit.position = Spot::middle_of(spots::STAIRS);
    test.send(&[ClientMessage::Up]);
    let (_, before) = the_chest();
    test.send(&[
        ClientMessage::DevTeleport {
            x: before.0,
            y: before.1,
        },
        ClientMessage::Face(Heading::NORTH),
    ]);
    let visit = test.client.visit.as_ref().expect("in the inn");
    assert_eq!((visit.index, visit.position()), (inn::UPSTAIRS, before));
}

/// Gives a ring and puts it in the chest: its id.
fn ring_in_chest(test: &mut TestClient) -> shared::id::Id {
    test.send(&[ring()]);
    let item = test.client.inventory.items[0].item.id;
    let (chest, _) = the_chest();
    test.send(&[ClientMessage::Bench(BenchAsk::Put { bench: chest, item })]);
    item
}

fn own_id(test: &TestClient) -> SiteId {
    SiteId::InnRoom(test.client.character_id.unwrap())
}

/// What the chest holds as the player sees it.
fn in_chest(test: &mut TestClient) -> Vec<shared::id::Id> {
    let (chest, _) = the_chest();
    let visit = test.client.visit.as_ref().unwrap();
    let here = visit.held_at(chest.0, chest.1).expect("the chest");
    match crate::sites::settle_held(&mut test.world, here.key, here.holder, 15) {
        Held::Store(stored) => stored.items().map(|item| item.id).collect(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn what_is_put_in_the_chest_is_kept_in_their_own_copy() {
    let mut test = TestClient::new();
    in_own_room(&mut test);
    let item = ring_in_chest(&mut test);
    assert!(test.client.inventory.items.is_empty());
    assert_eq!(in_chest(&mut test), [item]);
    let (chest, _) = the_chest();
    let own = test.world.own_room(own_id(&test)).expect("kept");
    assert!(own.stores.contains_key(&chest));
    let instance = test.client.visit.as_ref().unwrap().id;
    let shared = test.world.site(instance).levels.get(&inn::UPSTAIRS);
    assert!(shared.is_none_or(|level| level.stores.is_empty()));

    // Out of the inn: kept with the character, let go of by the world.
    assert!(super::super::leave_inn(&mut test.client));
    test.send(&[]);
    assert!(test.world.own_room(own_id(&test)).is_none());
    assert!(test.client.inn.room.stores.contains_key(&chest));

    // Back in: there again.
    in_own_room(&mut test);
    assert_eq!(in_chest(&mut test), [item]);
}

#[test]
fn the_room_is_saved_with_the_character() {
    let folder = TestFolder::new();
    let seed = WorldSeed::new(1);
    let open = || ServerWorld::open(seed, Storage::in_folder(&folder.0, seed.id())).unwrap();
    let mut test = TestClient::of(open());
    in_own_room(&mut test);
    let item = ring_in_chest(&mut test);
    test.leave();
    let mut test = TestClient::of(open());
    let (chest, _) = the_chest();
    assert!(test.client.inn.room.stores.contains_key(&chest));
    in_own_room(&mut test);
    assert_eq!(in_chest(&mut test), [item]);
}

#[test]
fn what_is_dropped_there_lies_in_their_own_copy() {
    let mut test = TestClient::new();
    in_own_room(&mut test);
    test.send(&[ring()]);
    let item = test.client.inventory.items[0].item.id;
    test.send(&[ClientMessage::DropItem { item }]);
    let own = test.world.own_room(own_id(&test)).expect("kept");
    assert_eq!(own.ground.len(), 1);
    let instance = test.client.visit.as_ref().unwrap().id;
    let shared = test.world.site(instance).levels.get(&inn::UPSTAIRS);
    assert!(shared.is_none_or(|level| level.ground.is_empty()));
}

#[test]
fn nobody_is_seen_in_an_own_room() {
    let instance = Some((SiteId::InnInstance(1), inn::UPSTAIRS));
    let room = inn::own_rooms()[0].rect;
    assert!(hidden_in_own_room(instance, (room.x, room.y)));
    assert!(!hidden_in_own_room(instance, spots::STAIRS));
    let ground = Some((SiteId::InnInstance(1), inn::GROUND));
    assert!(!hidden_in_own_room(ground, (room.x, room.y)));
    let elsewhere = Some((SiteId::Wilderness, inn::UPSTAIRS));
    assert!(!hidden_in_own_room(elsewhere, (room.x, room.y)));
}
