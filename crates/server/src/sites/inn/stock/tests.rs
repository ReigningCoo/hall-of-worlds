use shared::furniture::{Furniture, Piece};
use shared::inn::{self, RoomKind};
use shared::item::ItemKind;
use shared::protocol::ClientMessage;
use std::time::{Duration, Instant};

use super::{RESTOCK_SECONDS, restock_inns};
use crate::connections::tests::TestClient;
use crate::storage::SiteId;

const ID: SiteId = SiteId::InnInstance(1);

/// In the inn's first instance, its ground floor made and stocked.
fn in_the_inn() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevToInn]);
    assert_eq!(test.client.visit.as_ref().map(|visit| visit.id()), Some(ID));
    test
}

/// The ground floor's pieces that `kind` picks.
fn pieces(kind: fn(Furniture) -> bool) -> Vec<Piece> {
    let level = inn::level(inn::GROUND).expect("the ground floor").level;
    let on_top: Vec<Piece> = level.furniture_on_top().copied().collect();
    (level.furniture().copied().chain(on_top))
        .filter(|piece| kind(piece.furniture))
        .collect()
}

fn in_room(piece: &Piece, kind: RoomKind) -> bool {
    let (x, y) = piece.at;
    (inn::rooms(inn::GROUND).iter()).any(|room| room.kind == kind && room.contains(x, y))
}

#[test]
fn everything_starts_stocked() {
    let mut test = in_the_inn();
    let saved = &test.world.site(ID).levels[&inn::GROUND];
    for piece in pieces(|f| matches!(f, Furniture::CaskRack { .. })) {
        let Furniture::CaskRack { rack } = piece.furniture else {
            unreachable!()
        };
        let casks = &saved.racks[&piece.at];
        assert_eq!(casks.items().count(), rack.capacity(), "{piece:?}");
        let brewing = in_room(&piece, RoomKind::Brewing);
        for cask in casks.items() {
            let form = cask.cask().expect("a cask");
            assert_eq!(form.is_empty(), brewing, "{piece:?}");
        }
    }
    for piece in pieces(|f| matches!(f, Furniture::BottleRack { .. })) {
        let Furniture::BottleRack { rack } = piece.furniture else {
            unreachable!()
        };
        let bottles = &saved.bottle_racks[&piece.at];
        assert_eq!(bottles.items().count(), rack.capacity(), "{piece:?}");
    }
    for piece in pieces(|f| matches!(f, Furniture::Fireplace { .. })) {
        assert!(saved.fireplaces[&piece.at].always_lit);
    }
    for piece in pieces(|f| matches!(f, Furniture::CraftingBench { .. })) {
        let Furniture::CraftingBench { craft } = piece.furniture else {
            unreachable!()
        };
        let tools = &saved.benches[&piece.at];
        assert_eq!(tools.always_lit, craft.is_hearth());
        assert_eq!(tools.items().count(), craft.tools().len(), "{piece:?}");
    }
    let chests = pieces(|f| matches!(f, Furniture::Chest { .. }));
    for piece in chests.iter().filter(|p| in_room(p, RoomKind::Treasury)) {
        let stored = &saved.stores[&piece.at];
        assert!(stored.items().any(|item| item.kind == ItemKind::GoldCoin));
    }
}

fn on_rack(test: &mut TestClient, at: (i64, i64)) -> usize {
    let level = &test.world.site(ID).levels[&inn::GROUND];
    level.bottle_racks[&at].items().count()
}

fn take_some(test: &mut TestClient, at: (i64, i64), how_many: usize) {
    let level = test.world.site_mut(ID).levels.get_mut(&inn::GROUND);
    let bottles = level.and_then(|level| level.bottle_racks.get_mut(&at));
    let bottles = bottles.expect("a stocked rack");
    let ids: Vec<_> = bottles.items().map(|item| item.id).take(how_many).collect();
    for id in ids {
        bottles.take(id);
    }
}

#[test]
fn racks_refill_themselves() {
    let mut test = in_the_inn();
    let at = pieces(|f| matches!(f, Furniture::BottleRack { .. }))[0].at;
    let full = on_rack(&mut test, at);
    take_some(&mut test, at, 3);
    let start = Instant::now();
    restock_inns(&mut test.world, start);
    assert_eq!(
        on_rack(&mut test, at),
        full,
        "the first look refills at once"
    );
    assert!(test.world.bench_changes.contains(&(ID, inn::GROUND, at)));
    take_some(&mut test, at, 1);
    restock_inns(&mut test.world, start + Duration::from_secs(1));
    assert_eq!(on_rack(&mut test, at), full - 1, "not again so soon");
    restock_inns(
        &mut test.world,
        start + Duration::from_secs(RESTOCK_SECONDS),
    );
    assert_eq!(on_rack(&mut test, at), full);
}

#[test]
fn the_foyers_chests_are_only_to_look_at() {
    let chests = pieces(|f| matches!(f, Furniture::Chest { .. }));
    assert!(
        chests
            .iter()
            .all(|piece| inn::look_only(inn::GROUND, piece))
    );
    let own = inn::level(inn::UPSTAIRS).expect("upstairs").level;
    let tier_one = inn::own_rooms()[0];
    let mine = (own.furniture()).find(|p| tier_one.rect.contains(p.at.0, p.at.1));
    let mine = mine.filter(|p| matches!(p.furniture, Furniture::Chest { .. }));
    assert!(!inn::look_only(
        inn::UPSTAIRS,
        mine.expect("an own room's chest")
    ));
}
