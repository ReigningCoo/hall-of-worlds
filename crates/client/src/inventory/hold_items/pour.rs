//! Letting go of a cask, vessel or bottle on another (`Target::Pour`): poured
//! into it (`ClientMessage::Pour`) by `shared::pour`'s rule, carried or from
//! the ground, or the red line under the clothes saying why not. What is held
//! goes back where it was; a carried container alone shows the pour at once,
//! the rest waits for the server's `Inventory`.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::pour::{pour, pour_cost};
use shared::protocol::ClientMessage;

use super::super::hold::Target;
use super::super::{Carried, HeldBy, Holding};
use super::Outbox;
use crate::common::{GROUND, HeldStock, SpentEnergy};
use crate::connection::ToServer;

/// Pours what is held, if it is aimed at something to pour into: whether it
/// was (nothing else is then done with it).
pub(super) fn pour_on(
    held: HeldBy,
    aim: Target,
    holding: &mut ResMut<Holding>,
    carried: &mut ResMut<Carried>,
    held_stock: &mut ResMut<HeldStock>,
    out: &mut Outbox,
) -> bool {
    let Target::Pour { into, .. } = aim else {
        return false;
    };
    // A whole stack is poured from by its top one, as the server will.
    let stack = holding.group.take().map(|(stack, _)| stack);
    let stack = stack.or(holding.counted.take().map(|(stack, _)| stack));
    let (from, source) = match holding.stock.take() {
        Some((with, item)) if with == GROUND => (held.item.id, Some(item)),
        Some(_) => return false,
        None => {
            let from = stack.unwrap_or(held.item.id);
            (from, carried.0.find(from).cloned())
        }
    };
    held_stock.set_if_neq(HeldStock(None));
    let target = carried.0.find(into).cloned();
    let (Some(source), Some(target)) = (source, target) else {
        return true;
    };
    let tap = carried.0.count_of(ItemKind::Tap) > 0;
    match pour(source.top(), target.top(), tap) {
        Err(why) => {
            let (note, _, time, _) = &mut out.wear;
            note.say_words(why.words().to_owned(), time.elapsed_secs_f64());
        }
        Ok(done) => {
            out.to_server
                .write(ToServer(ClientMessage::Pour { from, into }));
            out.spent.write(SpentEnergy(pour_cost(done.ml)));
            show_now(&mut carried.0, from, &source, done.from);
            show_now(&mut carried.0, into, &target, done.into);
        }
    }
    true
}

/// Shows a carried container alone as it is after pouring; a stack, or one on
/// the ground, waits for the server.
fn show_now(carried: &mut shared::item::Inventory, id: Id, before: &Item, after: Item) {
    if before.count() == 1 && carried.find(id).is_some() {
        carried.replace_carried(after);
    }
}
