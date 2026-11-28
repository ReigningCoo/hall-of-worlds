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
    assert!(may_go_to(&trade, shop, buying_box()));
    assert!(may_go_to(&trade, buying_box(), shop));
    assert!(may_go_to(&trade, shop, shop));
    assert!(!may_go_to(&trade, shop, own_side()));
    assert!(!may_go_to(&Trading::default(), shop, buying_box()));
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
    assert!(!may_go_to(&trade, Id::for_test(99), buying_box()));
}
