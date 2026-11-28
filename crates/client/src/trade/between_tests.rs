//! Trades with players and NPCs: who is listed, how the window hears the
//! server, what its middle says and which parts it shows.

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Inventory, Item, ItemKind, Rotation};
use shared::protocol::{TradeEnd, TradeNews};
use shared::sheet::Feeling;
use shared::spot::Spot;
use shared::trade::{Between, Offer, Received, Side, TradingSetting};
use shared::value::{copper_shown, total};

use super::between::{hear_between, placed};
use super::people::{Seen, people_near};
use super::show::sums;
use super::sides::{Mode, mode};
use super::state::TradeWindow;
use crate::common::{TradeSetting, Trading};
use crate::inventory::Carried;

fn seen(n: u128, kind: CreatureKind, cells: i64, player: bool) -> Seen {
    Seen {
        id: Id::for_test(n),
        kind,
        feeling: Feeling::Neutral,
        player,
        never_trades: false,
        name: format!("Someone {n}"),
        at: Spot::middle_of((cells, 0)),
    }
}

fn offer(item: &Item, x: u8) -> Offer {
    Offer {
        item: item.id,
        x,
        y: 0,
        rotation: Rotation::None,
    }
}

fn between(mine: Vec<Offer>, theirs: Vec<Offer>, locked: (bool, bool)) -> Between {
    Between {
        with: Id::for_test(50),
        mine: Side {
            offers: mine,
            locked: locked.0,
        },
        theirs: Side {
            offers: theirs,
            locked: locked.1,
        },
    }
}

#[test]
fn players_and_people_near_enough_are_listed_nearest_first() {
    let here = Spot::middle_of((0, 0));
    let mut never = seen(3, CreatureKind::Elf, 1, true);
    never.never_trades = true;
    let mut angry = seen(6, CreatureKind::Human, 1, false);
    angry.feeling = Feeling::Unfriendly;
    let all = [
        seen(1, CreatureKind::Human, 4, true),
        seen(2, CreatureKind::Dwarf, 9, true),
        never,
        seen(4, CreatureKind::Human, 2, false),
        seen(5, CreatureKind::Wolf, 1, false),
        angry,
    ];
    let (players, npcs) = people_near(here, &all);
    let ids: Vec<_> = players.iter().map(|one| one.with).collect();
    assert_eq!(ids, [Id::for_test(3), Id::for_test(1)], "too far left out");
    assert!(players[0].never, "set never to trade: greyed");
    let ids: Vec<_> = npcs.iter().map(|one| one.with).collect();
    assert_eq!(
        ids,
        [Id::for_test(4)],
        "animals and the unfriendly left out"
    );
}

#[test]
fn beggars_come_first_among_npcs() {
    let here = Spot::middle_of((0, 0));
    let all = [
        seen(1, CreatureKind::Human, 1, false),
        seen(2, CreatureKind::Beggar, 4, false),
        seen(3, CreatureKind::Dwarf, 2, false),
        seen(4, CreatureKind::Beggar, 3, false),
    ];
    let (_, npcs) = people_near(here, &all);
    let ids: Vec<_> = npcs.iter().map(|one| one.with).collect();
    let order = [4, 2, 1, 3].map(Id::for_test);
    assert_eq!(ids, order, "beggars nearest first, then the rest");
}

#[test]
fn npcs_alike_are_numbered() {
    let here = Spot::middle_of((0, 0));
    let mut one = seen(1, CreatureKind::Human, 1, false);
    let mut two = seen(2, CreatureKind::Human, 2, false);
    one.name = "Human".to_owned();
    two.name = "Human".to_owned();
    let (_, npcs) = people_near(here, &[one, two]);
    let names: Vec<_> = npcs.iter().map(|one| one.name.as_str()).collect();
    assert_eq!(names, ["Human", "Human 2"]);
}

#[test]
fn asking_then_opening_and_the_box_and_setting_are_heard() {
    let mut window = TradeWindow::default();
    let mut trading = Trading::default();
    let mut setting = TradeSetting::default();
    let with = Id::for_test(50);
    let mut hear = |news: TradeNews, window: &mut TradeWindow, trading: &mut Trading| {
        assert!(hear_between(&news, window, trading, &mut setting));
    };
    hear(TradeNews::Asking { with }, &mut window, &mut trading);
    assert!(trading.asking);
    assert_eq!(mode(&window, &trading), Mode::Between);
    let sword = Item::new(Id::for_test(7), ItemKind::Sword);
    let news = TradeNews::Between(between(vec![offer(&sword, 1)], vec![], (true, false)));
    hear(news, &mut window, &mut trading);
    assert!(!trading.asking);
    assert_eq!(trading.with, Some(with));
    assert_eq!(
        trading.selling,
        [sword.id],
        "marked with a bag in the inventory"
    );
    assert!(trading.is_busy());
    let received = Received {
        items: vec![sword.clone()],
    };
    hear(TradeNews::Received(received), &mut window, &mut trading);
    assert_eq!(mode(&window, &trading), Mode::Receiving);
    let ended = TradeNews::Ended {
        with,
        why: TradeEnd::Exchanged,
    };
    assert!(!hear_between(
        &ended,
        &mut window,
        &mut trading,
        &mut setting
    ));
    hear_between(
        &TradeNews::Setting(TradingSetting::Never),
        &mut window,
        &mut trading,
        &mut setting,
    );
    assert_eq!(setting.0, TradingSetting::Never);
}

#[test]
fn being_asked_waits_for_an_answer() {
    let mut window = TradeWindow::default();
    let by = Id::for_test(9);
    let news = TradeNews::Asked {
        by,
        name: "Mara".to_owned(),
    };
    hear_between(
        &news,
        &mut window,
        &mut Trading::default(),
        &mut TradeSetting::default(),
    );
    assert_eq!(window.asked, Some((by, "Mara".to_owned())));
}

#[test]
fn each_side_is_placed_where_it_was_put_and_the_middle_says_who_gains() {
    let mine = Item::new(Id::for_test(1), ItemKind::Sword);
    let theirs = Item::new(Id::for_test(2), ItemKind::Sword);
    let deal = between(
        vec![offer(&mine, 2)],
        vec![offer(&theirs, 4)],
        (false, true),
    );
    let at = placed(&deal.theirs, std::slice::from_ref(&theirs));
    assert_eq!((at[0].x, at[0].item.id), (4, theirs.id));
    let carried = Inventory::default();
    let trading = Trading {
        with: Some(deal.with),
        between: Some(deal),
        ..Trading::default()
    };
    // What is carried isn't there: only theirs counts.
    let said = sums(&trading, &Carried(carried), std::slice::from_ref(&theirs));
    assert_eq!(said.own, 0);
    assert_eq!(said.theirs, copper_shown(total(&theirs)));
    assert!(said.theirs > 0);
    assert_eq!(said.difference, None, "no coins change hands");
}
