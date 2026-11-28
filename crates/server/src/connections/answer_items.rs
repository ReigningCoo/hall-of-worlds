//! Answering item moves: dropping, taking, wearing, tying bags to the belt, giving,
//! binning and moving things.

use bevy::prelude::*;
use shared::energy::handling_cost;
use shared::item::{Holder, Inventory, Item};
use shared::protocol::ClientMessage;

use super::{Action, Client, stack_parts};
use crate::ServerWorld;
use crate::ground::GroundAction;
use crate::ids::new_id;
use crate::sites::{self};

/// Answers a message about items. Whether the client is still welcome.
pub(super) fn answer_items(
    client: &mut Client,
    message: ClientMessage,
    world: &mut ServerWorld,
) -> bool {
    // Part of a stack: lifted out on its own, then dealt with as anything is.
    let message = match stack_parts::lift(client, message) {
        Ok(message) => message,
        Err(welcome) => return welcome,
    };
    // An id from inside a stack stands for it and all above it: lifted out on its
    // own, it moves like anything else, and goes back if it isn't moved.
    if let ClientMessage::MoveItem { item, .. }
    | ClientMessage::StackItem { item, .. }
    | ClientMessage::GiveItem { item, .. }
    | ClientMessage::WearItem { item }
    | ClientMessage::BinItem { item } = message
    {
        client.inventory.lift_part(item);
    }
    match message {
        ClientMessage::DropItem { item } => {
            let action = Action::Ground(GroundAction::Drop { item, part: None });
            client.waiting.push_back(action);
            true
        }
        ClientMessage::TakeFromGround {
            item,
            to,
            x,
            y,
            rotation,
        } => {
            client.waiting.push_back(Action::Ground(GroundAction::Take {
                item,
                to,
                x,
                y,
                rotation,
            }));
            true
        }
        ClientMessage::StackFromGround { item, onto } => {
            let action = Action::Ground(GroundAction::Stack { item, onto });
            client.waiting.push_back(action);
            true
        }
        ClientMessage::WearFromGround { item } => {
            let action = Action::Ground(GroundAction::Wear { item });
            client.waiting.push_back(action);
            true
        }
        ClientMessage::BinFromGround { item } => {
            let action = Action::Ground(GroundAction::Bin { item });
            client.waiting.push_back(action);
            true
        }
        ClientMessage::MoveItem {
            item,
            to,
            x,
            y,
            rotation,
        } => {
            let cost = client.inventory.move_cost(item, to);
            let tick = world.action_clock.ticks();
            let moving = |after: &mut Inventory| {
                let _ = after.move_item(item, to, x, y, rotation);
            };
            if client.may_take_off(item) && client.change_allowed(moving, tick) {
                match client.inventory.move_item(item, to, x, y, rotation) {
                    Ok(()) => {
                        client.unsaved = true;
                        client.energy.spend(tick, client.speed, cost);
                    }
                    Err(why) => warn!("Refused moving item {item:?} to {to:?}: {why:?}"),
                }
            }
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        ClientMessage::StackItem { item, onto } => {
            let to = client.inventory.placed(onto);
            let to = to.map_or(Holder::Inventory, |(holder, _)| holder);
            let cost = client.inventory.move_cost(item, to);
            let tick = world.action_clock.ticks();
            let stacks = world.stacks;
            let stacking = |after: &mut Inventory| {
                let _ = after.stack_onto(item, onto, stacks, &mut new_id);
            };
            if client.may_take_off(item) && client.change_allowed(stacking, tick) {
                match client
                    .inventory
                    .stack_onto(item, onto, world.stacks, &mut new_id)
                {
                    Ok(_) => {
                        client.unsaved = true;
                        client.energy.spend(tick, client.speed, cost);
                    }
                    Err(why) => warn!("Refused stacking {item:?} on {onto:?}: {why:?}"),
                }
            }
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        ClientMessage::SortGrid { holder } => {
            // Rearranging one grid takes no time.
            if client.inventory.sort_grid(holder, world.stacks) {
                client.unsaved = true;
            } else {
                debug!("Nothing sorted in {holder:?}");
            }
            client.send_inventory()
        }
        ClientMessage::GiveItem { item, to } => {
            let cost = client.inventory.put_down_cost(item);
            let quest_item = (client.inventory.find(item)).is_some_and(Item::holds_quest_item);
            let copper = (client.inventory.find(item)).map_or(0, crate::quests::copper_in);
            let tick = world.action_clock.ticks();
            let giving = |after: &mut Inventory| drop(after.hand_over(item));
            let given = if !client.may_take_off(item) {
                Err("it won't come off")
            } else if !client.change_allowed(giving, tick) {
                Err("its Strength is needed")
            } else if quest_item {
                // Quest items are the character's own (owner).
                Err("quest items can't be given")
            } else if sites::keeper_building(client, world, to).is_some() {
                // Shops are paid, through a trade (`trade.rs`).
                Err("shops take things only in a trade")
            } else {
                (crate::entities::give(client, world, item, to))
                    .map(|kind| crate::quests::gave(client, world, kind, copper))
            };
            match given {
                Ok(()) => {
                    client.unsaved = true;
                    let tick = world.action_clock.ticks();
                    client.energy.spend(tick, client.speed, cost);
                }
                Err(why) => warn!("Refused giving {item:?} to {to:?}: {why}"),
            }
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        ClientMessage::WearItem { item } => {
            let tick = world.action_clock.ticks();
            // What is in its way comes off, or the player is told why not.
            client.put_on(item, world.stacks, tick);
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        ClientMessage::TieBag { item, slot } => {
            let grams = client.inventory.find(item).map_or(0, Item::total_weight);
            match client.inventory.tie_bag(item, slot) {
                Ok(()) => {
                    client.unsaved = true;
                    let tick = world.action_clock.ticks();
                    client
                        .energy
                        .spend(tick, client.speed, handling_cost(grams));
                }
                Err(why) => warn!("Refused tying {item:?} to bag slot {slot}: {why:?}"),
            }
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        ClientMessage::BinItem { item } => {
            let cost = client.inventory.taking_off_cost(item);
            let tick = world.action_clock.ticks();
            let binning = |after: &mut Inventory| {
                after.to_bin(item);
            };
            if !client.may_take_off(item) || !client.change_allowed(binning, tick) {
                // Nothing more to say: it stays on.
            } else if client.inventory.to_bin(item) {
                client.unsaved = true;
                let tick = world.action_clock.ticks();
                client.energy.spend(tick, client.speed, cost);
            } else {
                warn!("Refused throwing away {item:?}");
            }
            client.update_speed(world.action_clock.ticks());
            client.send_inventory()
        }
        _ => unreachable!("`answer` only passes item messages here"),
    }
}
