//! A player splitting a log into kindling (`Ability::CutKindling`,
//! woodcutting plan PR 2): a one-handed axe held, and a log to split
//! (`woodcutting::log_to_cut`): the first carried, else the one on the ground
//! the client names in `UseAbility`'s `target` (which it does only while the
//! inventory is open with the ground's storage shown). A wind-up of
//! `cut_ticks`; when it lands, the log becomes a piece of kindling a kilogram
//! (`kindling_from`) of its wood, into the inventory (the rest on the ground),
//! and Woodcutting is practised. Moving, being harmed or Esc stop it
//! (`stop_collecting`).

use bevy::prelude::*;
use shared::ability::Ability;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;
use shared::woodcutting::{
    KINDLING_PRACTICE, LogAt, WOODCUTTING_SKILL, cut_ticks, kindling_from, log_to_cut, new_kindling,
};

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// The log the player would split now: carried, else `ground` if it names a
/// log lying in their reach.
fn log_here(client: &Client, world: &mut ServerWorld, ground: Option<Id>) -> Option<LogAt> {
    let lying = ground.and_then(|id| crate::ground::lying_stack(client, world, id).ok());
    let lying: Vec<Item> = lying.into_iter().collect();
    log_to_cut(&client.inventory, Some(&lying))
}

/// Begins splitting a log, if there is one: whether the client is still
/// there, and what it cost.
pub(super) fn start_cut(
    client: &mut Client,
    world: &mut ServerWorld,
    ground: Option<Id>,
) -> (bool, u32) {
    let Some(log) = log_here(client, world, ground) else {
        debug!("Not starting Cut Kindling: no log carried or named on the ground");
        return (true, 0);
    };
    let lands_in = cut_ticks(&client.sheet.skills);
    let tick = world.action_clock.ticks();
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::CutKindling(log),
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::CutKindling,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// The wind-up is over: the log, if still there and an axe still held, is
/// split. `false` if the client has gone.
pub(super) fn land_cut(client: &mut Client, world: &mut ServerWorld, log: LogAt) -> bool {
    let taken = match log {
        _ if client.inventory.held_axe().is_none() => None,
        LogAt::Carried(id) => client.inventory.take_one(id, &mut new_id),
        LogAt::Ground(id) => crate::ground::take_lying(client, world, id).ok(),
    };
    let form = taken.as_ref().and_then(|item| match &item.form {
        Some(ItemForm::Log(form)) if item.kind == ItemKind::Log => Some(*form),
        _ => None,
    });
    let Some(form) = form else {
        if let Some(item) = taken {
            // Not a log after all: put back where it can be found.
            give_or_drop(client, world, item);
        }
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    if matches!(log, LogAt::Ground(_)) {
        client.ground_told.again = true;
    }
    let count = kindling_from(&form);
    let made = Some(world.made_now());
    give_or_drop(
        client,
        world,
        new_kindling(form.wood, count, new_id(), made),
    );
    let tick = world.action_clock.ticks();
    if let Some(skill) = SkillId::find(WOODCUTTING_SKILL) {
        let amount = client.learning.skills.take(skill, KINDLING_PRACTICE, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
    client.unsaved = true;
    client.update_speed(tick);
    client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Split(count),
        })
}

/// Into the inventory, or onto the ground under the player if it won't fit.
fn give_or_drop(client: &mut Client, world: &mut ServerWorld, item: Item) {
    if let Err(item) = client.inventory.give(item, world.stacks, &mut new_id) {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
}
