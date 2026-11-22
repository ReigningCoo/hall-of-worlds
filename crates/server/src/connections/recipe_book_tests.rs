//! The recipe book through a connected client (recipes plan PR 3): a scroll
//! carried or on the ground read in and used up, told back for the client to
//! open; one already known refused and kept; a learnt one removed giving its
//! scroll back.

use shared::id::Id;
use shared::item::{ItemKind, StackLimits};
use shared::protocol::{ClientMessage, RecipeAsk, ServerMessage};
use shared::recipe::{Entry, RecipeForm, new_item};

use super::tests::TestClient;

/// A client in a castle carrying `count` scrolls of Naan as stack 7.
fn with_scrolls(count: u32) -> (TestClient, Id) {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let stack = Id::for_test(7);
    let scrolls = new_item(RecipeForm::cooking("naan"), count, stack, None);
    let new_id = &mut shared::id::stand_ins();
    let given = test
        .client
        .inventory
        .give(scrolls, StackLimits::NONE, new_id);
    assert!(given.is_ok());
    (test, stack)
}

fn insert(item: Id) -> ClientMessage {
    ClientMessage::Recipe(RecipeAsk::Insert { item })
}

fn scrolls(test: &TestClient) -> u32 {
    test.client.inventory.count_of(ItemKind::Recipe)
}

fn inserted(answers: &[ServerMessage]) -> Option<&Entry> {
    answers.iter().find_map(|answer| match answer {
        ServerMessage::RecipeInserted(entry) => Some(entry),
        _ => None,
    })
}

#[test]
fn reading_a_scroll_uses_it_up_and_a_known_one_is_kept() {
    let (mut test, stack) = with_scrolls(2);
    let answers = test.send(&[insert(stack)]);
    let form = RecipeForm::cooking("naan");
    assert!(test.client.sheet.book.knows(&form));
    assert_eq!(scrolls(&test), 1, "one scroll used up");
    let told = inserted(&answers).expect("told to open it");
    assert!(matches!(told, Entry::Learnt { dish, scroll: true, .. } if dish == "naan"));
    let answers = test.send(&[insert(stack)]);
    assert_eq!(scrolls(&test), 1, "known already: kept");
    assert!(inserted(&answers).is_none());
}

#[test]
fn a_scroll_on_the_ground_is_read_where_it_lies() {
    let (mut test, stack) = with_scrolls(1);
    test.send(&[ClientMessage::DropItem { item: stack }]);
    assert_eq!(scrolls(&test), 0, "dropped");
    test.send(&[insert(stack)]);
    assert!(test.client.sheet.book.knows(&RecipeForm::cooking("naan")));
}

#[test]
fn removing_a_learnt_recipe_gives_its_scroll_back() {
    let (mut test, stack) = with_scrolls(1);
    test.send(&[insert(stack)]);
    assert_eq!(scrolls(&test), 0);
    let entry = test.client.sheet.book.iter().next().cloned().unwrap();
    test.send(&[ClientMessage::Recipe(RecipeAsk::Remove(entry.clone()))]);
    assert!(test.client.sheet.book.is_empty());
    assert_eq!(scrolls(&test), 1, "the scroll back");
    test.send(&[ClientMessage::Recipe(RecipeAsk::Remove(entry))]);
    assert_eq!(scrolls(&test), 1, "not in the book: nothing given");
}
