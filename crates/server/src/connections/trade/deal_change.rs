//! Changing a trade with a character: answering being asked, offering or
//! taking back, locking in or out, and closing. Any change to a side unlocks
//! it; both sides are told after every change. Once both are locked in it
//! settles (`deal_settle.rs`).

use bevy::prelude::*;
use shared::protocol::{TradeAsk, TradeEnd, TradeNews};
use shared::trade::Refusal;

use super::deal_tell::{tell_pair, tell_state};
use super::deals::{end_deal, partner};
use super::{DealState, tell};
use crate::ServerWorld;
use crate::connections::Client;

/// Does `ask` to client `i`'s trade with a character, or tells why not.
pub(super) fn change_deal(
    clients: &mut [Client],
    world: &mut ServerWorld,
    i: usize,
    ask: TradeAsk,
) {
    let Some(dealing) = clients[i].trade.dealing.clone() else {
        debug!("Refused {ask:?} with no trade open");
        tell(&mut clients[i], TradeNews::Refused(Refusal::NotHere));
        return;
    };
    let j = partner(clients, i);
    let open = dealing.state == DealState::Open;
    let done = match ask {
        TradeAsk::Close => {
            end_deal(clients, i, TradeEnd::Closed, TradeEnd::Closed);
            return;
        }
        TradeAsk::Answer { by, allow }
            if dealing.state == DealState::Asked && dealing.with == by =>
        {
            answer(clients, i, j, allow);
            return;
        }
        TradeAsk::Offer(offer) if open => {
            let client = &mut clients[i];
            if client.may_take_off(offer.item) {
                let side = &mut client.trade.dealing.as_mut().expect("checked above").side;
                side.offer(offer, &client.inventory)
            } else {
                Err(Refusal::NotForSale)
            }
        }
        TradeAsk::Unsell { item } if open => {
            let dealing = clients[i].trade.dealing.as_mut().expect("checked above");
            dealing.side.take_back(item);
            Ok(())
        }
        TradeAsk::Lock(locked) if open => {
            let dealing = clients[i].trade.dealing.as_mut().expect("checked above");
            dealing.side.locked = locked;
            let npc = dealing.npc;
            let theirs = j.and_then(|j| clients[j].trade.dealing.as_ref());
            if locked && (npc || theirs.is_some_and(|theirs| theirs.side.locked)) {
                super::deal_settle::settle(clients, world, i, j);
                return;
            }
            Ok(())
        }
        _ => Err(Refusal::NotHere),
    };
    if let Err(why) = done {
        debug!("Refused a trade change ({ask:?}): {why:?}");
        tell(&mut clients[i], TradeNews::Refused(why));
        tell_state(clients, i, j);
        return;
    }
    tell_pair(clients, i, j);
}

/// Client `i` answers being asked by `j`: opened on both sides, or `j` told
/// it was denied.
fn answer(clients: &mut [Client], i: usize, j: Option<usize>, allow: bool) {
    let Some(j) = j else {
        end_deal(clients, i, TradeEnd::OutOfReach, TradeEnd::OutOfReach);
        return;
    };
    if !allow {
        end_deal(clients, i, TradeEnd::Closed, TradeEnd::Denied);
        return;
    }
    for k in [i, j] {
        if let Some(dealing) = &mut clients[k].trade.dealing {
            dealing.state = DealState::Open;
        }
    }
    info!(
        "{} and {} opened a trade",
        clients[i].in_log(),
        clients[j].in_log()
    );
    tell_pair(clients, i, Some(j));
}
