use super::button::BUTTON;
use super::markup::TRADE;

#[test]
fn the_markup_is_good() {
    BUTTON.assert_good();
    TRADE.assert_good();
    super::markup::ASKED.assert_good();
}

#[test]
fn the_icon_has_ink_on_both_arrows() {
    let icon = super::icon::trade_icon();
    let data = icon.data.expect("drawn");
    let alpha = |x: usize, y: usize| data[(y * 32 + x) * 4 + 3];
    // Each arrow's shaft and head.
    assert_eq!(alpha(10, 10), 255);
    assert_eq!(alpha(22, 10), 255);
    assert_eq!(alpha(21, 22), 255);
    assert_eq!(alpha(9, 22), 255);
    assert_eq!(alpha(16, 16), 0, "a gap between them");
}

use super::markup::{OWN_COLUMNS, STOCK_COLUMNS};
use super::show::middle;
use super::words::{coins_words, due_words, ended_words};
use crate::common::Trading;
use crate::inventory::Carried;
use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Inventory, Item, ItemKind, Rotation};
use shared::protocol::TradeEnd;
use shared::site::BuildingKind;
use shared::trade::{Offer, Payment, Rates, Totals, shop_asks};
use shared::value::total;

fn sword(n: u128) -> Item {
    Item::new(Id::for_test(n), ItemKind::Sword)
}

fn buying(item: &Item) -> (Offer, Item) {
    let buy = Offer {
        item: item.id,
        x: 0,
        y: 0,
        rotation: Rotation::None,
    };
    (buy, item.clone())
}

#[test]
fn the_grids_are_as_wide_as_their_sides() {
    let own = format!(
        "id=\"own\" type=\"item\" places=\"packed\" stacks=\"false\" columns=\"{OWN_COLUMNS}\""
    );
    let stock = format!("columns=\"{STOCK_COLUMNS}\"");
    assert!(TRADE.source.contains(&own));
    assert!(TRADE.source.contains(&stock));
    let buying = format!(
        "id=\"buying\" type=\"item\" places=\"packed\" stacks=\"false\" columns=\"{}\"",
        shared::trade::BUYING_COLUMNS
    );
    assert!(TRADE.source.contains(&buying));
}

#[test]
fn sums_are_written_as_coins_leaving_out_none() {
    assert_eq!(coins_words(0), "nothing");
    assert_eq!(coins_words(5), "5 copper");
    assert_eq!(coins_words(10_005), "1 gold, 5 copper");
    assert_eq!(due_words(0), "Nothing to pay");
    assert_eq!(due_words(102), "You pay 1 silver, 2 copper");
    assert_eq!(due_words(-3), "You receive 3 copper");
}

#[test]
fn a_completed_trade_says_what_moved_and_closing_says_nothing() {
    let paid = TradeEnd::Completed {
        paid: 3,
        payment: Payment::Exact,
    };
    assert_eq!(ended_words(paid).as_deref(), Some("Paid 3 copper."));
    assert_eq!(ended_words(TradeEnd::Closed), None);
    assert!(ended_words(TradeEnd::OutOfReach).is_some());
}

#[test]
fn buying_is_afforded_only_while_the_coins_cover_it() {
    let rates = Rates::default();
    let item = sword(1);
    let price = shop_asks(total(&item), rates);
    assert!(price > 0, "a sword is worth something");
    let mut trading = Trading {
        with: Some(Id::for_test(9)),
        terms: Some((rates, BuildingKind::Blacksmith)),
        ..Trading::default()
    };
    let due = shared::trade::due(price, 0) as u64;
    assert!(trading.affords(&item, due));
    assert!(!trading.affords(&item, due - 1));
    // What is sold pays towards it.
    trading.totals = Totals {
        asks: 0,
        pays: price,
        due: shared::trade::due(0, price),
    };
    assert!(trading.affords(&item, 0));
    // With no terms yet, nothing can be bought.
    assert!(!Trading::default().affords(&item, 1_000_000));
}

#[test]
fn the_middle_says_who_gains() {
    let item = sword(1);
    let rates = Rates::default();
    let asks = shop_asks(total(&item), rates);
    let trading = Trading {
        with: Some(Id::for_test(9)),
        terms: Some((rates, BuildingKind::Blacksmith)),
        buying: vec![buying(&item)],
        totals: Totals {
            asks,
            pays: 0,
            due: shared::trade::due(asks, 0),
        },
        ..Trading::default()
    };
    let words = middle(&trading, &Carried(Inventory::default()), &[]);
    // Buying at 150%: the shop gains.
    assert!(words.gain.starts_with("They gain"), "{words:?}");
    assert!(words.theirs.contains("they ask"), "{words:?}");
    // Nothing in the trade: nothing said of gains.
    let idle = middle(&Trading::default(), &Carried(Inventory::default()), &[]);
    assert!(idle.gain.is_empty());
}

/// Bevy only finds two parameters clashing when a system starts.
#[test]
fn no_system_asks_for_what_another_of_its_parameters_has() {
    fn starts<M>(system: impl IntoSystem<(), (), M>) {
        IntoSystem::into_system(system).initialize(&mut World::new());
    }
    starts(super::open::toggle_trade);
    starts(super::open::notice_closed);
    starts(super::server::read_server);
    starts(super::server::follow_carried);
    starts(super::shops::show_shops);
    starts(super::shops::choose_shop);
    starts(super::grids::note_gifts);
    starts(super::grids::bind_grids);
    starts(super::grids::draw_grids);
    starts(super::grids::show_aim);
    starts(super::show::press_complete);
    starts(super::show::show_middle);
    starts(super::truesight::ask_again_with_truesight);
    starts(super::sides::press_lock);
    starts(super::sides::show_sides);
    starts(super::sides::draw_their_side);
    starts(super::asked::answer_asked);
    starts(super::asked::show_asked);
}
