//! Eating what is held by letting go of it on the clothes panel: the top one of
//! food or a herb (`bonus::edible`), or a potion, drunk, carried or from the
//! ground, as the server will (`ClientMessage::Consume`); the rest of its stack
//! stays where it was. Also one of a kind, or a bottle of a potion, asked for
//! from the action bar (`EatKind`, `DrinkPotion`). Water drunk from a vessel
//! leaves the vessel where it is, a serving emptier (`vessel::VesselForm::drunk`).

use bevy::prelude::*;
use shared::bonus::{may_eat, usable_on_self};
use shared::energy::handling_cost;
use shared::id::Id;
use shared::item::{Inventory, Item};
use shared::protocol::ClientMessage;

use super::super::{Carried, DrinkPotion, EatKind, Holding};
use super::Outbox;
use crate::common::{SpentEnergy, TookItem};
use crate::connection::ToServer;

/// The carried stack `held` is, or is part of (`Holding::group` or `counted`),
/// if it is food, a herb or a potion: its id, as its top one is what is eaten.
pub(super) fn carried_food(carried: &Inventory, holding: &Holding, held: Id) -> Option<Id> {
    let group = holding.group.as_ref().map(|(stack, _)| *stack);
    let counted = holding.counted.map(|(stack, _)| stack);
    let stack = group.or(counted).unwrap_or(held);
    let item = carried.find(stack)?;
    (usable_on_self(item.top()) && !carried.is_worn(stack)).then_some(stack)
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

/// Drinks a bottle of each potion asked for, from the stack
/// `Inventory::potion_to_use` chooses, unless none is carried. The server
/// gives the bottle back empty.
pub(in crate::inventory) fn drink_potion(
    mut asked: MessageReader<DrinkPotion>,
    mut carried: ResMut<Carried>,
    mut holding: ResMut<Holding>,
    mut out: Outbox,
) {
    for &DrinkPotion(potion) in asked.read() {
        if let Some(stack) = carried.0.potion_to_use(potion) {
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
    if let Some(vessel) = carried.find(stack).filter(|it| it.count() == 1)
        && let Some((after, ml)) = drunk_from(vessel)
    {
        carried.replace_carried(after);
        out.spent.write(SpentEnergy(handling_cost(ml)));
        return;
    }
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
    if let Some((_, ml)) = drunk_from(&item) {
        // The vessel stays on the ground; the server says what is left in it.
        out.spent.write(SpentEnergy(handling_cost(ml)));
        return;
    }
    // Its id doesn't matter: it is gone at once.
    let top = item.take_top(&mut || Id::stand_in(0));
    if top.is_none() {
        out.took.write(TookItem { with, item: id });
    }
    let eaten = top.unwrap_or(item);
    out.spent
        .write(SpentEnergy(handling_cost(eaten.total_weight())));
}

/// A vessel after one drink from it, and how many millilitres were drunk:
/// `None` if it isn't a vessel.
fn drunk_from(item: &Item) -> Option<(Item, u32)> {
    let (after, ml) = item.top().vessel()?.clone().drunk();
    let after = Item {
        form: Some(shared::item_form::ItemForm::Vessel(after)),
        ..item.top().clone()
    };
    Some((after, ml))
}
