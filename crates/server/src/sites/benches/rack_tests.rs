use shared::cask::{CaskForm, CaskSize};
use shared::cask_rack::CaskRack;
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Held, Material, Piece, Wood};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::protocol::{
    BenchAsk, BenchNews, ClientMessage, DevFurniture, SeenBench, ServerMessage,
};
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;

/// A Firkin rack along `(6, 10)` and `(7, 10)`, its front to the south.
fn rack() -> Piece {
    let rack = CaskRack {
        size: CaskSize::Firkin,
        length: 2,
    };
    Piece {
        at: (6, 10),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CaskRack { rack },
    }
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

fn give_cask(test: &mut TestClient, id: u128, size: CaskSize) {
    let item = Item {
        form: Some(ItemForm::Cask(CaskForm::new(
            size,
            shared::material::Material::Oak,
        ))),
        ..Item::new(Id::for_test(id), ItemKind::Cask)
    };
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn casks_told(told: &[SeenBench]) -> Vec<usize> {
    let count = |seen: &SeenBench| match &seen.held {
        Held::Casks(casks) => casks.items().count(),
        Held::Bench(_) | Held::Bottles(_) => usize::MAX,
    };
    told.iter().map(count).collect()
}

#[test]
fn a_placed_rack_is_kept_and_takes_casks_of_its_size_only() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    test.send(&[ClientMessage::DevFurniture(DevFurniture { piece: rack() })]);
    let told = share(&mut test);
    assert_eq!(
        casks_told(&told),
        vec![0],
        "told empty once its cells are sent"
    );
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    assert_eq!(
        test.world.site(id).levels[&index].made_benches,
        vec![rack()]
    );
    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);

    give_cask(&mut test, 1, CaskSize::Firkin);
    give_cask(&mut test, 2, CaskSize::Pin);
    let put = |test: &mut TestClient, n| {
        test.send(&[ClientMessage::Bench(BenchAsk::Put {
            bench: (7, 10),
            item: Id::for_test(n),
        })])
    };
    put(&mut test, 1);
    assert_eq!(casks_told(&share(&mut test)), vec![1]);
    let answers = put(&mut test, 2);
    assert!(answers.contains(&ServerMessage::Bench(BenchNews::Refused)));
    assert!(test.client.inventory.find(Id::for_test(2)).is_some());

    // On the rack it goes by world time, as on the ground.
    let casks = &test.world.site(id).levels[&index].racks[&(6, 10)];
    let lying = casks
        .items()
        .next()
        .and_then(Item::cask)
        .map(|c| c.time.lying_since);
    assert!(matches!(lying, Some(Some(_))));

    test.send(&[ClientMessage::Bench(BenchAsk::Take {
        bench: (6, 10),
        item: Id::for_test(1),
    })]);
    let back = test.client.inventory.find(Id::for_test(1));
    assert_eq!(
        back.and_then(Item::cask).map(|c| c.time.lying_since),
        Some(None)
    );
    assert_eq!(casks_told(&share(&mut test)), vec![0]);
}
