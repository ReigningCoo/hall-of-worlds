use shared::map_mark::{MOST_HAND_MARKS, MapMark, MarkColour, MarkIcon};
use shared::protocol::{ClientMessage, MarkAsk, MarkNews, ServerMessage};
use shared::wilderness::SCALE;

use super::tests::TestClient;

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
