use super::*;
use crate::common::Trading;
use shared::id::Id;
use shared::site::BuildingKind;
use shared::trade::{Between, Rates, Side};

fn with_shop() -> Trading {
    Trading {
        with: Some(Id::for_test(99)),
        terms: Some((Rates::default(), BuildingKind::Blacksmith)),
        ..Trading::default()
    }
}

#[test]
fn a_shops_stock_goes_to_the_buying_box_and_back_only() {
    let trade = with_shop();
    let shop = Id::for_test(99);
    let item = Id::for_test(5);
    assert!(may_go_to(&trade, item, (shop, buying_box())));
    assert!(may_go_to(&trade, item, (buying_box(), shop)));
    assert!(may_go_to(&trade, item, (shop, shop)));
    assert!(!may_go_to(&trade, item, (shop, own_side())));
    assert!(!may_go_to(&Trading::default(), item, (shop, buying_box())));
    // Never into the inventory.
    assert!(is_shops(&trade, shop));
    assert!(is_shops(&trade, buying_box()));
    assert!(!is_shops(&trade, own_side()));
}

#[test]
fn a_characters_side_is_not_a_shop() {
    let trade = Trading {
        between: Some(Between {
            with: Id::for_test(99),
            mine: Side::default(),
            theirs: Side::default(),
        }),
        ..with_shop()
    };
    assert!(!is_shops(&trade, Id::for_test(99)));
    let item = Id::for_test(5);
    assert!(!may_go_to(&trade, item, (Id::for_test(99), buying_box())));
}

#[test]
fn stock_being_bought_goes_into_the_buying_box_only_once() {
    let mut trade = with_shop();
    let shop = Id::for_test(99);
    let item = shared::item::Item::new(Id::for_test(5), shared::item::ItemKind::Fruit);
    let offer = shared::trade::Offer {
        item: item.id,
        x: 0,
        y: 0,
        rotation: shared::item::Rotation::None,
    };
    trade.buying.push((offer, item.clone()));
    // Its copy in the stock stays there; the one in the box moves about.
    assert!(!may_go_to(&trade, item.id, (shop, buying_box())));
    assert!(may_go_to(&trade, item.id, (buying_box(), buying_box())));
    assert!(may_go_to(&trade, item.id, (buying_box(), shop)));
}
