//! A player lighting or feeding a campfire (`Ability::MakeCampfire`, cooking
//! plan PR 7): kindling carried, and the fire faced (fed a piece) or else
//! ground faced that can hold one (lit with a piece; `sites::fire_target`). A
//! wind-up of `campfire::light_ticks` or `feed_ticks`; when it lands, it is
//! checked again, a piece of the first kindling carried
//! (`campfire::kindling_to_burn`) goes on the fire, and everyone who sees it
//! is sent it (`sites::sync_campfires`). Moving, being harmed or Esc stop it
//! (`stop_collecting`).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::campfire::{feed_ticks, kindling_to_burn, light_ticks};
use shared::protocol::{Landed, ServerMessage};

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;
use crate::sites::{FireTarget, fire_target, light_or_feed};

/// Begins lighting or feeding a fire, if there is kindling and somewhere to:
/// whether the client is still there, and what it cost.
pub(super) fn start_fire(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let target =
        (client.visit.as_ref()).and_then(|visit| fire_target(world, visit, client.heading));
    let (Some(target), Some(_)) = (target, kindling_to_burn(&client.inventory)) else {
        debug!("Not starting Make Campfire: no kindling, or nowhere faced for a fire");
        return (true, 0);
    };
    let lands_in = match target {
        FireTarget::Feed(_) => feed_ticks(),
        FireTarget::Light(_) => light_ticks(),
        // A hearth: lighting it or feeding it are alike quick.
        FireTarget::Hearth(_) => feed_ticks(),
    };
    let tick = world.action_clock.ticks();
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::MakeCampfire(target),
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::MakeCampfire,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// The wind-up is over: a piece of kindling goes on the fire, if it is still
/// faced and can take it (or the ground is still free). `false` if the client
/// has gone.
pub(super) fn land_fire(client: &mut Client, world: &mut ServerWorld, target: FireTarget) -> bool {
    let burnt = (client.visit.as_ref())
        .filter(|visit| fire_target(world, visit, client.heading) == Some(target))
        .zip(kindling_to_burn(&client.inventory))
        .and_then(|(visit, (id, wood))| Some((id, light_or_feed(world, visit, target, wood)?)));
    let Some((kindling, pieces)) = burnt else {
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    // The piece burns: it was found carried, so it is there to take.
    let _ = client.inventory.take_one(kindling, &mut new_id);
    client.unsaved = true;
    client.send_inventory()
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Campfire {
                pieces: pieces.min(usize::from(u8::MAX)) as u8,
            },
        })
}
