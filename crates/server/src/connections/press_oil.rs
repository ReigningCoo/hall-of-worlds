//! A player pressing oil (`Ability::PressOil`, brewing plan PR 5): the client
//! names a stack carried (`UseAbility`'s `target`), and every bottle is a
//! wind-up of `potion::press_ticks`; when one lands, a bottle's worth of what
//! is pressed (`potion::grams_per_bottle`) is used up, an empty bottle is
//! filled with its oil and Alchemy is practised, then the next begins, until
//! there is too little left, no empty bottle, or the player moves or is harmed
//! (`stop_collecting`, as for Collect Water). `can_use` has already checked
//! the oil press is carried.

use bevy::prelude::*;
use shared::ability::{ALCHEMY, Ability};
use shared::id::Id;
use shared::potion::{PRESS_PRACTICE, Pressing, grams_per_bottle, press_ticks, pressed};
use shared::protocol::{Landed, ServerMessage};
use shared::skill::SkillId;

use super::Client;
use crate::ServerWorld;
use crate::combat::{Act, Pending};
use crate::ids::new_id;

/// Press Oil asked for on `stack`: what it is becomes what is pressed, and the
/// first bottle begins if there is enough of it and an empty bottle.
pub(super) fn choose_press(
    client: &mut Client,
    world: &mut ServerWorld,
    stack: Option<Id>,
) -> (bool, u32) {
    let item = stack.and_then(|stack| client.inventory.find(stack));
    client.pressing = item.and_then(|item| Pressing::of(item, &world.herbs));
    start_press(client, world)
}

/// Begins pressing the next bottle, if it can be: whether the client is still
/// there, and what it cost.
pub(super) fn start_press(client: &mut Client, world: &mut ServerWorld) -> (bool, u32) {
    let can = match client.pressing.as_ref() {
        Some(Pressing::Moromi) => client.inventory.moromi_to_press().is_some(),
        Some(pressing) => (client.inventory.can_press(pressing, &world.herbs)).is_some(),
        None => false,
    };
    if !can {
        debug!("Not pressing oil: nothing chosen, too little, or no empty bottle");
        client.pressing = None;
        return (true, 0);
    }
    let tick = world.action_clock.ticks();
    let lands_in = press_ticks(&client.sheet.skills);
    client.fight.pending = Some(Pending {
        target: client.entity_id(),
        act: Act::PressOil,
        lands_at: tick + lands_in,
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::PressOil,
        target: None,
        lands_in: lands_in as u32,
        ready_in: lands_in as u32,
    });
    // As long as the wind-up, so the player acts again once it lands.
    let cost = (lands_in * u64::from(client.speed)).min(u64::from(u32::MAX)) as u32;
    (stayed, cost)
}

/// A bottle's wind-up is over: a bottle's worth is used up and an empty
/// bottle filled with its oil (if both are still carried), Alchemy practised,
/// then the next begins. `false` if the client has gone.
pub(super) fn land_press(client: &mut Client, world: &mut ServerWorld) -> bool {
    let tick = world.action_clock.ticks();
    let Some(pressing) = client.pressing.clone() else {
        return true;
    };
    if pressing == Pressing::Moromi {
        return super::press_moromi::land_moromi(client, world);
    }
    let can = client.inventory.can_press(&pressing, &world.herbs);
    let bottle = client.inventory.empty_bottle();
    let (Some(oil), Some(bottle)) = (can, bottle) else {
        client.pressing = None;
        return client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Fizzled,
        });
    };
    use_up(client, &pressing, grams_per_bottle(oil));
    let Some(bottle) = client.inventory.take_one(bottle, &mut new_id) else {
        return true;
    };
    if let Err(item) = client
        .inventory
        .give(pressed(bottle, oil), world.stacks, &mut new_id)
    {
        crate::ground::put_down(client, world, item);
        client.ground_told.again = true;
    }
    practise_pressing(client, tick, 1);
    client.unsaved = true;
    client.update_speed(tick);
    let told = client.send_inventory()
        && client.send_sheet(tick)
        && client.link.send(&ServerMessage::AbilityLanded {
            who: None,
            target: None,
            landed: Landed::Pressed(oil),
        });
    if !told {
        return false;
    }
    // The next bottle, at once.
    let (stayed, cost) = start_press(client, world);
    client.energy.spend(tick, client.speed, cost);
    stayed
}

/// Alchemy practised for `bottles` pressed.
pub(super) fn practise_pressing(client: &mut Client, tick: u64, bottles: u32) {
    if let Some(skill) = SkillId::find(ALCHEMY) {
        let amount = (client.learning.skills).take(skill, PRESS_PRACTICE * bottles, tick);
        client.sheet.skills.practise(skill, amount);
        client.sheet.skills.used(skill, client.activity.played);
    }
}

/// Takes `pressing` one at a time, smallest stacks first, until at least
/// `grams` have gone (the last one may give more than is needed).
fn use_up(client: &mut Client, pressing: &Pressing, grams: u32) {
    let mut taken = 0;
    while taken < grams {
        let Some(stack) = client.inventory.stack_to_press(pressing) else {
            return;
        };
        let Some(one) = client.inventory.take_one(stack, &mut new_id) else {
            return;
        };
        taken += one.own_weight().max(1);
    }
}
