//! Dev tools' Herbs panel (`ClientMessage::DevHerbs`): learning one herb or every
//! herb in full, or forgetting them all; answered with the character's `Sheet`.

use shared::protocol::DevHerbs;

use super::Client;
use crate::ServerWorld;

/// Does it, and tells the client. Whether the client is still there.
pub(super) fn dev_herbs(client: &mut Client, world: &ServerWorld, change: DevHerbs) -> bool {
    let known = &mut client.sheet.herbs;
    match change {
        DevHerbs::Learn(key) => {
            if world.herbs.get(&key).is_some() {
                known.learn_all(&key);
            }
        }
        DevHerbs::LearnAll => {
            for def in world.herbs.iter() {
                known.learn_all(&def.key);
            }
        }
        DevHerbs::ForgetAll => known.forget_all(),
    }
    client.unsaved = true;
    client.send_sheet(world.action_clock.ticks())
}
