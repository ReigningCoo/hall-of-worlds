use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Material, Piece, Wood};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::poi::PoiKind;
use shared::protocol::{BenchAsk, BenchNews, ClientMessage, DevBench, SeenBench, ServerMessage};
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;

/// An Alchemy bench along `(6, 10)` and `(7, 10)`, its front to the south.
fn bench() -> Piece {
    Piece {
        at: (6, 10),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    }
}

/// A client in an empty Dev Area with `bench()` made in it holding a Common
/// Mortar and Pestle, standing in front of it facing it.
fn at_a_bench() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    test.send(&[ClientMessage::Bench(BenchAsk::Dev(DevBench {
        piece: bench(),
        tools: vec![(ItemKind::MortarAndPestle, Quality::Common)],
    }))]);
    share(&mut test);
    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    share(&mut test);
    test
}

/// Shares furniture and benches, as the tick does: the benches told.
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

fn give(test: &mut TestClient, id: u128, kind: ItemKind) {
    let mut item = Item::new(Id::for_test(id), kind);
    item.quality = Some(Quality::Rare);
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn kinds(seen: &SeenBench) -> Vec<ItemKind> {
    seen.tools.tools().iter().map(|tool| tool.kind).collect()
}

#[test]
fn a_made_bench_is_kept_with_its_tools_and_told_once_seen() {
    let mut test = at_a_bench();
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    assert!(visit.bench_at(7, 10).is_some(), "on the level");
    assert!(
        visit.benches_sent.contains_key(&(6, 10)),
        "told with its cells"
    );
    let saved = &test.world.site(id).levels[&index];
    assert_eq!(saved.made_benches, vec![bench()]);
    let tools = &saved.benches[&(6, 10)];
    assert!(tools.has(ItemKind::MortarAndPestle));
    // Told once, not every tick.
    assert!(share(&mut test).is_empty());
}

#[test]
fn tools_go_on_swap_and_come_off_and_everything_else_is_refused() {
    let mut test = at_a_bench();
    give(&mut test, 1, ItemKind::Retort);
    let answers = test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (7, 10),
        item: Id::for_test(1),
    })]);
    assert!(
        answers
            .iter()
            .any(|m| matches!(m, ServerMessage::Inventory(_)))
    );
    assert_eq!(test.client.inventory.count_of(ItemKind::Retort), 0);
    let told = share(&mut test);
    let expected = vec![ItemKind::MortarAndPestle, ItemKind::Retort];
    assert_eq!(told.iter().map(kinds).collect::<Vec<_>>(), vec![expected]);

    // A second Mortar and Pestle swaps with the first.
    give(&mut test, 2, ItemKind::MortarAndPestle);
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(2),
    })]);
    let mortar = test.client.inventory.count_of(ItemKind::MortarAndPestle);
    assert_eq!(mortar, 1, "the old one given back");
    let found = test.client.inventory.find(Id::for_test(2));
    assert!(found.is_none(), "the new one on the bench");

    // A coin isn't taken, and nothing says why.
    give(&mut test, 3, ItemKind::CopperCoin);
    let answers = test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(3),
    })]);
    assert!(answers.contains(&ServerMessage::Bench(BenchNews::Refused)));
    assert!(test.client.inventory.find(Id::for_test(3)).is_some());

    test.send(&[ClientMessage::Bench(BenchAsk::Take {
        bench: (6, 10),
        item: Id::for_test(1),
    })]);
    assert_eq!(test.client.inventory.count_of(ItemKind::Retort), 1);
    let told = share(&mut test);
    assert_eq!(
        told.iter().map(kinds).collect::<Vec<_>>(),
        vec![vec![ItemKind::MortarAndPestle]]
    );
}

#[test]
fn a_bench_out_of_reach_is_left_alone() {
    let mut test = at_a_bench();
    test.send(&[ClientMessage::DevTeleport { x: 6, y: 14 }]);
    give(&mut test, 1, ItemKind::Retort);
    let answers = test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(1),
    })]);
    assert!(!answers.contains(&ServerMessage::Bench(BenchNews::Refused)));
    assert_eq!(test.client.inventory.count_of(ItemKind::Retort), 1);
    let tools = test.client.inventory.count_of(ItemKind::MortarAndPestle);
    assert_eq!(tools, 0, "nothing taken off either");
}

#[test]
fn benches_over_walls_stairs_or_furniture_or_of_the_wrong_shape_are_refused() {
    let mut test = at_a_bench();
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    let long = Piece {
        size: (3, 1),
        ..bench()
    };
    let over = Piece {
        at: (7, 10),
        ..bench()
    };
    let in_wall = Piece {
        at: (-1, 10),
        ..bench()
    };
    for piece in [long, over, in_wall] {
        test.send(&[ClientMessage::Bench(BenchAsk::Dev(DevBench {
            piece,
            tools: Vec::new(),
        }))]);
    }
    let saved = &test.world.site(id).levels[&index];
    assert_eq!(saved.made_benches.len(), 1);
}

#[test]
fn a_herbalists_bench_starts_with_a_mortar_and_pestle_once() {
    let mut test = TestClient::new();
    let enter = |seed| ClientMessage::DevEnterSite {
        kind: PoiKind::HerbalistHut,
        seed,
        level: 0,
        ruin: 0,
    };
    let has_bench = |test: &TestClient| {
        let level = &test.client.visit.as_ref().unwrap().level.level;
        (level.furniture()).any(|piece| {
            matches!(
                piece.furniture,
                Furniture::CraftingBench {
                    craft: Craft::Alchemy
                }
            )
        })
    };
    let seed = (1..50).find(|&seed| {
        test.send(&[enter(seed)]);
        has_bench(&test)
    });
    let seed = seed.expect("a herbalist's hut with a bench on its first level");
    let visit = test.client.visit.as_ref().unwrap();
    let (id, index) = (visit.id, visit.index);
    let saved = &test.world.site(id).levels[&index];
    assert!(saved.benches_stocked);
    let mut stocked = saved.benches.iter();
    let (&at, tools) =
        (stocked.find(|(_, tools)| tools.craft == Craft::Alchemy)).expect("its bench stocked");
    assert!(tools.has(ItemKind::MortarAndPestle));
    // Taken off, it isn't stocked again.
    let mortar = tools.tool(ItemKind::MortarAndPestle).unwrap().id;
    let level = test.world.site_mut(id).levels.get_mut(&index).unwrap();
    level.benches.get_mut(&at).unwrap().take(mortar);
    test.send(&[enter(seed)]);
    let saved = &test.world.site(id).levels[&index];
    assert!(!saved.benches[&at].has(ItemKind::MortarAndPestle));
}
