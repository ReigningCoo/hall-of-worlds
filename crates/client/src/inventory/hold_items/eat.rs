//! Eating what is held by letting go of it on the clothes panel: the top one of
//! food (`bonus::is_food`), carried or from the ground, as the server will
//! (`ClientMessage::Consume`); the rest of its stack stays where it was. Also
//! one of a kind, asked for from the action bar (`EatKind`).

use bevy::prelude::*;
use shared::bonus::{is_food, may_eat};
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::{Inventory, Item};
use shared::protocol::ClientMessage;

use super::super::{Carried, EatKind, Holding};
use super::Outbox;
use crate::common::{SpentEnergy, TookItem};
use crate::connection::ToServer;

/// The carried stack `held` is, or is part of (`Holding::group` or `counted`),
/// if it is food: its id, as its top one is what is eaten.
pub(super) fn carried_food(carried: &Inventory, holding: &Holding, held: Id) -> Option<Id> {
    let group = holding.group.as_ref().map(|(stack, _)| *stack);
    let counted = holding.counted.map(|(stack, _)| stack);
    let stack = group.or(counted).unwrap_or(held);
    let item = carried.find(stack)?;
    (is_food(item.top().kind) && !carried.is_worn(stack)).then_some(stack)
}

/// Eats one of each kind asked for, from the stack `Inventory::stack_to_use`
/// chooses, unless none is carried or the player's kind won't eat it.
pub(in crate::inventory) fn eat_kind(
    mut asked: MessageReader<EatKind>,
    mut carried: ResMut<Carried>,
    mut holding: ResMut<Holding>,
    mut out: Outbox,
) {
    for &EatKind(kind) in asked.read() {
        if !may_eat(kind, carried.0.creature) {
            continue;
        }
        if let Some(stack) = carried.0.stack_to_use(kind) {
            eat_carried(stack, &mut carried.0, &mut || holding.stand_in(), &mut out);
        }
    }
}

/// Eats the top one of this carried stack, at once as the server will.
pub(super) fn eat_carried(
    stack: Id,
    carried: &mut Inventory,
    stand_in: &mut impl FnMut() -> Id,
    out: &mut Outbox,
) {
    out.to_server
        .write(ToServer(ClientMessage::Consume { item: stack }));
    if let Some(eaten) = carried.take_one(stack, stand_in) {
        out.spent
            .write(SpentEnergy(handling_cost(eaten.total_weight())));
    }
}

/// Eats the top one of this, lying in a bag in reach (`with` is the ground):
/// the rest stays there, as the server will say.
pub(super) fn eat_from_ground(id: Id, mut item: Item, with: Id, out: &mut Outbox) {
    out.to_server
        .write(ToServer(ClientMessage::Consume { item: id }));
    // Its id doesn't matter: it is gone at once.
    let top = item.take_top(&mut || Id::stand_in(0));
    if top.is_none() {
        out.took.write(TookItem { with, item: id });
    }
    let eaten = top.unwrap_or(item);
    out.spent
        .write(SpentEnergy(handling_cost(eaten.total_weight())));
}
