use shared::bottle_rack::{BottleRack, BottleRackKind};
use shared::door::Side;
use shared::facing::Heading;
use shared::fermentation::{Base, Drink, Sweetness};
use shared::furniture::{Furniture, Held, Material, Piece, TableStyle, Wood};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid};
use shared::protocol::{
    BenchAsk, BenchNews, ClientMessage, DevFurniture, SeenBench, ServerMessage,
};
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;

fn oak(at: (i64, i64), size: (u8, u8), facing: Side, furniture: Furniture) -> Piece {
    Piece {
        at,
        size,
        facing,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

/// A table along `(6, 10)` to `(8, 10)`.
fn table() -> Piece {
    let style = TableStyle::Trestle;
    oak((6, 10), (3, 1), Side::East, Furniture::Table { style })
}

/// A spirit rack on `(6, 10)` and `(7, 10)`, its front to the south.
fn spirit_rack() -> Piece {
    let rack = BottleRack {
        kind: BottleRackKind::Spirit,
        length: 2,
    };
    oak((6, 10), (2, 1), Side::South, Furniture::BottleRack { rack })
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

fn ale(n: u128) -> Item {
    let drink = Drink {
        base: Base::Grain("barley".into()),
        strength: 50,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    };
    let form = BottleForm::new(Some(Liquid::Drink(Box::new(drink))), 0).with_seal(true);
    Item {
        form: Some(ItemForm::Bottle(form)),
        ..Item::new(Id::for_test(n), ItemKind::Bottle)
    }
}

fn bottles_told(told: &[SeenBench]) -> Vec<usize> {
    let count = |seen: &SeenBench| match &seen.held {
        Held::Bottles(bottles) => bottles.items().count(),
        _ => usize::MAX,
    };
    told.iter().map(count).collect()
}

#[test]
fn a_spirit_rack_stands_on_a_table_and_takes_one_bottle_of_a_stack() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    // Nothing to stand on: refused.
    test.send(&[ClientMessage::DevFurniture(DevFurniture {
        piece: spirit_rack(),
    })]);
    share(&mut test);
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    assert!(visit.level.level.furniture_on_top_at(6, 10).is_none());

    test.send(&[ClientMessage::DevFurniture(DevFurniture { piece: table() })]);
    share(&mut test);
    test.send(&[ClientMessage::DevFurniture(DevFurniture {
        piece: spirit_rack(),
    })]);
    let told = share(&mut test);
    assert_eq!(bottles_told(&told), vec![0]);
    let visit = test.client.visit.as_ref().unwrap();
    assert!(
        visit
            .level
            .level
            .furniture_at(7, 10)
            .unwrap()
            .furniture
            .is_table()
    );
    assert_eq!(
        visit.level.level.furniture_on_top_at(7, 10),
        Some(&spirit_rack())
    );
    assert_eq!(
        test.world.site(id).levels[&index].made_benches,
        vec![spirit_rack()]
    );

    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);
    let stack = Item { count: 2, ..ale(1) };
    let new_id = &mut shared::id::stand_ins();
    let given = test
        .client
        .inventory
        .give(stack, StackLimits::default(), new_id);
    assert!(given.is_ok());
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (7, 10),
        item: Id::for_test(1),
    })]);
    assert_eq!(bottles_told(&share(&mut test)), vec![1]);
    // One went; the other is still carried.
    let left = test.client.inventory.find(Id::for_test(1));
    assert_eq!(left.map(Item::count), Some(1));
    let bottles = &test.world.site(id).levels[&index].bottle_racks[&(6, 10)];
    assert_eq!(bottles.items().map(Item::count).sum::<u32>(), 1);
    let glass = bottles
        .items()
        .next()
        .and_then(Item::bottle)
        .map(|b| b.glass.lying_since);
    assert!(matches!(glass, Some(Some(_))), "on world time");
}
