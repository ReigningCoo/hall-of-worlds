//! Trades with characters, done once every client's messages this tick have
//! been read, as only then can both sides be reached: what each asked
//! (`deal_open.rs`, `deal_change.rs`), then every trade kept honest. One whose
//! other side has gone, closed, left the level or walked out of reach ends
//! with nothing moved; something offered but no longer carried is taken off
//! its side, unlocking it.

use bevy::prelude::*;
use shared::id::Id;
use shared::protocol::TradeEnd;
use shared::trade::{REACH_CELLS, for_trade, in_reach};

use super::deal_tell::tell_pair;
use super::{DealAsk, DealState, end};
use crate::connections::{Client, Phase};
use crate::{ServerWorld, ServerWorlds};

/// Does what each client asked of its trades with characters, then checks
/// every such trade.
pub(in crate::connections) fn do_deals(clients: &mut [Client], worlds: &mut ServerWorlds) {
    for i in 0..clients.len() {
        for ask in std::mem::take(&mut clients[i].trade.asked) {
            if clients[i].gone || clients[i].phase != Phase::Playing {
                break;
            }
            let world = worlds.of(clients[i].world);
            match ask {
                DealAsk::Open(with) => super::deal_open::open_deal(clients, world, i, with),
                DealAsk::Change(ask) => super::deal_change::change_deal(clients, world, i, ask),
            }
        }
    }
    for i in 0..clients.len() {
        if !clients[i].gone && clients[i].trade.dealing.is_some() {
            let world = worlds.of(clients[i].world);
            keep_dealing(clients, world, i);
        }
    }
}

/// The other player in client `i`'s trade, if they are still playing in the
/// same world and their trade is with `i` too (at the matching stage).
pub(super) fn partner(clients: &[Client], i: usize) -> Option<usize> {
    let dealing = clients[i].trade.dealing.as_ref().filter(|deal| !deal.npc)?;
    let (me, world) = (clients[i].entity_id(), clients[i].world);
    let j = clients.iter().position(|other| {
        !other.gone
            && other.phase == Phase::Playing
            && other.world == world
            && other.entity_id() == dealing.with
    })?;
    let theirs = clients[j].trade.dealing.as_ref()?;
    let matches = matches!(
        (dealing.state, theirs.state),
        (DealState::Asking, DealState::Asked)
            | (DealState::Asked, DealState::Asking)
            | (DealState::Open, DealState::Open)
    );
    (j != i && !theirs.npc && theirs.with == me && matches).then_some(j)
}

/// Whether two players stand on one level within a trade's reach.
pub(super) fn near(a: &Client, b: &Client) -> bool {
    let (Some(a), Some(b)) = (&a.visit, &b.visit) else {
        return false;
    };
    a.population() == b.population() && in_reach(a.spot(), b.spot())
}

/// How far an NPC may be to be traded with, in cells.
pub(super) const NPC_REACH: i64 = REACH_CELLS as i64;

/// Ends client `i`'s trade with a character, and the other side's too, each
/// told `why` (the other `theirs`). Nothing moves.
pub(super) fn end_deal(clients: &mut [Client], i: usize, why: TradeEnd, theirs: TradeEnd) {
    let j = partner(clients, i);
    let Some(with) = clients[i].trade.dealing.as_ref().map(|deal| deal.with) else {
        return;
    };
    end(&mut clients[i], with, why);
    if let Some(j) = j {
        let me = clients[i].entity_id();
        end(&mut clients[j], me, theirs);
    }
}

/// Checks client `i`'s trade with a character still stands, ending it (both
/// sides) if not, and takes off its side what is no longer carried.
fn keep_dealing(clients: &mut [Client], world: &mut ServerWorld, i: usize) {
    let Some(dealing) = clients[i].trade.dealing.clone() else {
        return;
    };
    let j = partner(clients, i);
    let stands = match j {
        Some(j) => near(&clients[i], &clients[j]),
        None if dealing.npc => super::deal_open::npc_in_reach(&mut clients[i], world, dealing.with),
        None => false,
    };
    if !stands {
        debug!(
            "{}'s trade with {:?} ended",
            clients[i].in_log(),
            dealing.with
        );
        end_deal(clients, i, TradeEnd::OutOfReach, TradeEnd::OutOfReach);
        return;
    }
    let client = &mut clients[i];
    let gone: Vec<Id> = (dealing.side.offers.iter())
        .filter(|offer| !client.inventory.find(offer.item).is_some_and(for_trade))
        .map(|offer| offer.item)
        .collect();
    if gone.is_empty() {
        return;
    }
    if let Some(dealing) = &mut client.trade.dealing {
        gone.iter().for_each(|&item| dealing.side.take_back(item));
    }
    tell_pair(clients, i, j);
}
