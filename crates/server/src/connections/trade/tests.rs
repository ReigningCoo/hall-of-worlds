use shared::id::Id;
use shared::item::{Item, Rotation};
use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, ServerMessage, TradeAsk, TradeEnd, TradeNews};
use shared::shop::SOLD_KEPT_FOR;
use shared::trade::{BoxPlace, Payment, Refusal};

use crate::connections::tests::TestClient;
use crate::ids::new_id;

/// A village, a shop in it with the cell before its door, and a player
/// standing there with 100 gold and room.
fn at_a_shop() -> (TestClient, Id, (i64, i64)) {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevEnterSite {
        kind: PoiKind::Village,
        seed: 4,
        level: 0,
        ruin: 0,
    }]);
    let visit = test.client.visit.as_mut().unwrap();
    let (keeper, front) = visit.first_shop_for_test(&mut test.world);
    visit.move_for_test(front);
    test.client.inventory.items.clear();
    let limits = test.world.stacks;
    (test.client.inventory)
        .receive_coins(1_000_000, limits, None, &mut new_id)
        .unwrap();
    (test, keeper, front)
}

fn stock_in(answers: &[ServerMessage]) -> Option<Vec<Item>> {
    answers.iter().find_map(|answer| match answer {
        ServerMessage::TradeStock { items, .. } => Some(items.clone()),
        _ => None,
    })
}

fn news(answers: &[ServerMessage]) -> Vec<TradeNews> {
    let news = answers.iter().filter_map(|answer| match answer {
        ServerMessage::Trade(news) => Some(news.clone()),
        _ => None,
    });
    news.collect()
}

fn ended(answers: &[ServerMessage]) -> Option<TradeEnd> {
    news(answers).into_iter().find_map(|news| match news {
        TradeNews::Ended { why, .. } => Some(why),
        _ => None,
    })
}

fn refused(answers: &[ServerMessage]) -> Option<Refusal> {
    news(answers).into_iter().find_map(|news| match news {
        TradeNews::Refused(why) => Some(why),
        _ => None,
    })
}

/// Buying `item`, to the first free place in the Buying box.
fn buy(item: Id) -> ClientMessage {
    ClientMessage::Trade(TradeAsk::Buy { item, at: None })
}

fn trade(ask: TradeAsk) -> ClientMessage {
    ClientMessage::Trade(ask)
}

#[test]
fn nothing_is_bought_until_the_trade_is_completed() {
    let (mut test, with, _) = at_a_shop();
    let answers = test.send(&[ClientMessage::OpenTrade { with }]);
    let stock = stock_in(&answers).expect("it opens");
    assert!(matches!(news(&answers)[0], TradeNews::Opened { .. }));
    let item = stock[0].id;
    let coins = test.client.inventory.coins_carried();
    test.send(&[buy(item)]);
    assert!(
        test.client.inventory.find(item).is_none(),
        "still the shop's"
    );
    assert_eq!(test.client.trade.shop.as_ref().unwrap().buying.len(), 1);
    let answers = test.send(&[trade(TradeAsk::Complete)]);
    let Some(TradeEnd::Completed { paid, payment }) = ended(&answers) else {
        panic!("completed: {answers:?}");
    };
    assert_eq!(payment, Payment::Exact);
    assert!(paid > 0);
    assert!(test.client.inventory.find(item).is_some());
    assert_eq!(test.client.inventory.coins_carried(), coins - paid as u64);
    let left = stock_in(&answers).expect("the stock comes back");
    assert!(left.iter().all(|one| one.id != item));
    assert!(test.client.trade.shop.is_none());
}

#[test]
fn walking_away_or_closing_moves_nothing() {
    let (mut test, with, front) = at_a_shop();
    let stock = stock_in(&test.send(&[ClientMessage::OpenTrade { with }])).unwrap();
    test.send(&[buy(stock[0].id)]);
    let answers = test.send(&[trade(TradeAsk::Close)]);
    assert_eq!(ended(&answers), Some(TradeEnd::Closed));
    test.send(&[ClientMessage::OpenTrade { with }, buy(stock[0].id)]);
    (test.client.visit.as_mut().unwrap()).move_for_test((front.0 + 40, front.1));
    let answers = test.send(&[]);
    assert_eq!(ended(&answers), Some(TradeEnd::OutOfReach));
    assert!(test.client.trade.shop.is_none());
    assert!(test.client.inventory.find(stock[0].id).is_none());
    // Out of reach, it doesn't open at all.
    let answers = test.send(&[ClientMessage::OpenTrade { with }]);
    assert_eq!(stock_in(&answers), None);
}

