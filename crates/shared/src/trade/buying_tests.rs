use super::*;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Rotation, StackLimits};
use crate::site::BuildingKind;

fn rings(count: u128) -> Vec<Item> {
    (1..=count)
        .map(|n| Item::new(Id::for_test(n), ItemKind::Ring))
        .collect()
}

fn at(x: u8, y: u8) -> Option<BoxPlace> {
    Some(BoxPlace {
        x,
        y,
        rotation: Rotation::None,
    })
}

fn stand_in() -> impl FnMut() -> Id {
    let mut next = 1000;
    move || {
        next += 1;
        Id::for_test(next)
    }
}

#[test]
fn the_box_is_eight_wide_and_four_rows_at_least() {
    let stock = rings(40);
    let mut trade = ShopTrade::new(Id::for_test(99));
    assert_eq!(trade.buying_rows(&stock), BUYING_ROWS);
    trade.buy(Id::for_test(1), at(7, 3), &stock).unwrap();
    assert_eq!(trade.buying_rows(&stock), 4);
    // Past the edge, or onto something, while there is room: refused.
    assert_eq!(
        trade.buy(Id::for_test(2), at(8, 0), &stock),
        Err(Refusal::NoRoom)
    );
    assert_eq!(
        trade.buy(Id::for_test(2), at(0, 4), &stock),
        Err(Refusal::NoRoom)
    );
    assert_eq!(
        trade.buy(Id::for_test(2), at(7, 3), &stock),
        Err(Refusal::NoRoom)
    );
    assert_eq!(
        trade.buy(Id::for_test(99), None, &stock),
        Err(Refusal::NotStocked)
    );
}

#[test]
fn it_grows_a_row_when_something_would_not_fit_and_shrinks_back() {
    let stock = rings(40);
    let mut trade = ShopTrade::new(Id::for_test(99));
    for n in 1..=32 {
        trade.buy(Id::for_test(n), None, &stock).unwrap();
    }
    assert_eq!(trade.buying_rows(&stock), 4);
    let last = trade.buying.last().unwrap();
    assert_eq!(
        (last.x, last.y),
        (7, 3),
        "the first free places, row by row"
    );
    // Full: one more goes below, wherever it was let go.
    trade.buy(Id::for_test(33), at(3, 1), &stock).unwrap();
    let grown = trade.buying.last().unwrap();
    assert_eq!((grown.x, grown.y), (0, 4));
    assert_eq!(trade.buying_rows(&stock), 5);
    trade.unbuy(Id::for_test(33));
    assert_eq!(trade.buying_rows(&stock), 4);
    for n in 1..=32 {
        trade.unbuy(Id::for_test(n));
    }
    assert!(trade.buying.is_empty());
    assert_eq!(trade.buying_rows(&stock), BUYING_ROWS);
}

#[test]
fn moving_within_the_box_buys_it_again_there() {
    let stock = rings(2);
    let mut trade = ShopTrade::new(Id::for_test(99));
    trade.buy(Id::for_test(1), at(0, 0), &stock).unwrap();
    trade.buy(Id::for_test(1), at(5, 2), &stock).unwrap();
    assert_eq!(trade.buying.len(), 1);
    assert_eq!((trade.buying[0].x, trade.buying[0].y), (5, 2));
    // Its old place is free for another.
    trade.buy(Id::for_test(2), at(0, 0), &stock).unwrap();
    assert_eq!(trade.buying.len(), 2);
}

#[test]
fn what_is_bought_goes_where_it_fits_and_the_rest_to_the_receive_box() {
    let stock: Vec<Item> = (1..=200)
        .map(|n| Item::new(Id::for_test(n), ItemKind::Sword))
        .collect();
    let mut trade = ShopTrade::new(Id::for_test(999));
    for n in 1..=200 {
        trade.buy(Id::for_test(n), None, &stock).unwrap();
    }
    let mut carried = Inventory::default();
    let gold = Item {
        count: 100,
        ..Item::new(Id::for_test(500), ItemKind::GoldCoin)
    };
    carried
        .give(gold, StackLimits::default(), &mut stand_in())
        .unwrap();
    let done = complete(
        &trade,
        &stock,
        &carried,
        (Rates::default(), BuildingKind::Blacksmith),
        0,
        StackLimits::default(),
        None,
        &mut stand_in(),
    )
    .unwrap();
    let carried = (1..=200).filter(|&n| done.inventory.find(Id::for_test(n)).is_some());
    let carried = carried.count();
    assert!(carried > 0, "some fit");
    assert!(!done.received.is_empty(), "not all of them fit");
    assert!(done.stock.is_empty());
}
