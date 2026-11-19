use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::map_mark::{MOST_HAND_MARKS, MapMark, MarkColour, MarkIcon};
use shared::protocol::{ClientMessage, MarkAsk, MarkNews, ServerMessage};
use shared::wilderness::SCALE;

use super::tests::TestClient;
use crate::storage::{SiteId, SpotMap};

fn add(at: (i64, i64)) -> ClientMessage {
    ClientMessage::Mark(MarkAsk::Add {
        at,
        icon: MarkIcon::Camp,
        colour: MarkColour::Green,
    })
}

fn added(answers: &[ServerMessage]) -> Option<MapMark> {
    answers.iter().find_map(|answer| match answer {
        ServerMessage::Marks(MarkNews::Added(mark)) => Some(*mark),
        _ => None,
    })
}

#[test]
fn a_mark_is_added_round_the_planet_kept_and_removed() {
    let mut test = TestClient::new();
    let width = test.world.width * SCALE;
    let mark = added(&test.send(&[add((-5, 24))])).expect("added");
    assert_eq!(mark.at, (width - 5, 24));
    assert_eq!(
        (mark.icon, mark.colour, mark.auto),
        (MarkIcon::Camp, MarkColour::Green, false)
    );
    assert_eq!(test.client.character.marks, [mark]);
    assert!(test.client.unsaved);

    let answers = test.send(&[ClientMessage::Mark(MarkAsk::Remove(mark.id))]);
    assert!(answers.contains(&ServerMessage::Marks(MarkNews::Removed(mark.id))));
    assert!(test.client.character.marks.is_empty());
    // Someone else's, or one already gone: nothing.
    let again = test.send(&[ClientMessage::Mark(MarkAsk::Remove(mark.id))]);
    assert!(
        !again
            .iter()
            .any(|answer| matches!(answer, ServerMessage::Marks(_)))
    );
}

#[test]
fn marks_past_the_ice_or_the_limit_are_refused() {
    let mut test = TestClient::new();
    let pole = test.world.width / 4 * SCALE;
    assert_eq!(added(&test.send(&[add((0, pole))])), None);
    for n in 0..MOST_HAND_MARKS as i64 {
        assert!(added(&test.send(&[add((n, 0))])).is_some());
    }
    assert_eq!(added(&test.send(&[add((0, 0))])), None);
    assert_eq!(test.client.character.marks.len(), MOST_HAND_MARKS);
}

/// Everything about marks among the answers.
fn mark_news(answers: &[ServerMessage]) -> Vec<MarkNews> {
    let news = answers.iter().filter_map(|answer| match answer {
        ServerMessage::Marks(news) => Some(news.clone()),
        _ => None,
    });
    news.collect()
}

/// A player exploring the wilderness from where they started.
fn in_the_wilderness() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::Down]);
    assert!(test.client.visit.as_ref().unwrap().is_wilderness());
    test
}

fn put_down(test: &mut TestClient, n: u128, kind: ItemKind) {
    let item = Item::new(Id::for_test(n), kind);
    crate::ground::put_down(&mut test.client, &mut test.world, item);
}

/// The wilderness's ground.
fn wild_ground(test: &mut TestClient) -> &mut SpotMap<Vec<Item>> {
    let site = test.world.site_mut(SiteId::Wilderness);
    &mut site.levels.entry(0).or_default().ground
}

#[test]
fn a_new_pile_in_the_wilderness_is_marked_white_and_unmarked_when_emptied() {
    let mut test = in_the_wilderness();
    put_down(&mut test, 1, ItemKind::CopperCoin);
    let pile = test.client.visit.as_ref().unwrap().spot();
    let mark = test.client.character.marks[0];
    assert_eq!((mark.icon, mark.colour), (MarkIcon::Bag, MarkColour::White));
    assert!(mark.auto);
    assert_eq!(mark.pile, Some(pile));
    assert_eq!(mark.at.1, pile.cell().1);
    assert!(mark_news(&test.send(&[])).contains(&MarkNews::Added(mark)));

    // Into the same pile: no second mark.
    put_down(&mut test, 2, ItemKind::Sword);
    assert_eq!(test.client.character.marks.len(), 1);

    // Taking one leaves the pile; taking the last empties it.
    let take = |n| ClientMessage::BinFromGround {
        item: Id::for_test(n),
    };
    assert!(mark_news(&test.send(&[take(1)])).is_empty());
    assert_eq!(test.client.character.marks.len(), 1);
    let news = mark_news(&test.send(&[take(2)]));
    assert_eq!(news, [MarkNews::Removed(mark.id)]);
    assert!(test.client.character.marks.is_empty());
}

#[test]
fn a_log_pile_gets_the_logs_icon_and_sites_get_none() {
    let mut test = in_the_wilderness();
    put_down(&mut test, 1, ItemKind::Log);
    assert_eq!(test.client.character.marks[0].icon, MarkIcon::Logs);

    let mut test = TestClient::new();
    test.enter_a_castle();
    put_down(&mut test, 1, ItemKind::CopperCoin);
    assert!(test.client.character.marks.is_empty());
}

#[test]
fn a_pile_emptied_by_someone_else_takes_the_mark_now_or_on_joining() {
    let mut test = in_the_wilderness();
    put_down(&mut test, 1, ItemKind::CopperCoin);
    let mark = test.client.character.marks[0];
    test.send(&[]);
    // Emptied by someone else: gone on the next tick.
    wild_ground(&mut test).clear();
    let news = mark_news(&test.send(&[]));
    assert_eq!(news, [MarkNews::Removed(mark.id)]);

    // While away: gone before the marks are told on joining.
    put_down(&mut test, 2, ItemKind::CopperCoin);
    let mark = test.client.character.marks[0];
    wild_ground(&mut test).clear();
    assert_eq!(test.client.drop_gone_piles(&mut test.world), [mark.id]);
    assert!(test.client.character.marks.is_empty());
    // One the player removed themselves: nothing more when its pile goes.
    put_down(&mut test, 3, ItemKind::CopperCoin);
    let mark = test.client.character.marks[0];
    test.send(&[ClientMessage::Mark(MarkAsk::Remove(mark.id))]);
    wild_ground(&mut test).clear();
    assert!(mark_news(&test.send(&[])).is_empty());
}
