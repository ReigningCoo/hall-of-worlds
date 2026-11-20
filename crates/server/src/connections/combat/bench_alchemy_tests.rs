//! Alchemy at a crafting bench: its Mortar and Pestle is used when none is
//! carried, only while the player is at the bench, facing it.

use shared::alchemy::{AlchemyOrder, OrderStep};
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Material, Piece, Wood};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid, Oil};
use shared::protocol::{BenchAsk, ClientMessage, DevBench};
use shared::site::{DevArea, DevLayout, DevMobs};

use super::tests::run;
use crate::connections::tests::TestClient;

/// Ticks a Mortar step takes untrained, and a few more.
const GRINDING_TICKS: u64 = 10 * 20 + 2;

/// In an empty Dev Area before a bench along `(6, 10)` and `(7, 10)` holding
/// a Mortar and Pestle, facing it, carrying a herb and a bottle of oil but no
/// apparatus; the order grinding them at the bench.
fn at_a_bench() -> (TestClient, AlchemyOrder) {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    let piece = Piece {
        at: (6, 10),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    };
    test.send(&[ClientMessage::Bench(BenchAsk::Dev(DevBench {
        piece,
        tools: vec![(ItemKind::MortarAndPestle, Quality::Common)],
    }))]);
    // Shared at the end of the tick.
    crate::sites::share_furniture(std::slice::from_mut(&mut test.client), &mut test.world);
    test.send(&[
        ClientMessage::DevTeleport { x: 7, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    let new_id = &mut shared::id::stand_ins();
    let def = test.world.herbs.iter().next().unwrap().clone();
    let herb = shared::herb::new_item(&def, 3, Id::for_test(91), None);
    let mut oil = Item::new(Id::for_test(92), ItemKind::Bottle);
    let liquid = Some(Liquid::Oil(Oil::Nut));
    oil.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 1)));
    for item in [herb, oil] {
        let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
        assert!(given.is_ok());
    }
    let order = AlchemyOrder {
        bench: Some((6, 10)),
        source: None,
        steps: vec![OrderStep::Mortar {
            herbs: vec![Id::for_test(91)],
            solvent: Some(Id::for_test(92)),
        }],
        count: 1,
    };
    (test, order)
}

#[test]
fn the_benchs_mortar_grinds_when_none_is_carried() {
    let (mut test, order) = at_a_bench();
    test.send(&[ClientMessage::Alchemy(order)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, GRINDING_TICKS, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Herb), 2);
}

#[test]
fn nothing_begins_away_from_the_bench_or_without_asking_for_it() {
    let (mut test, order) = at_a_bench();
    let carried_only = AlchemyOrder {
        bench: None,
        ..order.clone()
    };
    test.send(&[ClientMessage::Alchemy(carried_only)]);
    assert!(test.client.fight.pending.is_none(), "no apparatus carried");
    test.send(&[ClientMessage::Face(Heading::SOUTH)]);
    test.send(&[ClientMessage::Alchemy(order)]);
    assert!(test.client.fight.pending.is_none(), "turned away from it");
}
