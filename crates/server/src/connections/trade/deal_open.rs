//! Opening a trade with a player or an NPC, both within a trade's reach on
//! the same level. A player's Trading setting decides: Allow opens it on
//! both sides, Ask puts it to them first, Never refuses. An NPC (one of the
//! people kinds, not unfriendly, in sight) trades straight away, its side
//! empty and locked in.

use bevy::prelude::*;
use shared::id::Id;
use shared::protocol::{TradeEnd, TradeNews};
use shared::sheet::Feeling;
use shared::trade::{Refusal, TradingSetting, in_reach};

use super::deal_tell::{tell_pair, tell_state};
use super::deals::{end_deal, near};
use super::{DealState, Dealing, end, tell};
use crate::ServerWorld;
use crate::connections::{Client, Phase};

/// Opens client `i`'s trade with `with`, a player or an NPC, closing any
/// other it has open, or tells why not.
pub(super) fn open_deal(clients: &mut [Client], world: &mut ServerWorld, i: usize, with: Id) {
    end_deal(clients, i, TradeEnd::Closed, TradeEnd::Closed);
    if let Some(shop) = clients[i].trade.shop.as_ref().map(|shop| shop.with) {
        end(&mut clients[i], shop, TradeEnd::Closed);
    }
    if !clients[i].trade.may_start() {
        tell(&mut clients[i], TradeNews::Refused(Refusal::Unclaimed));
        return;
    }
    let world_id = clients[i].world;
    let found = clients.iter().position(|other| {
        !other.gone
            && other.phase == Phase::Playing
            && other.world == world_id
            && other.entity_id() == with
    });
    let opened = match found.filter(|&j| j != i) {
        Some(j) => with_player(clients, i, j),
        None => with_npc(&mut clients[i], world, with),
    };
    if let Err(why) = opened {
        debug!(
            "{} couldn't trade with {with:?}: {why:?}",
            clients[i].in_log()
        );
        tell(&mut clients[i], TradeNews::Refused(why));
    }
}

/// Opens a trade between players `i` and `j`, or asks `j`, by `j`'s setting.
fn with_player(clients: &mut [Client], i: usize, j: usize) -> Result<(), Refusal> {
    if !near(&clients[i], &clients[j]) {
        return Err(Refusal::NotHere);
    }
    let other = &clients[j];
    if other.trade.setting() == TradingSetting::Never {
        return Err(Refusal::Never);
    }
    let busy = other.trade.dealing.is_some() || other.trade.shop.is_some();
    if busy || !other.trade.may_start() {
        return Err(Refusal::Busy);
    }
    let (me, them) = (clients[i].entity_id(), clients[j].entity_id());
    if clients[j].trade.setting() == TradingSetting::Allow {
        clients[i].trade.dealing = Some(Dealing::new(them, false, DealState::Open));
        clients[j].trade.dealing = Some(Dealing::new(me, false, DealState::Open));
        info!(
            "{} and {} opened a trade",
            clients[i].in_log(),
            clients[j].in_log()
        );
        tell_pair(clients, i, Some(j));
        return Ok(());
    }
    clients[i].trade.dealing = Some(Dealing::new(them, false, DealState::Asking));
    clients[j].trade.dealing = Some(Dealing::new(me, false, DealState::Asked));
    let name = clients[i].name.clone();
    tell(&mut clients[i], TradeNews::Asking { with: them });
    tell(&mut clients[j], TradeNews::Asked { by: me, name });
    Ok(())
}

/// Opens a trade with the NPC `with` on the client's level.
fn with_npc(client: &mut Client, world: &mut ServerWorld, with: Id) -> Result<(), Refusal> {
    if !npc_in_reach(client, world, with) {
        return Err(Refusal::NotHere);
    }
    client.trade.dealing = Some(Dealing::new(with, true, DealState::Open));
    // Only this client: `tell_state` reaches it by index, so a one-long slice.
    tell_state(std::slice::from_mut(client), 0, None);
    Ok(())
}

/// Whether `with` is an NPC the client may trade with: one of the people
/// kinds (those with an alignment), not unfriendly to them, in sight on
/// their level and within a trade's reach.
pub(super) fn npc_in_reach(client: &mut Client, world: &mut ServerWorld, with: Id) -> bool {
    let (kind, character) = (client.inventory.creature, client.character_id);
    let Some(visit) = client.visit.as_mut() else {
        return false;
    };
    let Some(population) = world.populations.0.get(&visit.population()) else {
        return false;
    };
    let Some(mob) = population.mobs.iter().find(|mob| mob.id == with) else {
        return false;
    };
    let friendly = mob.feeling_shown(kind, character.as_ref()) != Feeling::Unfriendly;
    let (at, cell) = (mob.spot, mob.position);
    mob.alignment.is_some()
        && friendly
        && in_reach(visit.spot(), at)
        && visit.can_see(cell.0, cell.1)
}
