//! Noticing hostile actions involving the player (`common::Attacked`), from
//! what the server already tells: a blow landing on the player, the player's
//! blow landing on an enemy, an enemy beginning an attack aimed at the player,
//! and someone in sight turning unfriendly (the server shows a mob as
//! `Unfriendly` while it has the player in its sights).

use std::collections::HashMap;

use bevy::prelude::*;
use shared::id::Id;
use shared::protocol::{Landed, ServerMessage};
use shared::sheet::Feeling;

use crate::common::Attacked;
use crate::connection::FromServer;

/// Each creature's feeling as last seen, to notice one turning unfriendly.
#[derive(Resource, Default)]
pub(super) struct Feelings(HashMap<Id, Feeling>);

/// Tells `Attacked` for each hostile action in what the server says.
pub(super) fn notice_attacks(
    mut from_server: MessageReader<FromServer>,
    mut feelings: ResMut<Feelings>,
    mut attacked: MessageWriter<Attacked>,
) {
    for FromServer(message) in from_server.read() {
        if hostile(message, &mut feelings.0) {
            attacked.write(Attacked);
        }
    }
}

/// Whether a message tells of a hostile action involving the player,
/// keeping `feelings` up to date as it goes.
pub(super) fn hostile(message: &ServerMessage, feelings: &mut HashMap<Id, Feeling>) -> bool {
    let unfriendly =
        |id: &Id, feelings: &HashMap<Id, Feeling>| feelings.get(id) == Some(&Feeling::Unfriendly);
    match message {
        ServerMessage::EntitySeen(look) => {
            let was = feelings.insert(look.id, look.feeling);
            // Seen again, turned on the player: not when first seen, as a
            // creature unfriendly by nature may not be after them.
            look.feeling == Feeling::Unfriendly && was.is_some_and(|was| was != Feeling::Unfriendly)
        }
        ServerMessage::EntityGone { id, .. } => {
            feelings.remove(id);
            false
        }
        ServerMessage::AbilityLanded {
            who,
            target,
            landed: Landed::Hit { .. },
        } => match (who, target) {
            // A blow on the player.
            (Some(_), None) => true,
            // The player's blow on an enemy.
            (None, Some(id)) => unfriendly(id, feelings),
            _ => false,
        },
        ServerMessage::AbilityStarted {
            who: Some(id),
            target: None,
            ..
        } => unfriendly(id, feelings),
        _ => false,
    }
}

#[cfg(test)]
mod tests;
