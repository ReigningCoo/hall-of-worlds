use shared::bar::{BarCounter, BarKind};
use shared::cask::{CaskForm, CaskSize};
use shared::cask_rack::CaskRack;
use shared::door::Side;
use shared::facing::Heading;
use shared::fermentation::{Base, Drink, Sweetness};
use shared::furniture::{Furniture, Held, Material, Piece, Wood};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::Liquid;
use shared::protocol::{
    BenchAsk, BenchNews, ClientMessage, DevFurniture, SeenBench, ServerMessage,
};
use shared::site::{DevArea, DevLayout, DevMobs};
use shared::vessel::{VesselForm, VesselKind};

use crate::connections::tests::TestClient;

fn oak(at: (i64, i64), length: u8, furniture: Furniture) -> Piece {
    Piece {
        at,
        size: (length, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

/// A bar counter along `(6, 13)` to `(8, 13)`, served from the south.
fn bar() -> Piece {
    let counter = BarCounter {
        kind: BarKind::Single,
        length: 3,
    };
    oak((6, 13), 3, Furniture::BarCounter { counter })
}

/// A Firkin rack on `(6, 10)`.
fn rack() -> Piece {
    let rack = CaskRack {
        size: CaskSize::Firkin,
        length: 1,
    };
    oak((6, 10), 1, Furniture::CaskRack { rack })
}

/// Shares furniture and pieces with places, as the tick does: what was told.
fn share(test: &mut TestClient) -> Vec<SeenBench> {
    let clients = std::slice::from_mut(&mut test.client);
    crate::sites::share_furniture(clients, &mut test.world);
    crate::sites::share_benches(std::slice::from_mut(&mut test.client), &mut test.world);
    let received = std::iter::from_fn(|| test.end.receive().unwrap());
    let told = received.filter_map(|message| match message {
        ServerMessage::Bench(BenchNews::Tools(benches)) => Some(benches),
        _ => None,
    });
    told.flatten().collect()
}

fn ale() -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Grain("barley".into()),
        strength: 50,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn cask_of_ale(n: u128) -> Item {
    let form = CaskForm::new(CaskSize::Firkin, shared::material::Material::Oak);
    Item {
        form: Some(ItemForm::Cask(form.holding(Some(ale()), 20_000))),
        ..Item::new(Id::for_test(n), ItemKind::Cask)
    }
}

fn tankard(n: u128) -> Item {
    Item {
        form: Some(ItemForm::Vessel(VesselForm::new(VesselKind::Tankard))),
        ..Item::new(Id::for_test(n), ItemKind::Vessel)
    }
}

/// A Dev Area with the bar and the rack, a cask of ale on the rack.
fn tavern() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    for piece in [bar(), rack()] {
        test.send(&[ClientMessage::DevFurniture(DevFurniture::Piece(piece))]);
    }
    share(&mut test);
    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);
    give(&mut test, cask_of_ale(1));
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(1),
    })]);
    share(&mut test);
    test.send(&[
        ClientMessage::DevTeleport { x: 7, y: 14 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);
    test
}

fn tankard_ml(test: &TestClient) -> u32 {
    let found = test.client.inventory.find(Id::for_test(2));
    found.and_then(Item::vessel).map_or(0, |vessel| vessel.ml)
}

#[test]
fn the_bar_pours_from_a_racked_cask_into_a_carried_tankard() {
    let mut test = tavern();
    give(&mut test, tankard(2));
    let pour = ClientMessage::Bench(BenchAsk::Pour {
        bench: (7, 13),
        from: Id::for_test(1),
        into: Id::for_test(2),
    });
    // No tap: refused by the pour rule.
    test.send(std::slice::from_ref(&pour));
    assert_eq!(tankard_ml(&test), 0);
    give(&mut test, Item::new(Id::for_test(3), ItemKind::Tap));
    test.send(&[pour]);
    assert_eq!(tankard_ml(&test), 500);
    let told = share(&mut test);
    let casks = told.iter().find_map(|seen| match &seen.held {
        Held::Casks(casks) => casks.items().next().cloned(),
        _ => None,
    });
    let cask = casks.and_then(|item| item.cask().cloned());
    assert_eq!(cask.map(|cask| cask.ml), Some(19_500), "kept on its rack");
}

#[test]
fn never_from_the_ground_nor_away_from_the_bar() {
    let mut test = tavern();
    give(&mut test, tankard(2));
    give(&mut test, Item::new(Id::for_test(3), ItemKind::Tap));
    give(&mut test, cask_of_ale(4));
    // A carried cask (as one lying on the ground) is not the room's storage.
    test.send(&[ClientMessage::Bench(BenchAsk::Pour {
        bench: (7, 13),
        from: Id::for_test(4),
        into: Id::for_test(2),
    })]);
    assert_eq!(tankard_ml(&test), 0);
    // Out of the bar's reach.
    test.send(&[ClientMessage::DevTeleport { x: 20, y: 20 }]);
    test.send(&[ClientMessage::Bench(BenchAsk::Pour {
        bench: (7, 13),
        from: Id::for_test(1),
        into: Id::for_test(2),
    })]);
    assert_eq!(tankard_ml(&test), 0);
    // At a rack, not a bar.
    test.send(&[ClientMessage::DevTeleport { x: 6, y: 11 }]);
    test.send(&[ClientMessage::Bench(BenchAsk::Pour {
        bench: (6, 10),
        from: Id::for_test(1),
        into: Id::for_test(2),
    })]);
    assert_eq!(tankard_ml(&test), 0);
}

#[test]
fn a_bar_counter_keeps_drinking_vessels() {
    let mut test = tavern();
    give(&mut test, tankard(2));
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (8, 13),
        item: Id::for_test(2),
    })]);
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    let vessels = &test.world.site(id).levels[&index].bars[&(6, 13)];
    assert_eq!(vessels.items().count(), 1);
    assert!(test.client.inventory.find(Id::for_test(2)).is_none());
}
