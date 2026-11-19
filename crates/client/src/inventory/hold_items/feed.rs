//! Letting go of flour on yeast or on a bottle of fresh water
//! (`Target::Feed`): the yeast fed or a starter begun (`ClientMessage::Yeast`)
//! by `shared::yeast`'s rules, or the red line under the clothes saying why
//! not. The flour goes back where it was, and the rest waits for the server's
//! `Inventory` (a step up is its roll).

use bevy::prelude::*;
use shared::protocol::ClientMessage;

use super::super::hold::{Target, feeding};
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use crate::common::HeldStock;
use crate::connection::ToServer;

/// Feeds or begins a starter with the flour held, if it is aimed at yeast or a
/// bottle: whether it was (nothing else is then done with it).
pub(super) fn feed_on(
    held: HeldBy,
    aim: Target,
    holding: &mut ResMut<Holding>,
    carried: &ResMut<Carried>,
    held_stock: &mut ResMut<HeldStock>,
    out: &mut Outbox,
) -> bool {
    let Target::Feed { onto, .. } = aim else {
        return false;
    };
    if holding.stock.is_some() {
        // Flour from the ground: only carried flour feeds.
        return false;
    }
    // A part of a stack stands for all of it, as for pouring.
    let stack = holding.group.take().map(|(stack, _)| stack);
    let stack = stack.or(holding.counted.take().map(|(stack, _)| stack));
    held_stock.set_if_neq(HeldStock(None));
    let flour = stack.unwrap_or(held.item.id);
    let (Some(item), Some(under)) = (carried.0.find(flour), carried.0.find(onto)) else {
        return true;
    };
    match feeding(&carried.0, item, under) {
        Some(Ok(ask)) => {
            out.to_server.write(ToServer(ClientMessage::Yeast(ask)));
        }
        Some(Err(why)) => {
            let (note, _, time, _) = &mut out.wear;
            note.say_words(why.to_owned(), time.elapsed_secs_f64());
        }
        None => {}
    }
    true
}
