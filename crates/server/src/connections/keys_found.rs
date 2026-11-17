//! Keys remember where a character first had them (`item_form::KeyFound`):
//! however a key came to be carried (picked up, given, a quest's, Key for This
//! Door), it is noted here, once, where its first holder is.

use bevy::prelude::*;
use shared::door::{has_unfound_keys, note_found_keys};

use super::systems::for_each_world;
use super::{Client, Connections, Phase};
use crate::sites::key_place;
use crate::{ServerWorld, ServerWorlds};

/// Every tick: notes where each player is on any key they carry that no
/// character has had before, and tells them.
pub(crate) fn note_found(mut connections: ResMut<Connections>, mut worlds: ResMut<ServerWorlds>) {
    for_each_world(&mut connections.clients, &mut worlds, |clients, world| {
        for client in clients {
            note_for(client, world);
        }
    });
}

/// Notes where `client` is on their keys not had before. A client that has
/// gone is noticed when its messages are next read.
pub(super) fn note_for(client: &mut Client, world: &ServerWorld) {
    if client.phase != Phase::Playing || !has_unfound_keys(&client.inventory) {
        return;
    }
    let place = key_place(client, world);
    if note_found_keys(&mut client.inventory, &place) {
        client.unsaved = true;
        client.send_inventory();
    }
}
