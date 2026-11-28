use super::*;
use crate::id::Id;
use crate::item::{Holder, Inventory, Item, ItemKind, Rotation, StackLimits};

fn item(kind: ItemKind, id: u128) -> Item {
    Item::new(Id::for_test(id), kind)
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    for item in items {
        inventory
            .give(item, StackLimits::NONE, &mut stand_in())
            .expect("room for it");
    }
    inventory
}

fn stand_in() -> impl FnMut() -> Id {
    let mut next = 1000;
    move || {
        next += 1;
        Id::for_test(next)
    }
}

fn offer(id: u128, x: u8, y: u8) -> Offer {
    Offer {
        item: Id::for_test(id),
        x,
        y,
        rotation: Rotation::None,
    }
}

#[test]
fn offers_fit_in_eight_by_eight_and_never_overlap() {
    let carried = carrying(vec![item(ItemKind::Dagger, 1), item(ItemKind::Dagger, 2)]);
    let shape = carried.find(Id::for_test(1)).unwrap().shape();
    let mut side = Side::default();
    side.offer(offer(1, 0, 0), &carried).unwrap();
    assert_eq!(side.offer(offer(2, 0, 0), &carried), Err(Refusal::NoRoom));
    let past = SIDE_CELLS - shape.width + 1;
    assert_eq!(
        side.offer(offer(2, past, 0), &carried),
        Err(Refusal::NoRoom)
    );
    side.offer(offer(2, shape.width, 0), &carried).unwrap();
    assert_eq!(side.offers.len(), 2);
    // Moved within the side: the same offer, somewhere else.
    side.offer(offer(1, 0, shape.height), &carried).unwrap();
    assert_eq!(side.offers.len(), 2);
}

#[test]
fn any_change_unlocks_the_side() {
    let carried = carrying(vec![item(ItemKind::Dagger, 1)]);
    let mut side = Side {
        locked: true,
        ..Side::default()
    };
    side.offer(offer(1, 0, 0), &carried).unwrap();
    assert!(!side.locked);
    side.locked = true;
    side.take_back(Id::for_test(7));
    assert!(side.locked, "nothing taken back, nothing changed");
    side.take_back(Id::for_test(1));
    assert!(!side.locked && side.offers.is_empty());
}

#[test]
fn quest_items_and_what_isnt_carried_cant_be_offered_but_coins_can() {
    let mut quest = item(ItemKind::Dagger, 1);
    quest.quest = Some(crate::quest::QuestId {
        town: crate::poi::ChunkPos { x: 0, y: 0 },
        slot: crate::quest::Slot::Feeder,
    });
    let coins = Item {
        count: 5,
        ..item(ItemKind::CopperCoin, 2)
    };
    let carried = carrying(vec![quest, coins]);
    let mut side = Side::default();
    assert_eq!(
        side.offer(offer(1, 0, 0), &carried),
        Err(Refusal::NotForSale)
    );
    assert_eq!(
        side.offer(offer(3, 0, 0), &carried),
        Err(Refusal::NotCarried)
    );
    side.offer(offer(2, 0, 0), &carried).unwrap();
}

#[test]
fn something_inside_something_offered_goes_with_it() {
    let mut carried = carrying(vec![item(ItemKind::Bag, 1)]);
    let dagger = item(ItemKind::Dagger, 2);
    let (x, y, rotation) = carried
        .first_place_for_new(&dagger, Holder::Inside(Id::for_test(1)), Rotation::None)
        .expect("room in the bag");
    (carried.add(dagger, Holder::Inside(Id::for_test(1)), x, y, rotation)).unwrap();
    let mut side = Side::default();
    side.offer(offer(1, 0, 0), &carried).unwrap();
    assert_eq!(
        side.offer(offer(2, 4, 4), &carried),
        Err(Refusal::AlreadyIn)
    );
}

#[test]
fn handing_over_is_all_or_nothing() {
    let mut carried = carrying(vec![item(ItemKind::Dagger, 1), item(ItemKind::Dagger, 2)]);
    let mut side = Side::default();
    side.offer(offer(1, 0, 0), &carried).unwrap();
    side.offer(offer(2, 4, 0), &carried).unwrap();
    assert!(side.still_carried(&carried));
    let mut lost = carried.clone();
    lost.hand_over(Id::for_test(2)).unwrap();
    assert!(!side.still_carried(&lost));
    let before = lost.clone();
    assert!(side.hand_over(&mut lost).is_none());
    assert_eq!(lost, before, "nothing taken");
    let items = side.hand_over(&mut carried).unwrap();
    assert_eq!(items.len(), 2);
    assert!(carried.find(Id::for_test(1)).is_none());
}

#[test]
fn the_receive_box_empties_into_what_is_carried() {
    let mut received = Received {
        items: vec![item(ItemKind::Dagger, 1), item(ItemKind::Dagger, 2)],
    };
    let mut carried = Inventory::default();
    let dagger = &received.items[0];
    let (x, y, rotation) = carried
        .first_place_for_new(dagger, Holder::Inventory, Rotation::None)
        .unwrap();
    let to = Place {
        item: Id::for_test(1),
        to: Holder::Inventory,
        x,
        y,
        rotation,
    };
    assert!(received.take(&to, &mut carried));
    assert!(!received.take(&to, &mut carried), "taken already");
    assert_eq!(received.items.len(), 1);
    let taken = received.take_all(&mut carried, StackLimits::NONE, &mut stand_in());
    assert_eq!(taken, 1);
    assert!(received.is_empty());
    assert!(carried.find(Id::for_test(2)).is_some());
}

#[test]
fn trading_is_asked_by_default_and_reach_is_five_cells() {
    assert_eq!(TradingSetting::default(), TradingSetting::Ask);
    let at = crate::spot::Spot::middle_of((10, 10));
    assert!(in_reach(at, crate::spot::Spot::middle_of((15, 10))));
    assert!(!in_reach(at, crate::spot::Spot::middle_of((16, 10))));
}

#[test]
fn the_first_free_place_is_beside_what_is_there() {
    let carried = carrying(vec![item(ItemKind::Dagger, 1), item(ItemKind::Dagger, 2)]);
    let shape = carried.find(Id::for_test(1)).unwrap().shape();
    let mut side = Side::default();
    let first = side.first_free(Id::for_test(1), &carried).unwrap();
    assert_eq!((first.x, first.y), (0, 0));
    side.offer(first, &carried).unwrap();
    let next = side.first_free(Id::for_test(2), &carried).unwrap();
    assert_eq!((next.x, next.y), (shape.width, 0));
    // Nothing not carried has a place.
    assert_eq!(side.first_free(Id::for_test(9), &carried), None);
}
