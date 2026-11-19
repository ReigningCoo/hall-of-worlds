//! Sight following the character's difficulty: the cone on Hard, all round on
//! Easy, worked out again as soon as the difficulty changes.

use shared::difficulty::Difficulty;
use shared::protocol::ClientMessage;

use super::tests::enter;
use crate::connections::tests::TestClient;

/// How many cells of the level they are on the player remembers seeing.
fn remembered(test: &TestClient) -> u32 {
    let visit = test.client.visit.as_ref().expect("on a level");
    let memory = &test.client.character.memory[&visit.id][&visit.index];
    memory.values().map(|mask| mask.count_ones()).sum()
}

#[test]
fn changing_to_easy_sees_what_is_behind_at_once() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::SetDifficulty(Difficulty::Hard)]);
    enter(&mut test, 0);
    let in_cone = remembered(&test);
    assert!(in_cone > 0);
    test.send(&[ClientMessage::SetDifficulty(Difficulty::Easy)]);
    assert!(remembered(&test) > in_cone, "seeing all round sees more");
    assert!(test.client.field().is_all_round());
}

#[test]
fn hard_keeps_the_cone() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::SetDifficulty(Difficulty::Hard)]);
    assert!(!test.client.field().is_all_round());
    test.send(&[ClientMessage::SetDifficulty(Difficulty::Normal)]);
    assert!(test.client.field().is_all_round());
}
