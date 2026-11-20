//! Fungi through a connected client: Give Item's fungus, eating one raw,
//! cutting a piece into servings, and a poison eaten.

use shared::bonus::Source;
use shared::fungus::FungusTag;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::protocol::ClientMessage;

use super::tests::TestClient;

fn give(key: &str, count: u32) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind: ItemKind::Fungus,
        size: None,
        magical: false,
        quality: Quality::Common,
        set: false,
        favour: None,
        seed: Some(3),
        count,
        identical: true,
        request: shared::item_form::RollRequest::NONE,
        herb: Some(key.to_owned()),
        to: None,
    })
}

fn carried(test: &TestClient) -> Vec<&Item> {
    let items = test.client.inventory.items.iter();
    items
        .map(|placed| &placed.item)
        .filter(|item| item.kind == ItemKind::Fungus)
        .collect()
}

fn has(test: &TestClient, source: Source) -> bool {
    let effects = test.client.sheet.effects.0.iter();
    effects.into_iter().any(|effect| effect.source == source)
}

fn eat_first(test: &mut TestClient) -> Id {
    let item: Id = carried(test)[0].id;
    test.send(&[ClientMessage::Consume { item }]);
    item
}

#[test]
fn eating_a_fungus_raw_heals_and_one_goes() {
    let mut test = TestClient::new();
    test.send(&[give("button_mushroom", 2)]);
    let found = carried(&test)[0].fungus().map(|form| form.fungus.clone());
    assert_eq!(found.as_deref(), Some("button_mushroom"));
    let stack = eat_first(&mut test);
    assert_eq!(test.client.inventory.find(stack).map_or(0, Item::count), 1);
    assert!(has(
        &test,
        Source::EatenFungus(FungusTag::of("button_mushroom"))
    ));
}

#[test]
fn a_giant_puffball_is_cut_and_its_servings_kept() {
    let mut test = TestClient::new();
    test.send(&[give("giant_puffball", 1)]);
    let puffball = eat_first(&mut test);
    assert!(test.client.inventory.find(puffball).is_none(), "cut");
    let left = carried(&test);
    assert_eq!(left.len(), 1);
    assert!(left[0].fungus().is_some_and(|form| form.serving));
    assert_eq!(left[0].count, 9);
}

#[test]
fn a_death_cap_poisons() {
    let mut test = TestClient::new();
    test.send(&[give("death_cap", 1)]);
    eat_first(&mut test);
    let tag = FungusTag::of("death_cap");
    assert!(has(&test, Source::FungusPoison(tag)));
    assert!(!has(&test, Source::EatenFungus(tag)), "never eaten raw");
}
