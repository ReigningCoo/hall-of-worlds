use shared::id::Id;
use shared::inn::{self, RoomKind};
use shared::item::{Item, ItemKind};
use shared::spot::Spot;
use std::time::{Duration, Instant};

use super::{LITTER_SECONDS, clear_litter};
use crate::connections::tests::TestClient;
use crate::storage::SiteId;

/// In the inn (instance 1), with an apple lying in the middle of a room of
/// each kind given, by its spot.
fn litter_in(kinds: &[RoomKind]) -> (TestClient, Vec<Spot>) {
    let mut test = TestClient::new();
    test.send(&[shared::protocol::ClientMessage::DevToInn]);
    let id = SiteId::InnInstance(1);
    assert_eq!(test.client.visit.as_ref().map(|visit| visit.id), Some(id));
    let rooms = inn::rooms(inn::GROUND);
    let mut spots = Vec::new();
    let level = test
        .world
        .site_mut(id)
        .levels
        .entry(inn::GROUND)
        .or_default();
    for (number, kind) in kinds.iter().enumerate() {
        let room = rooms.iter().find(|room| room.kind == *kind).unwrap();
        let rect = room.rects[0];
        let at = Spot::middle_of((rect.x + rect.width / 2, rect.y + rect.height / 2));
        level.ground.insert(
            at,
            vec![Item::new(Id::for_test(number as u128 + 1), ItemKind::Ring)],
        );
        spots.push(at);
    }
    (test, spots)
}

fn lying(test: &mut TestClient, at: Spot) -> bool {
    let site = test.world.site(SiteId::InnInstance(1));
    site.levels[&inn::GROUND].ground.contains_key(&at)
}

#[test]
fn things_left_in_public_rooms_are_cleared_after_a_while() {
    let kinds = [RoomKind::Main, RoomKind::Kitchen, RoomKind::PrivateRoom];
    let (mut test, spots) = litter_in(&kinds);
    let start = Instant::now();
    clear_litter(&mut test.world, start);
    let nearly = start + Duration::from_secs(LITTER_SECONDS - 1);
    clear_litter(&mut test.world, nearly);
    assert!(spots.iter().all(|&at| lying(&mut test, at)));
    clear_litter(&mut test.world, start + Duration::from_secs(LITTER_SECONDS));
    assert!(!lying(&mut test, spots[0]));
    assert!(!lying(&mut test, spots[1]));
    // Private rooms are their own (PR 4): left alone.
    assert!(lying(&mut test, spots[2]));
}
