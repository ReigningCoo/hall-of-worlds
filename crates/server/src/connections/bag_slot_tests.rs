use shared::id::Id;
use shared::item::{Holder, ItemKind, Quality, Rotation};
use shared::protocol::ClientMessage;

use super::tests::TestClient;

fn give(kind: ItemKind) -> ClientMessage {
    ClientMessage::DevGiveItem(shared::protocol::DevGive {
        kind,
        size: None,
        magical: false,
        quality: Quality::Rare,
        set: false,
        favour: None,
        seed: Some(1),
        count: 1,
        identical: true,
        request: shared::item_form::RollRequest::NONE,
        to: None,
    })
}

/// The id of something of this kind in the own grid.
fn carried(test: &TestClient, kind: ItemKind) -> Id {
    let items = &test.client.inventory.items;
    let found = items.iter().find(|placed| placed.item.kind == kind);
    found.expect("carried").item.id
}

#[test]
fn bags_are_tied_to_a_worn_belt_by_the_rules_and_untied_with_a_move() {
    let mut test = TestClient::new();
    test.send(&[give(ItemKind::Pouch), give(ItemKind::Backpack)]);
    let (pouch, backpack) = (
        carried(&test, ItemKind::Pouch),
        carried(&test, ItemKind::Backpack),
    );

    // No belt yet, and a backpack never.
    test.send(&[ClientMessage::TieBag {
        item: pouch,
        slot: 0,
    }]);
    assert!(!test.client.inventory.is_tied(pouch));

    test.send(&[give(ItemKind::Belt)]);
    let belt = carried(&test, ItemKind::Belt);
    test.send(&[ClientMessage::WearItem { item: belt }]);
    assert_eq!(test.client.inventory.bag_slot_count(), 3, "a Rare belt");
    test.send(&[ClientMessage::TieBag {
        item: backpack,
        slot: 0,
    }]);
    assert!(!test.client.inventory.is_tied(backpack));

    let cells = test.client.inventory.grid_cells();
    test.send(&[ClientMessage::TieBag {
        item: pouch,
        slot: 0,
    }]);
    assert!(test.client.inventory.is_tied(pouch));
    assert_eq!(test.client.inventory.grid_cells(), cells + 16);
    assert!(test.client.unsaved);

    // Back into the grid, where it would lie once untied.
    let mut untied = test.client.inventory.clone();
    let bag = untied.bag_slots.pop().expect("tied");
    let (x, y, rotation) = untied
        .first_place_for_new(&bag, Holder::Inventory, Rotation::None)
        .expect("room");
    test.send(&[ClientMessage::MoveItem {
        item: pouch,
        to: Holder::Inventory,
        x,
        y,
        rotation,
    }]);
    assert!(!test.client.inventory.is_tied(pouch));
    assert_eq!(test.client.inventory.grid_cells(), cells);
}
