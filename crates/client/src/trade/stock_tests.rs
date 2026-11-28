//! A shop's stock across a completed trade (final trade tweaks).

use super::cancel::cancelling;
use super::server::{hear, heard_stock};
use super::state::TradeWindow;
use crate::common::Trading;
use crate::inventory::Carried;
use shared::id::Id;
use shared::item::{Item, ItemKind, MadeAt, Rotation};
use shared::protocol::{TradeAsk, TradeEnd, TradeNews};
use shared::site::BuildingKind;
use shared::trade::{Offer, Payment, Rates};

fn shop() -> Id {
    Id::for_test(99)
}

/// Made at this minute: older things are laid out first.
fn sword(n: u128, minute: u64) -> Item {
    let made = MadeAt {
        world: shared::world::WorldId(uuid::Uuid::nil()),
        minute,
    };
    Item {
        made_at: Some(made),
        ..Item::new(Id::for_test(n), ItemKind::Sword)
    }
}

fn trading_with_shop() -> Trading {
    Trading {
        with: Some(shop()),
        terms: Some((Rates::default(), BuildingKind::Blacksmith)),
        ..Trading::default()
    }
}

fn offer(item: Id) -> Offer {
    Offer {
        item,
        x: 0,
        y: 0,
        rotation: Rotation::None,
    }
}

#[test]
fn what_is_sold_shows_in_the_stock_and_nothing_else_moves() {
    let mut window = TradeWindow::default();
    let mut trading = trading_with_shop();
    let (bought, kept, sold) = (sword(1, 50), sword(2, 60), sword(3, 5));
    heard_stock(
        &mut window,
        &trading,
        shop(),
        &[bought.clone(), kept.clone()],
    );
    trading.buying.push((offer(bought.id), bought.clone()));
    trading.selling.push(sold.id);
    let grid = window.theirs.as_ref().expect("the shop's grid");
    let was = grid.place_of(kept.id).expect("in the stock");
    // Completed: the server ends the trade, then says what the shop has now.
    let why = TradeEnd::Completed {
        paid: 0,
        payment: Payment::Exact,
    };
    let ended = TradeNews::Ended { with: shop(), why };
    hear(&ended, &mut window, &mut trading, &Carried::default());
    heard_stock(&mut window, &trading, shop(), &[kept.clone(), sold.clone()]);
    let grid = window.theirs.as_ref().expect("still the shop's grid");
    assert_eq!(grid.place_of(kept.id), Some(was), "nothing moved");
    assert!(grid.place_of(sold.id).is_some(), "what was sold shows");
    assert_eq!(grid.place_of(bought.id), None, "what was bought has gone");
    assert!(grid.marked.is_empty(), "nothing is being bought now");
}

#[test]
fn cancel_takes_everything_out_of_the_trade() {
    let mut trading = trading_with_shop();
    assert!(cancelling(&trading).is_empty(), "nothing to cancel");
    let (bought, sold) = (sword(1, 0), sword(2, 0));
    trading.buying.push((offer(bought.id), bought.clone()));
    trading.selling.push(sold.id);
    let asks = cancelling(&trading);
    assert!(asks.contains(&TradeAsk::Unsell { item: sold.id }));
    assert!(asks.contains(&TradeAsk::Unbuy { item: bought.id }));
    assert_eq!(asks.len(), 2);
}

#[test]
fn cancel_with_a_character_takes_back_the_own_side_and_unlocks_it() {
    let mut mine = shared::trade::Side::default();
    mine.offers.push(offer(Id::for_test(5)));
    mine.locked = true;
    let trading = Trading {
        between: Some(shared::trade::Between {
            with: Id::for_test(7),
            mine,
            theirs: shared::trade::Side::default(),
        }),
        ..trading_with_shop()
    };
    let asks = cancelling(&trading);
    let unsell = TradeAsk::Unsell {
        item: Id::for_test(5),
    };
    assert_eq!(asks, [unsell, TradeAsk::Lock(false)]);
}
