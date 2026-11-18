//! Vegetables through a connected client: Give Item's vegetable, eating one
//! raw, cutting a whole one into servings, and raw harm.

use shared::bonus::Source;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::protocol::ClientMessage;
use shared::vegetable::VegetableTag;

use super::tests::TestClient;

fn give(key: &str, count: u32) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind: ItemKind::Vegetable,
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
    let items = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    items
        .filter(|item| item.kind == ItemKind::Vegetable)
        .collect()
}

fn has(test: &TestClient, source: Source) -> bool {
    test.client
        .sheet
        .effects
        .0
        .iter()
        .any(|effect| effect.source == source)
}

fn eat_first(test: &mut TestClient) -> Id {
    let item: Id = carried(test)[0].id;
    test.send(&[ClientMessage::Consume { item }]);
    item
}

#[test]
fn eating_a_vegetable_raw_heals_and_one_goes() {
    let mut test = TestClient::new();
    test.send(&[give("carrot", 2)]);
    let found = carried(&test)[0]
        .vegetable()
        .map(|form| form.vegetable.clone());
    assert_eq!(found.as_deref(), Some("carrot"));
    let stack = eat_first(&mut test);
    assert_eq!(test.client.inventory.find(stack).map_or(0, Item::count), 1);
    let tag = VegetableTag::of("carrot");
    assert!(has(&test, Source::EatenVegetable(tag)));
    assert!(
        has(&test, Source::VegetableEffects(tag)),
        "Perception, raw too"
    );
}

#[test]
fn a_whole_pumpkin_is_cut_and_its_servings_kept() {
    let mut test = TestClient::new();
    test.send(&[give("pumpkin", 1)]);
    let pumpkin = eat_first(&mut test);
    assert!(test.client.inventory.find(pumpkin).is_none(), "cut");
    let left = carried(&test);
    assert_eq!(left.len(), 1);
    assert!(left[0].vegetable().is_some_and(|form| form.serving));
    assert_eq!(left[0].count, 11);
    let tag = VegetableTag::of("pumpkin");
    assert!(!has(&test, Source::EatenVegetable(tag)), "rarely eaten raw");
}

#[test]
fn raw_cassava_only_harms_and_grain_does_nothing() {
    let mut test = TestClient::new();
    test.send(&[give("cassava", 1), give("wheat", 1)]);
    let cassava = carried(&test)
        .into_iter()
        .find(|item| {
            item.vegetable()
                .is_some_and(|form| form.vegetable == "cassava")
        })
        .map(|item| item.id)
        .expect("cassava");
    test.send(&[ClientMessage::Consume { item: cassava }]);
    let tag = VegetableTag::of("cassava");
    assert!(!has(&test, Source::EatenVegetable(tag)));
    assert!(has(&test, Source::VegetableEffects(tag)));
    let wheat = carried(&test)
        .into_iter()
        .find(|item| {
            item.vegetable()
                .is_some_and(|form| form.vegetable == "wheat")
        })
        .map(|item| item.id)
        .expect("wheat");
    test.send(&[ClientMessage::Consume { item: wheat }]);
    assert!(
        test.client.inventory.find(wheat).is_none(),
        "eaten all the same"
    );
    let tag = VegetableTag::of("wheat");
    assert!(!has(&test, Source::EatenVegetable(tag)));
}
