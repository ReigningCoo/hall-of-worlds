//! Sending steps and messages, counting map changes, and hanging up on quitting.

use bevy::prelude::*;
use shared::protocol::ServerMessage;

use super::prediction::Prediction;
use super::stumble::Stumbling;
use super::{FromServer, ServerConnection, ToServer};
use crate::player::{Moved, Stepped};

/// Sends the steps and moves the player took this frame, keeping them until
/// the server confirms them: on the world map as steps, on a level or in the
/// Hall as moves in units. A stumble (`stumble.rs`) puts the player where it
/// ends, and drops the rest walked this frame.
pub(super) fn send_steps(
    (mut stepped, mut moved): (MessageReader<Stepped>, MessageReader<Moved>),
    mut prediction: ResMut<Prediction>,
    mut to_server: MessageWriter<ToServer>,
    mut stumbling: Stumbling,
) {
    let steps: Vec<Stepped> = stepped.read().copied().collect();
    for Stepped { step, moved, to } in steps {
        let (map, number) = prediction.next();
        let stumbled = stumbling.world_step(map, number, step, (to, moved));
        let moved = stumbled.unwrap_or(moved);
        to_server.write(ToServer(prediction.step(step, moved)));
        if stumbled.is_some() {
            break;
        }
    }
    let moves: Vec<Moved> = moved.read().copied().collect();
    for Moved { from, to } in moves {
        let (map, number) = prediction.next();
        let landed = match prediction.in_hall() {
            true => None,
            false => stumbling.level_move(map, number, from, to),
        };
        let Some(landed) = landed else {
            to_server.write(ToServer(prediction.moved(from, to)));
            continue;
        };
        to_server.write(ToServer(prediction.moved_to(from, to, landed)));
        break;
    }
}

/// The server has put the player on another map (applied by `site` this
/// frame): steps from now on are on it.
pub(super) fn count_map_changes(
    mut from_server: MessageReader<FromServer>,
    mut prediction: ResMut<Prediction>,
) {
    for FromServer(message) in from_server.read() {
        match message {
            ServerMessage::EnterLevel(_) => prediction.changed_map(true),
            ServerMessage::EnterHall { .. } => prediction.changed_map_to(true, true),
            ServerMessage::OnWorldMap { .. } => prediction.changed_map(false),
            ServerMessage::Teleported { .. } => {
                let (on_level, hall) = (prediction.on_level(), prediction.in_hall());
                prediction.changed_map_to(on_level, hall);
            }
            _ => {}
        }
    }
}

/// Sends what other systems wrote this frame.
pub(super) fn send(connection: Res<ServerConnection>, mut to_server: MessageReader<ToServer>) {
    for message in to_server.read() {
        connection.link.send(&message.0);
    }
}

/// Quitting: hangs up, so the server knows the player has gone (and an
/// embedded one saves and stops; this waits for it).
pub(super) fn hang_up_on_exit(mut commands: Commands, mut exit: MessageReader<AppExit>) {
    if exit.read().next().is_some() {
        commands.remove_resource::<ServerConnection>();
    }
}
