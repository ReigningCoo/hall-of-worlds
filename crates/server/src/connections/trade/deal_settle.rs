//! Settling a trade with a character once both sides are locked in, all at
//! once or not at all. Between players, what each offered goes into the
//! other's receive box (taking no room until taken out). To an NPC, what was
//! offered is given to it, as Give Item would from a trade's reach (coins
//! to a beggar raising standing: `quests::alms`).

use bevy::prelude::*;
use shared::protocol::{TradeEnd, TradeNews};
use shared::trade::Refusal;

use super::deal_tell::{tell_pair, tell_state};
use super::deals::NPC_REACH;
use super::{end, tell};
use crate::ServerWorld;
use crate::connections::Client;
use crate::connections::for_others::pair;

/// Settles client `i`'s trade with player `j`, or (none) with its NPC. If it
/// can't be, both sides are unlocked and told why.
pub(super) fn settle(clients: &mut [Client], world: &mut ServerWorld, i: usize, j: Option<usize>) {
    let settled = match j {
        Some(j) => between_players(clients, i, j),
        None => to_npc(&mut clients[i], world),
    };
    let tick = world.action_clock.ticks();
    match settled {
        Ok(()) => {
            for k in std::iter::once(i).chain(j) {
                let client = &mut clients[k];
                client.update_speed(tick);
                client.send_inventory();
                let received = client.trade.received().clone();
                tell(client, TradeNews::Received(received));
            }
        }
        Err(why) => {
            debug!("{} couldn't settle a trade: {why:?}", clients[i].in_log());
            for k in std::iter::once(i).chain(j) {
                if let Some(dealing) = &mut clients[k].trade.dealing {
                    dealing.side.locked = false;
                }
            }
            tell(&mut clients[i], TradeNews::Refused(why));
            match j {
                Some(_) => tell_pair(clients, i, j),
                None => tell_state(clients, i, None),
            }
        }
    }
}

/// Swaps what players `i` and `j` offered into each other's receive box.
fn between_players(clients: &mut [Client], i: usize, j: usize) -> Result<(), Refusal> {
    let (a, b) = pair(clients, i, j);
    let (Some(side_a), Some(side_b)) = (a.trade.dealing.clone(), b.trade.dealing.clone()) else {
        return Err(Refusal::NotHere);
    };
    let (side_a, side_b) = (side_a.side, side_b.side);
    if side_a.offers.is_empty() && side_b.offers.is_empty() {
        return Err(Refusal::Nothing);
    }
    let comes_off = |client: &mut Client, side: &shared::trade::Side| {
        side.offers
            .iter()
            .all(|offer| client.may_take_off(offer.item))
    };
    if !comes_off(a, &side_a) || !comes_off(b, &side_b) {
        return Err(Refusal::NotForSale);
    }
    let (mut carried_a, mut carried_b) = (a.inventory.clone(), b.inventory.clone());
    let from_a = side_a
        .hand_over(&mut carried_a)
        .ok_or(Refusal::NotCarried)?;
    let from_b = side_b
        .hand_over(&mut carried_b)
        .ok_or(Refusal::NotCarried)?;
    (a.inventory, b.inventory) = (carried_a, carried_b);
    a.trade.saved.received.items.extend(from_b);
    b.trade.saved.received.items.extend(from_a);
    info!("{} and {} traded", a.in_log(), b.in_log());
    let (with_a, with_b) = (a.entity_id(), b.entity_id());
    for (client, with) in [(a, with_b), (b, with_a)] {
        client.unsaved = true;
        end(client, with, TradeEnd::Exchanged);
    }
    Ok(())
}

/// Gives the NPC what the player offered.
fn to_npc(client: &mut Client, world: &mut ServerWorld) -> Result<(), Refusal> {
    let dealing = client.trade.dealing.clone().ok_or(Refusal::NotHere)?;
    let side = &dealing.side;
    if side.offers.is_empty() {
        return Err(Refusal::Nothing);
    }
    if !side.still_carried(&client.inventory)
        || !(side.offers.iter()).all(|offer| client.may_take_off(offer.item))
    {
        return Err(Refusal::NotCarried);
    }
    for offer in &side.offers {
        let copper = (client.inventory.find(offer.item)).map_or(0, crate::quests::copper_in);
        match crate::entities::give_within(client, world, (offer.item, dealing.with), NPC_REACH) {
            // Coins to a beggar raise standing in its town (trading plan PR 7).
            Ok(kind) => crate::quests::gave(client, world, kind, copper),
            Err(why) => warn!(
                "{} couldn't hand over {:?}: {why}",
                client.in_log(),
                offer.item
            ),
        }
    }
    info!("{} traded with {:?}", client.in_log(), dealing.with);
    client.unsaved = true;
    end(client, dealing.with, TradeEnd::Exchanged);
    Ok(())
}
