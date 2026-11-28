//! Holding and letting go in a trade (trading plan PR 4, `common::Trading`):
//! what is carried let go on the trade window's own side (Selling) is sold (it
//! stays carried, marked), and taken off it anywhere else is taken back;
//! anywhere else in the window it goes back where it was (trading tweaks PR 3). What is
//! bought lies in the window's Buying box (`common::buying_box`), never in
//! the inventory: stock let go there is bought (`trade` sends where), and
//! what is in it let go on the shop's grid is put back. With a player or an
//! NPC (PR 6), what is let go on the own side is offered where it lands
//! (`trade` sends that); what is in the receive box is taken out where let
//! go.

mod quick;

use bevy::prelude::*;
use shared::item::Inventory;
use shared::protocol::{ClientMessage, TradeAsk};
use shared::trade::Place;

use super::super::hold::{Target, drop_move};
use super::super::{HeldBy, Holding};
use super::Outbox;
use crate::common::{
    GaveItem, HeldOverGifts, HeldStock, Trading, buying_box, own_side, receive_box,
};
use crate::connection::ToServer;
pub(super) use quick::quick_in_trade;

/// Whether `item`, held from `from`'s grid, may be let go on `to`'s: back
/// where it came from, or between a shop's stock and the Buying box (never
/// into the box again while it is being bought).
pub(super) fn may_go_to(
    trading: &Trading,
    item: shared::id::Id,
    (from, to): (shared::id::Id, shared::id::Id),
) -> bool {
    let shop = trading.with.filter(|_| trading.between.is_none());
    let new = trading.bought(item).is_none();
    let swap = shop.is_some_and(|shop| {
        (from == shop && to == buying_box() && new) || (from == buying_box() && to == shop)
    });
    from == to || swap
}

/// Whether something held is a shop's, from its stock or the Buying box:
/// it goes nowhere in the inventory.
pub(super) fn is_shops(trading: &Trading, from: shared::id::Id) -> bool {
    from == buying_box() || (trading.between.is_none() && Some(from) == trading.with)
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
    // Let go on the own side (Selling): sold, or offered.
    let selling = aim == Target::Someone(own_side());
    if selling && holding.stock.is_none() {
        // Only whole things: part of a stack would be new.
        let part = holding.group.take().is_some() | holding.counted.take().is_some();
        if !part && trading.with.is_some() {
            // To a character, `trade` offers it where it lands.
            if trading.between.is_none() {
                send(TradeAsk::Sell { item: held.item.id });
            }
            if let Some(gift) = over_gifts.0.filter(|gift| gift.to == own_side()) {
                gave.write(GaveItem(gift));
            }
        }
        return true;
    }
    // Nothing carried goes into the receive box, the Buying box or the
    // shop's stock: back where it was.
    let shops = matches!(aim, Target::Someone(to) if is_shops(trading, to));
    if (shops || aim == Target::Someone(receive_box())) && holding.stock.is_none() {
        return true;
    }
    let Some((with, item)) = holding.stock.clone() else {
        return false;
    };
    if with == receive_box() {
        // Taken out of the receive box to where it was let go, if anywhere.
        holding.stock = None;
        held_stock.set_if_neq(HeldStock(None));
        if let Some((to, x, y, rotation)) = drop_move(carried, held.item, aim) {
            let to = Place {
                item: item.id,
                to,
                x,
                y,
                rotation,
            };
            send(TradeAsk::Take(to));
        }
        return true;
    }
    if is_shops(trading, with) && with != buying_box() {
        // The shop's stock goes only into the Buying box (`trade` buys it
        // there), or back.
        holding.stock = None;
        held_stock.set_if_neq(HeldStock(None));
        if let Some(gift) = over_gifts.0 {
            gave.write(GaveItem(gift));
        }
        return true;
    }
    if with != own_side() && with != buying_box() {
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
    // In the Buying box: moved about in it (`trade` sends where), or put back
    // on the shop's grid.
    let gift = over_gifts.0;
    if let Some(gift) = gift.filter(|gift| Some(gift.to) == trading.with) {
        send(TradeAsk::Unbuy { item: item.id });
        gave.write(GaveItem(gift));
    } else if let Some(gift) = gift.filter(|gift| gift.to == buying_box()) {
        gave.write(GaveItem(gift));
    }
    true
}

#[cfg(test)]
mod tests;
