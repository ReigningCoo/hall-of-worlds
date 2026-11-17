//! Fruit through a connected client: Give Item's fruit, eating one, cutting a
//! whole one into servings, and herbs that bear fruit.

use shared::bonus::Source;
use shared::fruit::FruitTag;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::protocol::ClientMessage;

use super::tests::TestClient;

fn give(kind: ItemKind, key: &str, count: u32) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind,
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

fn carried(test: &TestClient, kind: ItemKind) -> Vec<&Item> {
    let items = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    items.filter(|item| item.kind == kind).collect()
}

fn has(test: &TestClient, source: Source) -> bool {
    test.client
        .sheet
        .effects
        .0
        .iter()
        .any(|effect| effect.source == source)
}

#[test]
fn eating_a_fruit_heals_and_one_goes() {
    let mut test = TestClient::new();
    test.send(&[give(ItemKind::Fruit, "apple", 2)]);
    let apples = carried(&test, ItemKind::Fruit);
    assert_eq!(apples[0].fruit().map(|form| &form.fruit[..]), Some("apple"));
    let stack: Id = apples[0].id;
    test.send(&[ClientMessage::Consume { item: stack }]);
    assert_eq!(test.client.inventory.find(stack).map_or(0, Item::count), 1);
    assert!(has(&test, Source::EatenFruit(FruitTag::of("apple"))));
}

#[test]
fn a_whole_fruit_is_cut_and_its_servings_kept() {
    let mut test = TestClient::new();
    test.send(&[give(ItemKind::Fruit, "watermelon", 1)]);
    let melon: Id = carried(&test, ItemKind::Fruit)[0].id;
    test.send(&[ClientMessage::Consume { item: melon }]);
    assert!(test.client.inventory.find(melon).is_none(), "cut");
    let left = carried(&test, ItemKind::Fruit);
    assert_eq!(left.len(), 1);
    assert!(left[0].fruit().is_some_and(|form| form.serving));
    assert_eq!(left[0].count, 10);
    assert!(has(&test, Source::EatenFruit(FruitTag::of("watermelon"))));
}

#[test]
fn harmful_fruit_only_harms() {
    let mut test = TestClient::new();
    test.send(&[give(ItemKind::Fruit, "acorn", 1)]);
    let acorn: Id = carried(&test, ItemKind::Fruit)[0].id;
    test.send(&[ClientMessage::Consume { item: acorn }]);
    let tag = FruitTag::of("acorn");
    assert!(!has(&test, Source::EatenFruit(tag)));
    assert!(has(&test, Source::FruitEffects(tag)));
}

#[test]
fn a_herb_bearing_fruit_gives_its_fruit_too() {
    let mut test = TestClient::new();
    test.send(&[give(ItemKind::Herb, "bilberry", 1)]);
    let herb: Id = carried(&test, ItemKind::Herb)[0].id;
    test.send(&[ClientMessage::Consume { item: herb }]);
    assert!(has(&test, Source::EatenFruit(FruitTag::of("bilberry"))));
    assert_eq!(
        has(&test, Source::RawHerb),
        shared::herb::RAW_DOSE_WITH_FRUIT
    );
}
