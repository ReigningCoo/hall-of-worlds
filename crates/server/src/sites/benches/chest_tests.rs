use shared::chest::{Chest, Wardrobe};
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Held, Material, Piece, Wood};
use shared::id::Id;
use shared::item::{Favour, Item, ItemKind, StackLimits};
use shared::protocol::{
    BenchAsk, BenchNews, ClientMessage, DevFurniture, SeenBench, ServerMessage,
};
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;

fn oak(at: (i64, i64), size: (u8, u8), furniture: Furniture) -> Piece {
    Piece {
        at,
        size,
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

/// A chest on `(6, 10)` and `(7, 10)`, and a wardrobe on `(9, 10)`, both
/// facing south.
fn chest() -> Piece {
    let chest = Chest::Large;
    oak((6, 10), (2, 1), Furniture::Chest { chest })
}

fn wardrobe() -> Piece {
    let wardrobe = Wardrobe::Narrow;
    oak((9, 10), (1, 1), Furniture::Wardrobe { wardrobe })
}

/// Shares furniture and pieces with places, as the tick does: what was told,
/// and whether anything was refused.
fn share(test: &mut TestClient) -> (Vec<SeenBench>, bool) {
    let clients = std::slice::from_mut(&mut test.client);
    crate::sites::share_furniture(clients, &mut test.world);
    crate::sites::share_benches(std::slice::from_mut(&mut test.client), &mut test.world);
    let received: Vec<_> = std::iter::from_fn(|| test.end.receive().unwrap()).collect();
    let refused = (received.iter())
        .any(|message| matches!(message, ServerMessage::Bench(BenchNews::Refused)));
    let told = received.into_iter().filter_map(|message| match message {
        ServerMessage::Bench(BenchNews::Tools(benches)) => Some(benches),
        _ => None,
    });
    (told.flatten().collect(), refused)
}

fn put(test: &mut TestClient, bench: (i64, i64), n: u128) -> (Vec<SeenBench>, bool) {
    let answered = test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench,
        item: Id::for_test(n),
    })]);
    let refused = (answered.iter())
        .any(|message| matches!(message, ServerMessage::Bench(BenchNews::Refused)));
    let (told, also) = share(test);
    (told, refused || also)
}

#[test]
fn a_chest_keeps_anything_a_wardrobe_only_clothes() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    test.send(&[
        ClientMessage::DevFurniture(DevFurniture::Piece(chest())),
        ClientMessage::DevFurniture(DevFurniture::Piece(wardrobe())),
        ClientMessage::DevTeleport { x: 8, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    // A cursed ring, not known to be.
    let ring = Item {
        favour: Some(Favour::Cursed),
        ..Item::new(Id::for_test(1), ItemKind::Ring)
    };
    let tunic = Item::new(Id::for_test(2), ItemKind::Tunic);
    let new_id = &mut shared::id::stand_ins();
    for item in [ring, tunic] {
        let given = (test.client.inventory).give(item, StackLimits::default(), new_id);
        assert!(given.is_ok());
    }

    let (_, refused) = put(&mut test, (9, 10), 1);
    assert!(refused, "a ring isn't clothes");
    assert!(test.client.inventory.find(Id::for_test(1)).is_some());

    let (told, refused) = put(&mut test, (7, 10), 1);
    assert!(!refused);
    let Some(Held::Store(stored)) = told.first().map(|seen| &seen.held) else {
        panic!("told of the chest: {told:?}");
    };
    let sent: Vec<_> = stored.items().collect();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].favour, None, "secrets hidden");
    let kept = &test.world.site(id).levels[&index].stores[&(6, 10)];
    assert_eq!(kept.items().next().unwrap().favour, Some(Favour::Cursed));
    assert!(test.client.inventory.find(Id::for_test(1)).is_none());

    let (told, refused) = put(&mut test, (9, 10), 2);
    assert!(!refused, "a tunic goes in a wardrobe");
    assert_eq!(told.len(), 1);
    let wardrobe = &test.world.site(id).levels[&index].stores[&(9, 10)];
    assert_eq!(wardrobe.items().count(), 1);

    test.send(&[ClientMessage::Bench(BenchAsk::Take {
        bench: (6, 10),
        item: Id::for_test(1),
    })]);
    share(&mut test);
    assert!(test.client.inventory.find(Id::for_test(1)).is_some());
    let kept = &test.world.site(id).levels[&index].stores[&(6, 10)];
    assert_eq!(kept.items().count(), 0);
}
