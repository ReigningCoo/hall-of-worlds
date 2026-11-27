//! Holding and letting go in a trade (trading plan PR 4, `common::Trading`):
//! what is carried let go on the trade window's own side is sold (it stays
//! carried, marked), and taken off it anywhere else is taken back; what is
//! being bought, drawn unpaid where it is to go, takes room there, can be
//! moved (bought again to its new place) or put back on the shop's grid; and
//! nothing more is bought than can be paid for.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Inventory;
use shared::protocol::{ClientMessage, TradeAsk};
use shared::trade::Buy;

use super::super::hold::{Target, drop_move};
use super::super::{HeldBy, Holding};
use super::Outbox;
use crate::common::{GaveItem, HeldOverGifts, HeldStock, Trading, own_side};
use crate::connection::ToServer;

/// What is carried with what is being bought placed where it is to go (but
/// `held`), for aiming: none when nothing is being bought.
pub(super) fn with_unpaid(carried: &Inventory, trading: &Trading, held: Id) -> Option<Inventory> {
    if trading.buying.is_empty() {
        return None;
    }
    let mut view = carried.clone();
    for (buy, item) in trading.buying.iter().filter(|(buy, _)| buy.item != held) {
        // (Anywhere it fits, if its place has been taken since: not drawn.)
        let _ = view.add(item.clone(), buy.to, buy.x, buy.y, buy.rotation);
    }
    Some(view)
}

/// `target` as the trade allows it: never onto something unpaid, and, when
/// `buying` something new from the shop, only if it can be paid for.
pub(super) fn trade_aim(
    target: Target,
    trading: &Trading,
    buying: Option<&shared::item::Item>,
    coins: u64,
) -> Target {
    let unpaid = |id| trading.unpaid(id).is_some();
    let afford = buying.is_none_or(|item| trading.affords(item, coins));
    match target {
        Target::Stack { onto, .. } if unpaid(onto) || !afford => Target::Stack {
            onto,
            allowed: false,
        },
        Target::Bag { bag, .. } if unpaid(bag) || !afford => Target::Bag {
            bag,
            place: None,
            onto: None,
        },
        Target::Cells {
            holder,
            x,
            y,
            allowed,
        } => Target::Cells {
            holder,
            x,
            y,
            allowed: allowed && afford,
        },
        target => target,
    }
}

/// Lets go in the trade, if this is in it: `true` if done.
pub(super) fn let_go_in_trade(
    held: HeldBy,
    aim: Target,
    holding: &mut ResMut<Holding>,
    carried: &Inventory,
    (over_gifts, held_stock): (&HeldOverGifts, &mut ResMut<HeldStock>),
    out: &mut Outbox,
) -> bool {
    let Outbox {
        to_server,
        gave,
        trading,
        ..
    } = out;
    let mut send = |ask| {
        to_server.write(ToServer(ClientMessage::Trade(ask)));
    };
    // Let go on the own side, or the shop's: sold (shown on the own side).
    let selling = match aim {
        Target::Someone(to) => to == own_side() || Some(to) == trading.with,
        _ => false,
    };
    if selling && holding.stock.is_none() {
        // Only whole things: part of a stack would be new.
        let part = holding.group.take().is_some() | holding.counted.take().is_some();
        if !part && trading.with.is_some() {
            send(TradeAsk::Sell { item: held.item.id });
            if let Some(gift) = over_gifts.0.filter(|gift| gift.to == own_side()) {
                gave.write(GaveItem(gift));
            }
        }
        return true;
    }
    let Some((with, item)) = holding.stock.clone() else {
        return false;
    };
    if with != own_side() && trading.unpaid(item.id).is_none() {
        return false;
    }
    holding.stock = None;
    held_stock.set_if_neq(HeldStock(None));
    let over = over_gifts.0.filter(|gift| gift.to == with);
    if with == own_side() {
        // Moved about on the own side, or taken off it.
        match over {
            Some(gift) => {
                gave.write(GaveItem(gift));
            }
            None => send(TradeAsk::Unsell { item: item.id }),
        }
        return true;
    }
    // Being bought: back on the shop's grid, or bought again where let go.
    if let Some(gift) = over {
        send(TradeAsk::Unbuy { item: item.id });
        gave.write(GaveItem(gift));
    } else if let Some((to, x, y, rotation)) = drop_move(carried, held.item, aim) {
        send(TradeAsk::Unbuy { item: item.id });
        let buy = Buy {
            item: item.id,
            to,
            x,
            y,
            rotation,
        };
        send(TradeAsk::Buy(buy));
    }
    true
}

#[cfg(test)]
mod tests;
