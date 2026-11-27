use super::*;
use crate::common::Trading;
use shared::item::{Holder, Item, ItemKind, Rotation};
use shared::site::BuildingKind;
use shared::trade::Rates;

fn ring(n: u128) -> Item {
    Item::new(Id::for_test(n), ItemKind::Ring)
}

fn buying(item: &Item, x: u8) -> (Buy, Item) {
    let buy = Buy {
        item: item.id,
        to: Holder::Inventory,
        x,
        y: 0,
        rotation: Rotation::None,
    };
    (buy, item.clone())
}

fn trading(buys: Vec<(Buy, Item)>) -> Trading {
    Trading {
        with: Some(Id::for_test(99)),
        terms: Some((Rates::default(), BuildingKind::Blacksmith)),
        buying: buys,
        ..Trading::default()
    }
}

#[test]
fn what_is_being_bought_takes_its_room_but_what_is_held() {
    let carried = Inventory::default();
    let bought = ring(1);
    let trade = trading(vec![buying(&bought, 0)]);
    assert!(with_unpaid(&carried, &Trading::default(), bought.id).is_none());
    let view = with_unpaid(&carried, &trade, Id::for_test(5)).expect("something bought");
    assert!(view.find(bought.id).is_some());
    // Held, it is moving: its old place is free.
    let view = with_unpaid(&carried, &trade, bought.id).expect("something bought");
    assert!(view.find(bought.id).is_none());
}

#[test]
fn nothing_goes_onto_something_unpaid_or_past_the_coins() {
    let bought = ring(1);
    let trade = trading(vec![buying(&bought, 0)]);
    let onto = Target::Stack {
        onto: bought.id,
        allowed: true,
    };
    let refused = Target::Stack {
        onto: bought.id,
        allowed: false,
    };
    assert_eq!(trade_aim(onto, &trade, None, 0), refused);
    let cells = Target::Cells {
        holder: Holder::Inventory,
        x: 2,
        y: 0,
        allowed: true,
    };
    let another = ring(2);
    // No coins: refused; plenty: allowed; carried things aren't bought.
    let poor = trade_aim(cells, &trade, Some(&another), 0);
    assert!(matches!(poor, Target::Cells { allowed: false, .. }));
    assert_eq!(trade_aim(cells, &trade, Some(&another), 1_000_000), cells);
    assert_eq!(trade_aim(cells, &trade, None, 0), cells);
}
