use shared::door::Swing;
use shared::hall::{self, GatePlace, INN_WAY_OFFSET, WALKWAY_ROWS};
use shared::inn::{self, spots};
use shared::protocol::{ClientMessage, DoorAsk, LevelKind, ServerMessage};
use shared::spot::Spot;

use crate::connections::tests::TestClient;

/// A client in the Hall in front of a gate, and that tile (the Hall's own).
fn in_the_hall() -> (TestClient, (i64, i64)) {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevToHall {
        seed: None,
        gate: None,
    }]);
    let at = test.client.character.hall.expect("in the Hall").cell();
    (test, at)
}

/// The level entered, from what the server said: its kind, index and the
/// cell arrived on.
fn entered(answers: &[ServerMessage]) -> Option<(LevelKind, u32, (i64, i64))> {
    answers.iter().find_map(|message| match message {
        ServerMessage::EnterLevel(info) => Some((info.kind, info.index, info.arrival.cell())),
        _ => None,
    })
}

fn in_inn(test: &TestClient) -> bool {
    test.client
        .visit
        .as_ref()
        .is_some_and(|visit| visit.id.is_inn())
}

/// In the Hall, walked along to the way in just past the gate, then off it:
/// the client, and the tile walked off from.
fn into_the_inn() -> (TestClient, (i64, i64)) {
    let (mut test, (x, y)) = in_the_hall();
    test.steps(&[(1, 0), (1, 0)]);
    let front = test
        .client
        .character
        .hall
        .expect("still in the Hall")
        .cell();
    assert_eq!(front, (x + INN_WAY_OFFSET, y));
    assert!(hall::inn_way_behind(front.0, front.1).is_some());
    let answers = test.send(&[ClientMessage::UseExit]);
    let arrived = (LevelKind::Inn, inn::GROUND, spots::ARRIVAL);
    assert_eq!(entered(&answers), Some(arrived));
    (test, front)
}

#[test]
fn off_the_halls_edge_at_a_way_in_into_the_inns_foyer() {
    let (test, front) = into_the_inn();
    assert!(in_inn(&test));
    assert_eq!(test.client.character.hall, None);
    let back = Some(Spot::middle_of(front));
    assert_eq!(test.client.character.inn_return, back);
}

#[test]
fn anywhere_else_off_the_edge_is_refused() {
    let (mut test, (x, y)) = in_the_hall();
    test.steps(&[(1, 0)]);
    let answers = test.send(&[ClientMessage::UseExit]);
    assert_eq!(entered(&answers), None);
    let at = test
        .client
        .character
        .hall
        .expect("still in the Hall")
        .cell();
    assert_eq!(at, (x + 1, y));
    assert!(!in_inn(&test));
}

#[test]
fn out_of_the_entrance_door_back_to_the_same_tile() {
    let (mut test, front) = into_the_inn();
    let (x, y) = spots::ENTRANCE;
    test.send(&[ClientMessage::Door(DoorAsk::Open { x, y })]);
    let answers = test.steps(&[(0, 1)]);
    assert!(!in_inn(&test));
    assert_eq!(test.client.character.inn_return, None);
    let back = (hall::wrap_x(front.0), front.1);
    assert_eq!(test.client.character.hall.map(Spot::cell), Some(back));
    let in_hall = |m: &ServerMessage| matches!(m, ServerMessage::EnterHall { .. });
    assert!(answers.iter().any(in_hall));
}

#[test]
fn the_stairs_go_up_and_down() {
    let (mut test, _) = into_the_inn();
    let (x, y) = spots::STAIRS;
    let visit = test.client.visit.as_mut().expect("in the inn");
    visit.position = Spot::middle_of((x, y));
    let up = test.send(&[ClientMessage::Up]);
    assert_eq!(entered(&up), Some((LevelKind::Inn, inn::UPSTAIRS, (x, y))));
    let down = test.send(&[ClientMessage::Down]);
    assert_eq!(entered(&down), Some((LevelKind::Inn, inn::GROUND, (x, y))));
}

#[test]
fn the_treasury_door_is_locked_and_the_others_closed() {
    let (test, _) = into_the_inn();
    let visit = test.client.visit.as_ref().expect("in the inn");
    let level = &visit.level.level;
    let (x, y) = spots::TREASURY_DOOR;
    assert!(level.is_locked(x, y));
    let (x, y) = spots::KITCHEN_DOOR;
    assert!(!level.is_locked(x, y));
    assert_eq!(level.swing(x, y), Some(Swing::Closed));
}

#[test]
fn dev_tools_go_in_from_the_world_map_and_out_into_the_hall() {
    let mut test = TestClient::new();
    let answers = test.send(&[ClientMessage::DevToInn]);
    let kind = entered(&answers).map(|(kind, ..)| kind);
    assert_eq!(kind, Some(LevelKind::Inn));
    let back = test.client.character.inn_return.expect("a way back");
    let (x, y) = back.cell();
    assert!(hall::inn_way_behind(x, y).is_some());
    // Beside a gate, as Teleport to World's Hall of Worlds would use.
    let gate_y = if y == 0 { -1 } else { WALKWAY_ROWS };
    assert!(GatePlace::at(x - INN_WAY_OFFSET, gate_y).is_some());
    assert!(super::leave_inn(&mut test.client));
    test.send(&[]);
    assert_eq!(test.client.character.hall, Some(back));
}

#[test]
fn the_entrance_door_shuts_behind_whoever_comes_in() {
    let (mut test, _) = into_the_inn();
    let (x, y) = spots::ENTRANCE;
    test.send(&[ClientMessage::Door(DoorAsk::Open { x, y })]);
    test.steps(&[(0, 1)]);
    assert!(!in_inn(&test));
    test.send(&[ClientMessage::UseExit]);
    assert!(in_inn(&test));
    let visit = test.client.visit.as_ref().expect("in the inn");
    assert_eq!(visit.level.level.swing(x, y), Some(Swing::Closed));
}

#[test]
fn new_characters_start_in_the_main_room_and_come_out_along_the_hall() {
    let mut test = TestClient::new();
    // As joining sets it: resuming onto a spot needs the world they came from.
    test.client.world = Some(test.world.seed.id());
    assert!(super::start_in_inn(&mut test.client));
    let answers = test.send(&[]);
    let start = (LevelKind::Inn, inn::GROUND, spots::START);
    assert_eq!(
        entered(&answers).map(|(kind, index, _)| (kind, index)),
        Some((start.0, start.1))
    );
    let at = test
        .client
        .visit
        .as_ref()
        .expect("in the inn")
        .position
        .cell();
    assert_eq!(at, spots::START);
    let told = shared::protocol::Tutorial::WayInn;
    assert!(
        answers.contains(&ServerMessage::Tutorial(told)),
        "{answers:?}"
    );
    let id = test.client.character_id.expect("a character");
    let back = super::first_exit::first_way_back(&id);
    assert_eq!(test.client.character.inn_return, Some(back));
    // Told once only.
    assert!(!test.send(&[]).contains(&ServerMessage::Tutorial(told)));
    assert!(super::leave_inn(&mut test.client));
    test.send(&[]);
    assert_eq!(test.client.character.hall, Some(back));
}
