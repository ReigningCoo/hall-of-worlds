//! Herbs through a connected client: Give Item's herb, eating one raw, and the
//! Herbs panel's learning and forgetting.

use shared::bonus::Source;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::protocol::{ClientMessage, DevHerbs};

use super::tests::TestClient;

fn give_herb(herb: Option<&str>, count: u32) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind: ItemKind::Herb,
        size: None,
        magical: false,
        quality: Quality::Rare,
        set: false,
        favour: None,
        seed: Some(3),
        count,
        identical: true,
        request: shared::item_form::RollRequest::NONE,
        herb: herb.map(str::to_owned),
        to: None,
    })
}

fn carried_herb(test: &TestClient) -> Option<&Item> {
    let items = test.client.inventory.items.iter();
    items
        .map(|placed| &placed.item)
        .find(|item| item.kind == ItemKind::Herb)
}

#[test]
fn give_item_gives_the_herb_asked_for_or_one_from_the_seed() {
    let mut test = TestClient::new();
    test.send(&[give_herb(Some("roseroot"), 3)]);
    let herb = carried_herb(&test).expect("a herb");
    assert_eq!(herb.herb().map(|form| &form.herb[..]), Some("roseroot"));
    assert_eq!((herb.count(), herb.quality), (3, Some(Quality::Rare)));
    let mut test = TestClient::new();
    test.send(&[give_herb(None, 1)]);
    assert!(carried_herb(&test).and_then(Item::herb).is_some(), "rolled");
}

#[test]
fn eating_a_herb_raw_teaches_its_first_effect_and_doses_it() {
    let mut test = TestClient::new();
    test.send(&[give_herb(Some("roseroot"), 2)]);
    let stack: Id = carried_herb(&test).expect("herbs").id;
    test.send(&[ClientMessage::Consume { item: stack }]);
    let left = test.client.inventory.find(stack).map_or(0, Item::count);
    assert_eq!(left, 1, "only one is eaten");
    let known = &test.client.sheet.herbs;
    assert!(known.knows_effect("roseroot", 0));
    assert!(!known.knows_effect("roseroot", 1) && !known.knows_name("roseroot"));
    let effects = &test.client.sheet.effects.0;
    assert!(
        effects
            .iter()
            .any(|effect| effect.source == Source::RawHerb)
    );
}

#[test]
fn the_herbs_panel_learns_and_forgets() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevHerbs(DevHerbs::Learn("roseroot".into()))]);
    let known = &test.client.sheet.herbs;
    assert!(known.knows_name("roseroot") && known.knows_effect("roseroot", 3));
    assert!(!known.knows_name("labrador_tea"));
    test.send(&[ClientMessage::DevHerbs(DevHerbs::LearnAll)]);
    assert!(test.client.sheet.herbs.knows_name("labrador_tea"));
    test.send(&[ClientMessage::DevHerbs(DevHerbs::ForgetAll)]);
    assert!(test.client.sheet.herbs.is_empty());
}
