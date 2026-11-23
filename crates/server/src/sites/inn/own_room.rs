//! Players' own rooms in The Way Inn (`shared::inn::own_rooms`): everyone
//! who goes in has their own copy at the same time. Their copy is a site of
//! its own, `SiteId::InnRoom`, kept by the home world while they are in the
//! inn and saved with the character, the same on every world
//! (`CharacterSave::inn_room`); the level they walk is still their
//! instance's, so they see its corridors and who is in them. What lies in
//! the rooms' chests and wardrobes and on their floors is kept in their copy
//! (`Visit::own_room_at`: benches' keys, the ground), and nobody in one is
//! seen by anyone else.

use shared::inn;

use crate::ServerWorld;
use crate::connections::Client;
use crate::sites::Visit;
use crate::storage::SiteId;

impl Visit {
    /// Where things at `(x, y)` are kept if it is in one of the player's own
    /// rooms: their own copy.
    pub(crate) fn own_room_at(&self, x: i64, y: i64) -> Option<SiteId> {
        let mine = self.own_room?;
        inn::own_room_at(self.index, (x, y)).map(|_| mine)
    }

    /// Where the ground under the player is kept: their own room's copy if
    /// they stand in one, else the level's site.
    pub(crate) fn ground_site(&self) -> SiteId {
        let (x, y) = self.position();
        self.own_room_at(x, y).unwrap_or(self.id)
    }

    /// The player's own rooms' copy, while in the inn.
    pub(crate) fn own_rooms(&self) -> Option<SiteId> {
        self.own_room
    }
}

/// Whether someone on `level`, in cell `at`, is in an own room of the inn,
/// where nobody else sees them: everyone there is in their own copy.
pub(crate) fn hidden_in_own_room(level: Option<(SiteId, u32)>, at: (i64, i64)) -> bool {
    level.is_some_and(|(id, index)| id.is_inn() && inn::own_room_at(index, at).is_some())
}

/// The client's own rooms' copy kept by `world` (the home world) as saved,
/// coming into the inn; already kept, as it is.
pub(super) fn open_own_room(client: &Client, world: &mut ServerWorld) {
    let Some(character) = client.character_id else {
        return;
    };
    let id = SiteId::InnRoom(character);
    if world.own_room(id).is_none() {
        world.put_own_room(id, client.inn.room.clone());
    }
}

/// What has changed in the client's own rooms, kept with them to be saved:
/// whether anything had.
pub(crate) fn keep_own_room(client: &mut Client, world: &ServerWorld) -> bool {
    let Some(character) = client.character_id else {
        return false;
    };
    match world.own_room(SiteId::InnRoom(character)) {
        Some(kept) if *kept != client.inn.room => {
            client.inn.room = kept.clone();
            client.unsaved = true;
            true
        }
        _ => false,
    }
}

/// Out of the inn (or gone): what is in their own rooms is kept with them,
/// and `world` lets go of it.
pub(crate) fn close_own_room(client: &mut Client, world: &mut ServerWorld) {
    keep_own_room(client, world);
    if let Some(character) = client.character_id {
        world.drop_own_room(SiteId::InnRoom(character));
    }
}

#[cfg(test)]
mod tests;