#[test]
fn what_is_sold_stays_carried_until_completed_then_joins_the_stock_for_a_while() {
    let (mut test, with, front) = at_a_shop();
    let stone = Item::new(new_id(), shared::item::ItemKind::Stone);
    let limits = test.world.stacks;
    test.client
        .inventory
        .give(stone.clone(), limits, &mut new_id)
        .unwrap();
    // Given to a shop it is refused: shops take things only in a trade.
    test.send(&[ClientMessage::GiveItem {
        item: stone.id,
        to: with,
    }]);
    assert!(test.client.inventory.find(stone.id).is_some());
    test.send(&[ClientMessage::OpenTrade { with }]);
    test.send(&[trade(TradeAsk::Sell { item: stone.id })]);
    assert!(
        test.client.inventory.find(stone.id).is_some(),
        "still theirs"
    );
    let answers = test.send(&[trade(TradeAsk::Complete)]);
    assert!(matches!(ended(&answers), Some(TradeEnd::Completed { .. })));
    assert!(test.client.inventory.find(stone.id).is_none());
    let stock = stock_in(&answers).unwrap();
    assert!(stock.iter().any(|one| one.id == stone.id));
    // Past `SOLD_KEPT_FOR` it has gone, sooner than the shop's own.
    let later = test.world.minutes() + SOLD_KEPT_FOR as f64 + 60.0;
    test.send(&[ClientMessage::DevSetWorldClock {
        minutes: Some(later),
        rate: None,
    }]);
    test.client.visit.as_mut().unwrap().move_for_test(front);
    let stock = stock_in(&test.send(&[ClientMessage::OpenTrade { with }])).unwrap();
    assert!(stock.iter().all(|one| one.id != stone.id));
}

#[test]
fn coins_and_haggles_past_the_skill_are_refused() {
    let (mut test, with, _) = at_a_shop();
    let stock = stock_in(&test.send(&[ClientMessage::OpenTrade { with }])).unwrap();
    let coin = test.client.inventory.items[0].item.id;
    let answers = test.send(&[trade(TradeAsk::Sell { item: coin })]);
    assert_eq!(refused(&answers), Some(Refusal::NotForSale));
    test.send(&[buy(stock[0].id), trade(TradeAsk::Pay(Some(0)))]);
    let answers = test.send(&[trade(TradeAsk::Complete)]);
    assert_eq!(refused(&answers), Some(Refusal::TooShort));
    assert!(test.client.trade.shop.is_some(), "still open");
    assert!(test.client.inventory.find(stock[0].id).is_none());
}

#[test]
fn a_tip_raises_standing_and_a_big_one_unpractises_trading() {
    let (mut test, with, _) = at_a_shop();
    let answers = test.send(&[ClientMessage::OpenTrade { with }]);
    let stock = stock_in(&answers).unwrap();
    let Some(TradeNews::Opened { rates, shop, .. }) = news(&answers).first().cloned() else {
        panic!("opened: {answers:?}");
    };
    test.send(&[buy(stock[0].id)]);
    let open = test.client.trade.shop.clone().unwrap();
    let totals = open
        .totals(&stock, &test.client.inventory, rates, shop)
        .unwrap();
    let paying = totals.due + 5000;
    let answers = test.send(&[trade(TradeAsk::Pay(Some(paying)))]);
    assert_eq!(refused(&answers), None);
    let answers = test.send(&[trade(TradeAsk::Complete)]);
    let Some(TradeEnd::Completed { payment, paid }) = ended(&answers) else {
        panic!("completed: {answers:?}");
    };
    assert_eq!(paid, paying);
    assert!(matches!(payment, Payment::Tip { generous: true, .. }));
    let standing = test
        .client
        .character
        .quests
        .standing
        .values()
        .copied()
        .max();
    assert!(standing.unwrap_or(0) >= 5, "5 points for 5,000 copper");
}

#[test]
fn nothing_bought_that_cant_be_paid_for() {
    let (mut test, with, _) = at_a_shop();
    let stock = stock_in(&test.send(&[ClientMessage::OpenTrade { with }])).unwrap();
    test.client.inventory.items.clear();
    let answers = test.send(&[buy(stock[0].id)]);
    assert_eq!(refused(&answers), Some(Refusal::CantAfford));
    assert!(test.client.trade.shop.as_ref().unwrap().buying.is_empty());
}

#[test]
fn bought_things_lie_in_the_buying_box_where_put() {
    let (mut test, with, _) = at_a_shop();
    let stock = stock_in(&test.send(&[ClientMessage::OpenTrade { with }])).unwrap();
    let (first, second) = (stock[0].id, stock[1].id);
    let at = |x, y| {
        Some(BoxPlace {
            x,
            y,
            rotation: Rotation::None,
        })
    };
    test.send(&[trade(TradeAsk::Buy {
        item: first,
        at: at(0, 3),
    })]);
    // Onto it while there is room elsewhere: refused.
    let answers = test.send(&[trade(TradeAsk::Buy {
        item: second,
        at: at(0, 3),
    })]);
    assert_eq!(refused(&answers), Some(Refusal::NoRoom));
    test.send(&[buy(second)]);
    let open = test.client.trade.shop.clone().unwrap();
    assert_eq!(open.buying.len(), 2);
    assert_eq!((open.buying[0].x, open.buying[0].y), (0, 3));
    assert_eq!((open.buying[1].x, open.buying[1].y), (0, 0));
    // Moved within the box, then put back.
    test.send(&[trade(TradeAsk::Buy {
        item: first,
        at: at(4, 0),
    })]);
    let open = test.client.trade.shop.clone().unwrap();
    assert_eq!((open.buying[1].x, open.buying[1].y), (4, 0));
    test.send(&[trade(TradeAsk::Unbuy { item: first })]);
    assert_eq!(test.client.trade.shop.as_ref().unwrap().buying.len(), 1);
    assert!(test.client.inventory.find(first).is_none());
}
