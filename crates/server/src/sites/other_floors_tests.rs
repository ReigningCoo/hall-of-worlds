use shared::protocol::ServerMessage;

use super::tests::{castle_id, enter};
use crate::connections::tests::TestClient;

/// The other floors' cells sent as seen, by level.
fn seen_of_others(answers: &[ServerMessage]) -> Vec<(u32, usize)> {
    let after = answers
        .iter()
        .skip_while(|message| !matches!(message, ServerMessage::LevelCells(_)));
    let levels = after.filter_map(|message| match message {
        ServerMessage::RememberedLevel(level) => {
            let seen = level.chunks.iter().filter(|chunk| chunk.remembered);
            Some((level.index, seen.map(|chunk| chunk.cells.len()).sum()))
        }
        _ => None,
    });
    levels.collect()
}

#[test]
fn the_keeps_upper_floors_are_seen_from_the_courtyard_and_remembered() {
    let mut test = TestClient::new();
    let answers = enter(&mut test, 0);
    let seen = seen_of_others(&answers);
    assert!(
        seen.iter().any(|&(index, cells)| index == 1 && cells > 0),
        "{seen:?}"
    );
    let memory = &test.client.character.memory[&castle_id()];
    assert!(memory[&1].values().any(|&mask| mask != 0));

    // Coming back, it is remembered, so not sent again as new.
    let answers = enter(&mut test, 0);
    let before = answers
        .iter()
        .take_while(|message| !matches!(message, ServerMessage::LevelCells(_)));
    assert!(before.into_iter().any(|message| matches!(
        message,
        ServerMessage::RememberedLevel(level) if level.index == 1
    )));
    let seen = seen_of_others(&answers);
    assert!(seen.iter().all(|&(_, cells)| cells == 0), "{seen:?}");
}
